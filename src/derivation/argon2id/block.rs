use super::blake2b::{blake2b_512, blake2b_long};

#[derive(Debug, Clone)]
pub struct Block(pub [u64; 128]);

impl Block {
    pub(crate) const ZERO: Self = Self([0u64; 128]);

    pub(crate) fn in_place_xor(&mut self, other: &Block) {
        self.0
            .iter_mut()
            .zip(other.0.iter())
            .for_each(|(a, b)| *a ^= b);
    }

    pub(crate) fn from_bytes(bytes: [u8; 1024]) -> Self {
        let words = core::array::from_fn(|i| {
            let start = i * 8;
            u64::from_le_bytes(bytes[start..start + 8].try_into().unwrap())
        });
        Block(words)
    }

    pub(crate) fn to_bytes(&self) -> [u8; 1024] {
        let mut out = [0u8; 1024];

        self.0.iter().enumerate().for_each(|(i, word)| {
            let start = i * 8;
            out[start..start + 8].copy_from_slice(&word.to_le_bytes());
        });

        out
    }

    pub(crate) fn as_u32_slice(&self) -> &[u32; 256] {
        unsafe { &*(self.0.as_ptr() as *const [u32; 256]) }
    }

    pub(crate) fn compress(x: &Self, y: &Self) -> Self {
        let mut input = Vec::with_capacity(2048);
        input.extend_from_slice(x.to_bytes().as_slice());
        input.extend_from_slice(y.to_bytes().as_slice());

        let hash = blake2b_long(1024, &input);

        Block::from_bytes(hash.as_slice().try_into().unwrap())
    }

    pub(crate) fn generate_address(
        pass: u32,
        lane: u32,
        slice: u32,
        total_blocks: u32,
        time: u32,
        counter: u32,
    ) -> Self {
        let mut input = [0u8; 72];
        input[0..4].copy_from_slice(&pass.to_le_bytes());
        input[4..8].copy_from_slice(&lane.to_le_bytes());
        input[8..12].copy_from_slice(&slice.to_le_bytes());
        input[12..16].copy_from_slice(&total_blocks.to_le_bytes());
        input[16..20].copy_from_slice(&time.to_le_bytes());
        input[20..24].copy_from_slice(&2u32.to_le_bytes());
        input[24..28].copy_from_slice(&0x13u32.to_le_bytes());
        input[28..32].copy_from_slice(&counter.to_le_bytes());

        let hash = blake2b_512(&input);
        let mut block_input = [0u8; 1024];
        block_input[..64].copy_from_slice(&hash);

        Self::from_bytes(block_input)
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        self.0.iter_mut().for_each(|v| *v = 0);
    }
}
