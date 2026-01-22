use super::field::FieldElement;

pub fn ed25519_key_exchange(
    shared_secret: &mut [u8; 32],
    public_key: &[u8; 32],
    private_key: &[u8; 32],
) {
    let mut e = [0u8; 32];
    e.copy_from_slice(private_key);

    e[0] &= 248;
    e[31] &= 63;
    e[31] |= 64;

    let mut x1;
    let mut x2;
    let mut z2;
    let mut x3;
    let mut z3;
    let mut tmp0;
    let mut tmp1;

    // unpack + edwards -> montgomery
    x1 = FieldElement::from_bytes(public_key);

    tmp1 = FieldElement::ONE;
    tmp0 = x1 + tmp1;
    tmp1 = tmp1 - x1;
    tmp1 = tmp1.invert();
    x1 = tmp0 * tmp1;

    x2 = FieldElement::ONE;
    z2 = FieldElement::ZERO;
    x3 = x1;
    z3 = FieldElement::ONE;

    let mut swap: u32 = 0;

    for pos in (0..=254).rev() {
        let b = (e[pos / 8] >> (pos & 7)) & 1;
        let b_u32 = b as u32;

        swap ^= b_u32;
        x2.swap(&mut x3, swap);
        z2.swap(&mut z3, swap);
        swap = b_u32;

        tmp0 = x3 - z3;
        tmp1 = x2 - z2;
        x2 = x2 + z2;
        z2 = x3 + z3;
        z3 = tmp0 * x2;
        z2 = z2 * tmp1;
        tmp0 = tmp1.sq();
        tmp1 = x2.sq();
        x3 = z3 + z2;
        z2 = z3 - z2;
        x2 = tmp1 * tmp0;
        tmp1 = tmp1 - tmp0;
        z2 = z2.sq();
        z3 = tmp1.mul121666();
        x3 = x3.sq();
        tmp0 = tmp0 + z3;
        z3 = x1 * z2;
        z2 = tmp1 * tmp0;
    }

    x2.swap(&mut x3, swap);
    z2.swap(&mut z3, swap);

    z2 = z2.invert();
    x2 = x2 * z2;

    *shared_secret = x2.to_bytes();
}
