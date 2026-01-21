use cryptal::hash::sha512;

use crate::{
    ge::{GeP3, ge_p3_tobytes, ge_scalarmult_base},
    sc::{sc_muladd, sc_reduce},
};

pub fn ed25519_sign(
    signature: &mut [u8; 64],
    message: &[u8],
    public_key: &[u8; 32],
    private_key: &[u8; 64],
) {
    let mut hram = [0u8; 64];
    let mut r = [0u8; 64];
    let mut r_point = GeP3::default();

    let mut buf = Vec::with_capacity(32 + message.len());
    buf.extend_from_slice(&private_key[32..64]);
    buf.extend_from_slice(message);

    let digest = sha512(&buf);
    r.copy_from_slice(digest.as_ref());
    sc_reduce(&mut r);

    ge_scalarmult_base(&mut r_point, &r[32..].try_into().unwrap());
    ge_p3_tobytes((&mut signature[..32]).try_into().unwrap(), &r_point);

    let mut buf = Vec::with_capacity(32 + 32 + message.len());
    buf.extend_from_slice(&signature[..32]);
    buf.extend_from_slice(public_key);
    buf.extend_from_slice(message);

    let digest = sha512(&buf);
    hram.copy_from_slice(digest.as_ref());
    sc_reduce(&mut hram);

    let sig_s: &mut [u8; 32] = (&mut signature[32..64]).try_into().unwrap();
    let sk: &[u8; 32] = (&private_key[..32]).try_into().unwrap();

    sc_muladd(sig_s, &hram, sk, &r);
}
