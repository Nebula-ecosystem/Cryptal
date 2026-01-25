use super::block::Block;
use super::params::Argon2Params;
use super::reference::compute_reference_position;

#[derive(Debug, Clone)]
pub(crate) struct MemoryLayout {
    pub lanes: u32,
    pub lane_len: u32,
    pub segment_len: u32,
    pub total_blocks: u32,
}

impl MemoryLayout {
    pub(crate) fn new(params: &Argon2Params) -> Self {
        let sync_points = 4;
        let lanes = params.lanes;

        let total_blocks = (params.mem_kib / (sync_points * lanes)) * (sync_points * lanes);

        let lane_len = total_blocks / lanes;
        let segment_len = lane_len / sync_points;

        Self {
            lanes,
            lane_len,
            segment_len,
            total_blocks,
        }
    }

    #[inline]
    pub(crate) fn index(&self, lane: u32, index_in_lane: u32) -> usize {
        (lane * self.lane_len + index_in_lane) as usize
    }

    pub(crate) fn fill(&self, memory: &mut [Block], time: u32) {
        let segment_len = self.segment_len;
        let lane_len = self.lane_len;
        let lanes = self.lanes;
        let total_blocks = self.total_blocks;

        for pass in 0..time {
            for slice in 0..4 {
                for lane in 0..lanes {
                    let data_independent = pass == 0 && slice < 2;

                    let mut addr_block = Block::ZERO;
                    let mut address_counter: u32 = 1;

                    let start_idx = if pass == 0 && slice == 0 { 2 } else { 0 };

                    for i in start_idx..segment_len {
                        let index_in_lane = slice * segment_len + i;
                        let prev_idx = if index_in_lane == 0 {
                            lane_len - 1
                        } else {
                            index_in_lane - 1
                        };

                        let (j1, j2) = if data_independent {
                            if (i % 128) == 0 {
                                addr_block = Block::generate_address(
                                    pass,
                                    lane,
                                    slice,
                                    total_blocks,
                                    time,
                                    address_counter,
                                );

                                address_counter += 1;
                            }

                            let pair_idx = (i % 128) as usize;
                            let s = addr_block.as_u32_slice();
                            (s[2 * pair_idx], s[2 * pair_idx + 1])
                        } else {
                            let prev_block = &memory[self.index(lane, prev_idx)];
                            let s = prev_block.as_u32_slice();
                            (s[0], s[1])
                        };

                        let (ref_lane, ref_idx) = compute_reference_position(
                            pass,
                            slice,
                            lane,
                            index_in_lane,
                            self,
                            j1,
                            j2,
                        );

                        let cur = self.index(lane, index_in_lane);
                        let prev = self.index(lane, prev_idx);
                        let reference = self.index(ref_lane, ref_idx);

                        let compressed = Block::compress(&memory[prev], &memory[reference]);

                        if pass == 0 {
                            memory[cur] = compressed;
                        } else {
                            memory[cur].in_place_xor(&compressed);
                        }
                    }
                }
            }
        }
    }
}
