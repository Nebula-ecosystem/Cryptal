use crate::fe::{
    FE, fe_0, fe_1, fe_add, fe_copy, fe_cswap, fe_frombytes, fe_invert, fe_mul, fe_mul121666,
    fe_sq, fe_sub, fe_tobytes,
};

pub fn ed25519_key_exchange(
    shared_secret: &mut [u8; 32],
    public_key: &[u8; 32],
    private_key: &[u8; 32],
) {
    let mut e = [0u8; 32];

    let mut x1: FE = [0i32; 10];
    let mut x2: FE = [0i32; 10];
    let mut z2: FE = [0i32; 10];
    let mut x3: FE = [0i32; 10];
    let mut z3: FE = [0i32; 10];
    let mut tmp0: FE = [0i32; 10];
    let mut tmp1: FE = [0i32; 10];

    let x2_copy: FE = x2;
    let z2_copy: FE = z2;
    let x3_copy: FE = x3;
    let tmp0_copy: FE = tmp0;
    let tmp1_copy: FE = tmp1;

    let mut swap: u32 = 0;
    let mut b: u32;

    for (e_i, pk_i) in e.iter_mut().take(32).zip(private_key.iter().take(32)) {
        *e_i = *pk_i;
    }

    e[0] &= 248;
    e[31] &= 63;
    e[31] |= 64;

    fe_frombytes(&mut x1, public_key);
    fe_1(&mut tmp1);
    fe_add(&mut tmp0, &x1, &tmp1);
    fe_sub(&mut tmp1, &tmp1_copy, &x1);
    fe_invert(&mut tmp1, &tmp1_copy);
    fe_mul(&mut x1, &tmp0, &tmp1);

    fe_1(&mut x2);
    fe_0(&mut z2);
    fe_copy(&mut x3, &x1);
    fe_1(&mut z3);

    for pos in (0..=254).rev() {
        b = ((e[pos / 8] >> (pos & 7)) & 1) as u32;
        swap ^= b;
        fe_cswap(&mut x2, &mut x3, swap);
        fe_cswap(&mut z2, &mut z3, swap);
        swap = b;

        fe_sub(&mut tmp0, &x3, &z3);
        fe_sub(&mut tmp1, &x2, &z2);
        fe_add(&mut x2, &x2_copy, &z2);
        fe_add(&mut z2, &x3, &z3);
        fe_mul(&mut z3, &tmp0, &x2);
        fe_mul(&mut z2, &z2_copy, &tmp1);
        fe_sq(&mut tmp0, &tmp1);
        fe_sq(&mut tmp1, &x2);
        fe_add(&mut x3, &z3, &z2);
        fe_sub(&mut z2, &z3, &z2_copy);
        fe_mul(&mut x2, &tmp1, &tmp0);
        fe_sub(&mut tmp1, &tmp1_copy, &tmp0);
        fe_sq(&mut z2, &z2_copy);
        fe_mul121666(&mut z3, &tmp1);
        fe_sq(&mut x3, &x3_copy);
        fe_add(&mut tmp0, &tmp0_copy, &z3);
        fe_mul(&mut z3, &x1, &z2);
        fe_mul(&mut z2, &tmp1, &tmp0);
    }

    fe_cswap(&mut x2, &mut x3, swap);
    fe_cswap(&mut z2, &mut z3, swap);

    fe_invert(&mut z2, &z2_copy);
    fe_mul(&mut x2, &x2_copy, &z2);
    fe_tobytes(shared_secret, &x2);
}
