use cryptal::hash::sha512;

use crate::{
    fe::fe_neg,
    ge::{
        GeCached, GeP1P1, GeP3, ge_add, ge_frombytes_negate_vartime, ge_p1p1_to_p3,
        ge_p3_to_cached, ge_p3_tobytes, ge_scalarmult_base,
    },
    sc::sc_muladd,
};

pub fn ed25519_add_ascalar(
    public_key: Option<&mut [u8; 32]>,
    private_key: Option<&mut [u8; 64]>,
    scalar: &[u8; 32],
) {
    let mut sc_1 = [0u8; 32];
    sc_1[0] = 1;

    let mut n = [0u8; 32];

    let mut nb = GeP3::default();
    let mut a_p1p1 = GeP1P1::default();
    let mut a = GeP3::default();
    let mut public_key_unpacked = GeP3::default();
    let mut t = GeCached::default();

    n[..31].copy_from_slice(&scalar[..31]);
    n[31] = scalar[31] & 127;

    match (private_key, public_key) {
        (Some(private), Some(public)) => {
            let mut sk0 = [0u8; 32];
            sk0.copy_from_slice(&private[..32]);

            sc_muladd(&mut private[..32], &sc_1, &n, &sk0);

            let mut buf = [0u8; 64];
            buf[..32].copy_from_slice(&private[32..64]);
            buf[32..].copy_from_slice(scalar);

            let hashbuf = sha512(&buf);
            private[32..64].copy_from_slice(&hashbuf.as_ref()[..32]);

            ge_scalarmult_base(&mut a, (&private[..32]).try_into().unwrap());
            ge_p3_tobytes(public, &a);
        }

        (Some(private), None) => {
            let mut sk0 = [0u8; 32];
            sk0.copy_from_slice(&private[..32]);

            sc_muladd(&mut private[..32], &sc_1, &n, &sk0);

            let mut buf = [0u8; 64];
            buf[..32].copy_from_slice(&private[32..64]);
            buf[32..].copy_from_slice(scalar);

            let hashbuf = sha512(&buf);
            private[32..64].copy_from_slice(&hashbuf.as_ref()[..32]);
        }

        (None, Some(public)) => {
            ge_frombytes_negate_vartime(&mut public_key_unpacked, public);

            let pkux = public_key_unpacked.x;
            let pkut = public_key_unpacked.t;
            fe_neg(&mut public_key_unpacked.x, &pkux);
            fe_neg(&mut public_key_unpacked.t, &pkut);

            ge_p3_to_cached(&mut t, &public_key_unpacked);

            ge_scalarmult_base(&mut nb, &n);
            ge_add(&mut a_p1p1, &nb, &t);
            ge_p1p1_to_p3(&mut a, &a_p1p1);

            ge_p3_tobytes(public, &a);
        }

        (None, None) => {}
    }
}
