use super::memory::MemoryLayout;

pub(crate) fn compute_reference_position(
    pass: u32,
    slice: u32,
    lane: u32,
    index: u32,
    layout: &MemoryLayout,
    j1: u32,
    j2: u32,
) -> (u32, u32) {
    let segment_len = layout.lane_len / 4;
    let index_in_segment = index % segment_len;

    let ref_lane = if pass == 0 && slice == 0 {
        lane
    } else {
        j2 % layout.lanes
    };

    let (start_pos, w_size) = if pass == 0 {
        if slice == 0 {
            (0, index.saturating_sub(1))
        } else if ref_lane == lane {
            (0, slice * segment_len + index_in_segment.saturating_sub(1))
        } else {
            let size = slice * segment_len;
            if index_in_segment == 0 && size > 0 {
                (0, size - 1)
            } else {
                (0, size)
            }
        }
    } else {
        let start = ((slice + 1) % 4) * segment_len;

        if ref_lane == lane {
            let size = layout.lane_len - segment_len + index_in_segment.saturating_sub(1);
            (start, size)
        } else {
            let size = layout.lane_len - segment_len;
            if index_in_segment == 0 && size > 0 {
                (start, size - 1)
            } else {
                (start, size)
            }
        }
    };

    if w_size == 0 {
        return (ref_lane, start_pos % layout.lane_len);
    }

    let w = w_size as u64;
    let x = ((j1 as u64) * (j1 as u64)) >> 32;
    let y = (w * x) >> 32;
    let zz = (w - 1).wrapping_sub(y) as u32;

    let ref_index = (start_pos + zz) % layout.lane_len;
    (ref_lane, ref_index)
}
