use super::H256_INIT;
use super::computations::{all_rounds, expand_w};
use crate::primitives::U256;

use core::ptr::read_unaligned;

#[inline(always)]
pub fn compress(block: &[u8; 64], state: &mut [u32; 8]) {
    let mut w = [0u32; 64];

    let bp = block.as_ptr();
    let wp = w.as_mut_ptr();
    for i in 0..16 {
        unsafe {
            let p = bp.add(i * 4) as *const u32;
            let v = u32::from_be(read_unaligned(p));
            wp.add(i).write(v);
        }
    }

    expand_w(&mut w);

    let regs = all_rounds(*state, &w);

    let sp = state.as_mut_ptr();
    let rp = regs.as_ptr();
    for i in 0..8 {
        unsafe {
            let s = sp.add(i).read();
            let r = rp.add(i).read();
            sp.add(i).write(s.wrapping_add(r));
        }
    }
}

pub fn sha256(input: &[u8]) -> U256 {
    let mut state = H256_INIT;

    let mut i = 0;
    let len = input.len();

    while i + 64 <= len {
        let block = input[i..i + 64].try_into().unwrap();
        compress(block, &mut state);
        i += 64;
    }

    let mut block = [0u8; 64];
    let rem = len - i;

    block[..rem].copy_from_slice(&input[i..]);
    block[rem] = 0x80;

    if rem > 55 {
        compress(&block, &mut state);
        block = [0; 64];
    }

    let bit_len = (len as u64) << 3;
    block[56..].copy_from_slice(&bit_len.to_be_bytes());

    compress(&block, &mut state);

    U256::from(state)
}
