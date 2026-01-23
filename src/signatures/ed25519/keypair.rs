use super::group::GeP3;
use crate::{hash::sha512, signatures::ed25519::scalar::Scalar};

pub fn ed25519_create_keypair(
    public_key: &mut [u8; 32],
    private_key: &mut [u8; 64],
    seed: &[u8; 32],
) {
    let digest = sha512(seed);

    private_key.copy_from_slice(digest.as_ref());
    private_key[0] &= 248;
    private_key[31] &= 63;
    private_key[31] |= 64;

    *public_key = GeP3::from_scalar_mul(Scalar(private_key[..32].try_into().unwrap())).to_bytes()
}
