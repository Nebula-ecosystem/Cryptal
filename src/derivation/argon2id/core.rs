use super::blake2b::blake2b_512;
use super::block::Block;
use super::boundary::{finalize, init};
use super::memory::MemoryLayout;
use super::params::{Argon2ParamError, Argon2Params};

#[derive(Debug)]
pub enum Argon2Error {
    InvalidParams(Argon2ParamError),
    InvalidSalt,
}
pub fn argon2id(
    password: &[u8],
    salt: &[u8],
    params: &Argon2Params,
) -> Result<Vec<u8>, Argon2Error> {
    params.validate().map_err(Argon2Error::InvalidParams)?;

    if salt.len() < 8 {
        return Err(Argon2Error::InvalidSalt);
    }

    let lanes = params.lanes;
    let sync_points = 4u32;

    let m_min = 8u32.saturating_mul(lanes);
    let mut m_prime = params.mem_kib.max(m_min);
    m_prime = (m_prime / (sync_points * lanes)) * (sync_points * lanes);

    let mut params2 = params.clone();
    params2.mem_kib = m_prime;

    let layout = MemoryLayout::new(&params2);
    let mut memory = vec![Block::ZERO; layout.total_blocks as usize];

    let h0 = init(password, salt, params, m_prime).to_vec();

    for i in 0..lanes {
        let mut input = h0.clone();
        input.extend_from_slice(&i.to_le_bytes());
        let mut padded_hash = [0u8; 1024];
        let hash = blake2b_512(&input);
        padded_hash[..64].copy_from_slice(&hash);
        memory[layout.index(i, 0)] = Block::from_bytes(padded_hash);
    }

    layout.fill(&mut memory, params2.time);

    let tag = finalize(&memory, params2.tag_len);

    Ok(tag)
}
