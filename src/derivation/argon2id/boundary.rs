use super::blake2b::blake2b_512;
use super::block::Block;
use super::params::Argon2Params;

pub(crate) fn init(
    password: &[u8],
    salt: &[u8],
    params: &Argon2Params,
    mem_kib_rounded: u32,
) -> [u8; 64] {
    let mut buf = Vec::new();

    buf.extend_from_slice(&(password.len() as u32).to_le_bytes());
    buf.extend_from_slice(password);

    buf.extend_from_slice(&(salt.len() as u32).to_le_bytes());
    buf.extend_from_slice(salt);

    buf.extend_from_slice(&(params.tag_len as u32).to_le_bytes());
    buf.extend_from_slice(&mem_kib_rounded.to_le_bytes());
    buf.extend_from_slice(&params.time.to_le_bytes());
    buf.extend_from_slice(&0x13u32.to_le_bytes());
    buf.extend_from_slice(&2u32.to_le_bytes());

    buf.extend_from_slice(&(params.tag_len as u32).to_le_bytes());
    buf.extend_from_slice(&mem_kib_rounded.to_le_bytes());
    buf.extend_from_slice(&params.time.to_le_bytes());
    buf.extend_from_slice(&0x13u32.to_le_bytes());
    buf.extend_from_slice(&2u32.to_le_bytes());

    blake2b_512(&buf)
}

pub(crate) fn finalize(memory: &[Block], tag_len: usize) -> Vec<u8> {
    let mut final_block = Block::ZERO;
    for block in memory {
        final_block.in_place_xor(block);
    }

    let bytes = final_block.to_bytes();
    let final_hash = blake2b_512(&bytes);

    final_hash[..tag_len].to_vec()
}
