pub type FE = [i32; 10];

#[inline(always)]
pub fn load_3(input: &[u8]) -> u64 {
    (input[0] as u64) | ((input[1] as u64) << 8) | ((input[2] as u64) << 16)
}

#[inline(always)]
pub fn load_4(input: &[u8]) -> u64 {
    (input[0] as u64)
        | ((input[1] as u64) << 8)
        | ((input[2] as u64) << 16)
        | ((input[3] as u64) << 24)
}

pub fn fe_0(h: &mut FE) {
    *h = [0; 10];
}

pub fn fe_1(h: &mut FE) {
    *h = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
}

pub fn fe_add(h: &mut FE, f: &FE, g: &FE) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let g0 = g[0];
    let g1 = g[1];
    let g2 = g[2];
    let g3 = g[3];
    let g4 = g[4];
    let g5 = g[5];
    let g6 = g[6];
    let g7 = g[7];
    let g8 = g[8];
    let g9 = g[9];
    let h0 = f0 + g0;
    let h1 = f1 + g1;
    let h2 = f2 + g2;
    let h3 = f3 + g3;
    let h4 = f4 + g4;
    let h5 = f5 + g5;
    let h6 = f6 + g6;
    let h7 = f7 + g7;
    let h8 = f8 + g8;
    let h9 = f9 + g9;

    h[0] = h0;
    h[1] = h1;
    h[2] = h2;
    h[3] = h3;
    h[4] = h4;
    h[5] = h5;
    h[6] = h6;
    h[7] = h7;
    h[8] = h8;
    h[9] = h9;
}

pub fn fe_cmov(f: &mut FE, g: &FE, b: u32) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let g0 = g[0];
    let g1 = g[1];
    let g2 = g[2];
    let g3 = g[3];
    let g4 = g[4];
    let g5 = g[5];
    let g6 = g[6];
    let g7 = g[7];
    let g8 = g[8];
    let g9 = g[9];
    let mut x0 = f0 ^ g0;
    let mut x1 = f1 ^ g1;
    let mut x2 = f2 ^ g2;
    let mut x3 = f3 ^ g3;
    let mut x4 = f4 ^ g4;
    let mut x5 = f5 ^ g5;
    let mut x6 = f6 ^ g6;
    let mut x7 = f7 ^ g7;
    let mut x8 = f8 ^ g8;
    let mut x9 = f9 ^ g9;

    let b = -(b as i32);
    x0 &= b;
    x1 &= b;
    x2 &= b;
    x3 &= b;
    x4 &= b;
    x5 &= b;
    x6 &= b;
    x7 &= b;
    x8 &= b;
    x9 &= b;

    f[0] = f0 ^ x0;
    f[1] = f1 ^ x1;
    f[2] = f2 ^ x2;
    f[3] = f3 ^ x3;
    f[4] = f4 ^ x4;
    f[5] = f5 ^ x5;
    f[6] = f6 ^ x6;
    f[7] = f7 ^ x7;
    f[8] = f8 ^ x8;
    f[9] = f9 ^ x9;
}

pub fn fe_cswap(f: &mut FE, g: &mut FE, b: u32) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let g0 = g[0];
    let g1 = g[1];
    let g2 = g[2];
    let g3 = g[3];
    let g4 = g[4];
    let g5 = g[5];
    let g6 = g[6];
    let g7 = g[7];
    let g8 = g[8];
    let g9 = g[9];
    let mut x0 = f0 ^ g0;
    let mut x1 = f1 ^ g1;
    let mut x2 = f2 ^ g2;
    let mut x3 = f3 ^ g3;
    let mut x4 = f4 ^ g4;
    let mut x5 = f5 ^ g5;
    let mut x6 = f6 ^ g6;
    let mut x7 = f7 ^ g7;
    let mut x8 = f8 ^ g8;
    let mut x9 = f9 ^ g9;
    let b = -(b as i32);
    x0 &= b;
    x1 &= b;
    x2 &= b;
    x3 &= b;
    x4 &= b;
    x5 &= b;
    x6 &= b;
    x7 &= b;
    x8 &= b;
    x9 &= b;
    f[0] = f0 ^ x0;
    f[1] = f1 ^ x1;
    f[2] = f2 ^ x2;
    f[3] = f3 ^ x3;
    f[4] = f4 ^ x4;
    f[5] = f5 ^ x5;
    f[6] = f6 ^ x6;
    f[7] = f7 ^ x7;
    f[8] = f8 ^ x8;
    f[9] = f9 ^ x9;
    g[0] = g0 ^ x0;
    g[1] = g1 ^ x1;
    g[2] = g2 ^ x2;
    g[3] = g3 ^ x3;
    g[4] = g4 ^ x4;
    g[5] = g5 ^ x5;
    g[6] = g6 ^ x6;
    g[7] = g7 ^ x7;
    g[8] = g8 ^ x8;
    g[9] = g9 ^ x9;
}

pub fn fe_copy(h: &mut FE, f: &FE) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];

    h[0] = f0;
    h[1] = f1;
    h[2] = f2;
    h[3] = f3;
    h[4] = f4;
    h[5] = f5;
    h[6] = f6;
    h[7] = f7;
    h[8] = f8;
    h[9] = f9;
}

pub fn fe_frombytes(h: &mut FE, s: &[u8; 32]) {
    let mut h0 = load_4(&s[0..]) as i64;
    let mut h1 = (load_3(&s[4..]) << 6) as i64;
    let mut h2 = (load_3(&s[7..]) << 5) as i64;
    let mut h3 = (load_3(&s[10..]) << 3) as i64;
    let mut h4 = (load_3(&s[13..]) << 2) as i64;
    let mut h5 = load_4(&s[16..]) as i64;
    let mut h6 = (load_3(&s[20..]) << 7) as i64;
    let mut h7 = (load_3(&s[23..]) << 5) as i64;
    let mut h8 = (load_3(&s[26..]) << 4) as i64;
    let mut h9 = ((load_3(&s[29..]) & 8388607) << 2) as i64;

    let carry9 = (h9 + (1i64 << 24)) >> 25;
    h0 += carry9 * 19;
    h9 -= carry9 << 25;
    let carry1 = (h1 + (1i64 << 24)) >> 25;
    h2 += carry1;
    h1 -= carry1 << 25;
    let carry3 = (h3 + (1i64 << 24)) >> 25;
    h4 += carry3;
    h3 -= carry3 << 25;
    let carry5 = (h5 + (1i64 << 24)) >> 25;
    h6 += carry5;
    h5 -= carry5 << 25;
    let carry7 = (h7 + (1i64 << 24)) >> 25;
    h8 += carry7;
    h7 -= carry7 << 25;
    let carry0 = (h0 + (1i64 << 25)) >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;
    let carry2 = (h2 + (1i64 << 25)) >> 26;
    h3 += carry2;
    h2 -= carry2 << 26;
    let carry4 = (h4 + (1i64 << 25)) >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;
    let carry6 = (h6 + (1i64 << 25)) >> 26;
    h7 += carry6;
    h6 -= carry6 << 26;
    let carry8 = (h8 + (1i64 << 25)) >> 26;
    h9 += carry8;
    h8 -= carry8 << 26;

    h[0] = h0 as i32;
    h[1] = h1 as i32;
    h[2] = h2 as i32;
    h[3] = h3 as i32;
    h[4] = h4 as i32;
    h[5] = h5 as i32;
    h[6] = h6 as i32;
    h[7] = h7 as i32;
    h[8] = h8 as i32;
    h[9] = h9 as i32;
}

pub fn fe_invert(out: &mut FE, z: &FE) {
    let mut t0 = [0i32; 10];
    let mut t1 = [0i32; 10];
    let mut t2 = [0i32; 10];
    let mut t3 = [0i32; 10];

    // fe_sq(t0, z);
    fe_sq(&mut t0, z);

    // C: for (i = 1; i < 1; ++i) fe_sq(t0, t0);  => 0 itération
    // (donc rien ici)

    // fe_sq(t1, t0);
    {
        let a = t0;
        fe_sq(&mut t1, &a);
    }

    // C: for (i = 1; i < 2; ++i) fe_sq(t1, t1);  => 1 itération
    {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // fe_mul(t1, z, t1);
    {
        let a = t1;
        fe_mul(&mut t1, z, &a);
    }

    // fe_mul(t0, t0, t1);
    {
        let a = t0;
        let b = t1;
        fe_mul(&mut t0, &a, &b);
    }

    // fe_sq(t2, t0);
    {
        let a = t0;
        fe_sq(&mut t2, &a);
    }

    // C: for (i = 1; i < 1; ++i) fe_sq(t2, t2);  => 0 itération
    // (donc rien ici)

    // fe_mul(t1, t1, t2);
    {
        let a = t1;
        let b = t2;
        fe_mul(&mut t1, &a, &b);
    }

    // fe_sq(t2, t1);
    {
        let a = t1;
        fe_sq(&mut t2, &a);
    }

    // for (i = 1; i < 5; ++i) fe_sq(t2, t2);
    for _ in 1..5 {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // fe_mul(t1, t2, t1);
    {
        let a = t2;
        let b = t1;
        fe_mul(&mut t1, &a, &b);
    }

    // fe_sq(t2, t1);
    {
        let a = t1;
        fe_sq(&mut t2, &a);
    }

    // for (i = 1; i < 10; ++i) fe_sq(t2, t2);
    for _ in 1..10 {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // fe_mul(t2, t2, t1);
    {
        let a = t2;
        let b = t1;
        fe_mul(&mut t2, &a, &b);
    }

    // fe_sq(t3, t2);
    {
        let a = t2;
        fe_sq(&mut t3, &a);
    }

    // for (i = 1; i < 20; ++i) fe_sq(t3, t3);
    for _ in 1..20 {
        let a = t3;
        fe_sq(&mut t3, &a);
    }

    // fe_mul(t2, t3, t2);
    {
        let a = t3;
        let b = t2;
        fe_mul(&mut t2, &a, &b);
    }

    // fe_sq(t2, t2);
    {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // for (i = 1; i < 10; ++i) fe_sq(t2, t2);
    for _ in 1..10 {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // fe_mul(t1, t2, t1);
    {
        let a = t2;
        let b = t1;
        fe_mul(&mut t1, &a, &b);
    }

    // fe_sq(t2, t1);
    {
        let a = t1;
        fe_sq(&mut t2, &a);
    }

    // for (i = 1; i < 50; ++i) fe_sq(t2, t2);
    for _ in 1..50 {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // fe_mul(t2, t2, t1);
    {
        let a = t2;
        let b = t1;
        fe_mul(&mut t2, &a, &b);
    }

    // fe_sq(t3, t2);
    {
        let a = t2;
        fe_sq(&mut t3, &a);
    }

    // for (i = 1; i < 100; ++i) fe_sq(t3, t3);
    for _ in 1..100 {
        let a = t3;
        fe_sq(&mut t3, &a);
    }

    // fe_mul(t2, t3, t2);
    {
        let a = t3;
        let b = t2;
        fe_mul(&mut t2, &a, &b);
    }

    // fe_sq(t2, t2);
    {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // for (i = 1; i < 50; ++i) fe_sq(t2, t2);
    for _ in 1..50 {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // fe_mul(t1, t2, t1);
    {
        let a = t2;
        let b = t1;
        fe_mul(&mut t1, &a, &b);
    }

    // fe_sq(t1, t1);
    {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // for (i = 1; i < 5; ++i) fe_sq(t1, t1);
    for _ in 1..5 {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // fe_mul(out, t1, t0);
    {
        let a = t1;
        let b = t0;
        fe_mul(out, &a, &b);
    }
}

pub fn fe_isnegative(f: &FE) -> i32 {
    let mut s = [0u8; 32];

    fe_tobytes(&mut s, f);

    (s[0] & 1) as i32
}

pub fn fe_isnonzero(f: &FE) -> i32 {
    let mut s = [0u8; 32];

    fe_tobytes(&mut s, f);

    let mut r = s[0];
    r |= s[1];
    r |= s[2];
    r |= s[3];
    r |= s[4];
    r |= s[5];
    r |= s[6];
    r |= s[7];
    r |= s[8];
    r |= s[9];
    r |= s[10];
    r |= s[11];
    r |= s[12];
    r |= s[13];
    r |= s[14];
    r |= s[15];
    r |= s[16];
    r |= s[17];
    r |= s[18];
    r |= s[19];
    r |= s[20];
    r |= s[21];
    r |= s[22];
    r |= s[23];
    r |= s[24];
    r |= s[25];
    r |= s[26];
    r |= s[27];
    r |= s[28];
    r |= s[29];
    r |= s[30];
    r |= s[31];

    (r != 0) as i32
}

pub fn fe_mul(h: &mut FE, f: &FE, g: &FE) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let g0 = g[0];
    let g1 = g[1];
    let g2 = g[2];
    let g3 = g[3];
    let g4 = g[4];
    let g5 = g[5];
    let g6 = g[6];
    let g7 = g[7];
    let g8 = g[8];
    let g9 = g[9];
    let g1_19 = 19 * g1;
    let g2_19 = 19 * g2;
    let g3_19 = 19 * g3;
    let g4_19 = 19 * g4;
    let g5_19 = 19 * g5;
    let g6_19 = 19 * g6;
    let g7_19 = 19 * g7;
    let g8_19 = 19 * g8;
    let g9_19 = 19 * g9;
    let f1_2 = 2 * f1;
    let f3_2 = 2 * f3;
    let f5_2 = 2 * f5;
    let f7_2 = 2 * f7;
    let f9_2 = 2 * f9;
    let f0g0 = f0 as i64 * g0 as i64;
    let f0g1 = f0 as i64 * g1 as i64;
    let f0g2 = f0 as i64 * g2 as i64;
    let f0g3 = f0 as i64 * g3 as i64;
    let f0g4 = f0 as i64 * g4 as i64;
    let f0g5 = f0 as i64 * g5 as i64;
    let f0g6 = f0 as i64 * g6 as i64;
    let f0g7 = f0 as i64 * g7 as i64;
    let f0g8 = f0 as i64 * g8 as i64;
    let f0g9 = f0 as i64 * g9 as i64;
    let f1g0 = f1 as i64 * g0 as i64;
    let f1g1_2 = f1_2 as i64 * g1 as i64;
    let f1g2 = f1 as i64 * g2 as i64;
    let f1g3_2 = f1_2 as i64 * g3 as i64;
    let f1g4 = f1 as i64 * g4 as i64;
    let f1g5_2 = f1_2 as i64 * g5 as i64;
    let f1g6 = f1 as i64 * g6 as i64;
    let f1g7_2 = f1_2 as i64 * g7 as i64;
    let f1g8 = f1 as i64 * g8 as i64;
    let f1g9_38 = f1_2 as i64 * g9_19 as i64;
    let f2g0 = f2 as i64 * g0 as i64;
    let f2g1 = f2 as i64 * g1 as i64;
    let f2g2 = f2 as i64 * g2 as i64;
    let f2g3 = f2 as i64 * g3 as i64;
    let f2g4 = f2 as i64 * g4 as i64;
    let f2g5 = f2 as i64 * g5 as i64;
    let f2g6 = f2 as i64 * g6 as i64;
    let f2g7 = f2 as i64 * g7 as i64;
    let f2g8_19 = f2 as i64 * g8_19 as i64;
    let f2g9_19 = f2 as i64 * g9_19 as i64;
    let f3g0 = f3 as i64 * g0 as i64;
    let f3g1_2 = f3_2 as i64 * g1 as i64;
    let f3g2 = f3 as i64 * g2 as i64;
    let f3g3_2 = f3_2 as i64 * g3 as i64;
    let f3g4 = f3 as i64 * g4 as i64;
    let f3g5_2 = f3_2 as i64 * g5 as i64;
    let f3g6 = f3 as i64 * g6 as i64;
    let f3g7_38 = f3_2 as i64 * g7_19 as i64;
    let f3g8_19 = f3 as i64 * g8_19 as i64;
    let f3g9_38 = f3_2 as i64 * g9_19 as i64;
    let f4g0 = f4 as i64 * g0 as i64;
    let f4g1 = f4 as i64 * g1 as i64;
    let f4g2 = f4 as i64 * g2 as i64;
    let f4g3 = f4 as i64 * g3 as i64;
    let f4g4 = f4 as i64 * g4 as i64;
    let f4g5 = f4 as i64 * g5 as i64;
    let f4g6_19 = f4 as i64 * g6_19 as i64;
    let f4g7_19 = f4 as i64 * g7_19 as i64;
    let f4g8_19 = f4 as i64 * g8_19 as i64;
    let f4g9_19 = f4 as i64 * g9_19 as i64;
    let f5g0 = f5 as i64 * g0 as i64;
    let f5g1_2 = f5_2 as i64 * g1 as i64;
    let f5g2 = f5 as i64 * g2 as i64;
    let f5g3_2 = f5_2 as i64 * g3 as i64;
    let f5g4 = f5 as i64 * g4 as i64;
    let f5g5_38 = f5_2 as i64 * g5_19 as i64;
    let f5g6_19 = f5 as i64 * g6_19 as i64;
    let f5g7_38 = f5_2 as i64 * g7_19 as i64;
    let f5g8_19 = f5 as i64 * g8_19 as i64;
    let f5g9_38 = f5_2 as i64 * g9_19 as i64;
    let f6g0 = f6 as i64 * g0 as i64;
    let f6g1 = f6 as i64 * g1 as i64;
    let f6g2 = f6 as i64 * g2 as i64;
    let f6g3 = f6 as i64 * g3 as i64;
    let f6g4_19 = f6 as i64 * g4_19 as i64;
    let f6g5_19 = f6 as i64 * g5_19 as i64;
    let f6g6_19 = f6 as i64 * g6_19 as i64;
    let f6g7_19 = f6 as i64 * g7_19 as i64;
    let f6g8_19 = f6 as i64 * g8_19 as i64;
    let f6g9_19 = f6 as i64 * g9_19 as i64;
    let f7g0 = f7 as i64 * g0 as i64;
    let f7g1_2 = f7_2 as i64 * g1 as i64;
    let f7g2 = f7 as i64 * g2 as i64;
    let f7g3_38 = f7_2 as i64 * g3_19 as i64;
    let f7g4_19 = f7 as i64 * g4_19 as i64;
    let f7g5_38 = f7_2 as i64 * g5_19 as i64;
    let f7g6_19 = f7 as i64 * g6_19 as i64;
    let f7g7_38 = f7_2 as i64 * g7_19 as i64;
    let f7g8_19 = f7 as i64 * g8_19 as i64;
    let f7g9_38 = f7_2 as i64 * g9_19 as i64;
    let f8g0 = f8 as i64 * g0 as i64;
    let f8g1 = f8 as i64 * g1 as i64;
    let f8g2_19 = f8 as i64 * g2_19 as i64;
    let f8g3_19 = f8 as i64 * g3_19 as i64;
    let f8g4_19 = f8 as i64 * g4_19 as i64;
    let f8g5_19 = f8 as i64 * g5_19 as i64;
    let f8g6_19 = f8 as i64 * g6_19 as i64;
    let f8g7_19 = f8 as i64 * g7_19 as i64;
    let f8g8_19 = f8 as i64 * g8_19 as i64;
    let f8g9_19 = f8 as i64 * g9_19 as i64;
    let f9g0 = f9 as i64 * g0 as i64;
    let f9g1_38 = f9_2 as i64 * g1_19 as i64;
    let f9g2_19 = f9 as i64 * g2_19 as i64;
    let f9g3_38 = f9_2 as i64 * g3_19 as i64;
    let f9g4_19 = f9 as i64 * g4_19 as i64;
    let f9g5_38 = f9_2 as i64 * g5_19 as i64;
    let f9g6_19 = f9 as i64 * g6_19 as i64;
    let f9g7_38 = f9_2 as i64 * g7_19 as i64;
    let f9g8_19 = f9 as i64 * g8_19 as i64;
    let f9g9_38 = f9_2 as i64 * g9_19 as i64;
    let mut h0 = f0g0
        + f1g9_38
        + f2g8_19
        + f3g7_38
        + f4g6_19
        + f5g5_38
        + f6g4_19
        + f7g3_38
        + f8g2_19
        + f9g1_38;
    let mut h1 =
        f0g1 + f1g0 + f2g9_19 + f3g8_19 + f4g7_19 + f5g6_19 + f6g5_19 + f7g4_19 + f8g3_19 + f9g2_19;
    let mut h2 =
        f0g2 + f1g1_2 + f2g0 + f3g9_38 + f4g8_19 + f5g7_38 + f6g6_19 + f7g5_38 + f8g4_19 + f9g3_38;
    let mut h3 =
        f0g3 + f1g2 + f2g1 + f3g0 + f4g9_19 + f5g8_19 + f6g7_19 + f7g6_19 + f8g5_19 + f9g4_19;
    let mut h4 =
        f0g4 + f1g3_2 + f2g2 + f3g1_2 + f4g0 + f5g9_38 + f6g8_19 + f7g7_38 + f8g6_19 + f9g5_38;
    let mut h5 = f0g5 + f1g4 + f2g3 + f3g2 + f4g1 + f5g0 + f6g9_19 + f7g8_19 + f8g7_19 + f9g6_19;
    let mut h6 = f0g6 + f1g5_2 + f2g4 + f3g3_2 + f4g2 + f5g1_2 + f6g0 + f7g9_38 + f8g8_19 + f9g7_38;
    let mut h7 = f0g7 + f1g6 + f2g5 + f3g4 + f4g3 + f5g2 + f6g1 + f7g0 + f8g9_19 + f9g8_19;
    let mut h8 = f0g8 + f1g7_2 + f2g6 + f3g5_2 + f4g4 + f5g3_2 + f6g2 + f7g1_2 + f8g0 + f9g9_38;
    let mut h9 = f0g9 + f1g8 + f2g7 + f3g6 + f4g5 + f5g4 + f6g3 + f7g2 + f8g1 + f9g0;

    let mut carry0 = (h0 + (1i64 << 25)) >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;
    let mut carry4 = (h4 + (1i64 << 25)) >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;

    let carry1 = (h1 + (1i64 << 24)) >> 25;
    h2 += carry1;
    h1 -= carry1 << 25;
    let carry5 = (h5 + (1i64 << 24)) >> 25;
    h6 += carry5;
    h5 -= carry5 << 25;

    let carry2 = (h2 + (1i64 << 25)) >> 26;
    h3 += carry2;
    h2 -= carry2 << 26;
    let carry6 = (h6 + (1i64 << 25)) >> 26;
    h7 += carry6;
    h6 -= carry6 << 26;

    let carry3 = (h3 + (1i64 << 24)) >> 25;
    h4 += carry3;
    h3 -= carry3 << 25;
    let carry7 = (h7 + (1i64 << 24)) >> 25;
    h8 += carry7;
    h7 -= carry7 << 25;

    carry4 = (h4 + (1i64 << 25)) >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;
    let carry8 = (h8 + (1i64 << 25)) >> 26;
    h9 += carry8;
    h8 -= carry8 << 26;

    let carry9 = (h9 + (1i64 << 24)) >> 25;
    h0 += carry9 * 19;
    h9 -= carry9 << 25;

    carry0 = (h0 + (1i64 << 25)) >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;

    h[0] = h0 as i32;
    h[1] = h1 as i32;
    h[2] = h2 as i32;
    h[3] = h3 as i32;
    h[4] = h4 as i32;
    h[5] = h5 as i32;
    h[6] = h6 as i32;
    h[7] = h7 as i32;
    h[8] = h8 as i32;
    h[9] = h9 as i32;
}

pub fn fe_mul121666(h: &mut FE, f: &FE) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let mut h0 = f0 as i64 * 121666i64;
    let mut h1 = f1 as i64 * 121666i64;
    let mut h2 = f2 as i64 * 121666i64;
    let mut h3 = f3 as i64 * 121666i64;
    let mut h4 = f4 as i64 * 121666i64;
    let mut h5 = f5 as i64 * 121666i64;
    let mut h6 = f6 as i64 * 121666i64;
    let mut h7 = f7 as i64 * 121666i64;
    let mut h8 = f8 as i64 * 121666i64;
    let mut h9 = f9 as i64 * 121666i64;

    let carry9 = (h9 + (1i64 << 24)) >> 25;
    h0 += carry9 * 19;
    h9 -= carry9 << 25;
    let carry1 = (h1 + (1i64 << 24)) >> 25;
    h2 += carry1;
    h1 -= carry1 << 25;
    let carry3 = (h3 + (1i64 << 24)) >> 25;
    h4 += carry3;
    h3 -= carry3 << 25;
    let carry5 = (h5 + (1i64 << 24)) >> 25;
    h6 += carry5;
    h5 -= carry5 << 25;
    let carry7 = (h7 + (1i64 << 24)) >> 25;
    h8 += carry7;
    h7 -= carry7 << 25;

    let carry0 = (h0 + (1i64 << 25)) >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;
    let carry2 = (h2 + (1i64 << 25)) >> 26;
    h3 += carry2;
    h2 -= carry2 << 26;
    let carry4 = (h4 + (1i64 << 25)) >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;
    let carry6 = (h6 + (1i64 << 25)) >> 26;
    h7 += carry6;
    h6 -= carry6 << 26;
    let carry8 = (h8 + (1i64 << 25)) >> 26;
    h9 += carry8;
    h8 -= carry8 << 26;

    h[0] = h0 as i32;
    h[1] = h1 as i32;
    h[2] = h2 as i32;
    h[3] = h3 as i32;
    h[4] = h4 as i32;
    h[5] = h5 as i32;
    h[6] = h6 as i32;
    h[7] = h7 as i32;
    h[8] = h8 as i32;
    h[9] = h9 as i32;
}

pub fn fe_neg(h: &mut FE, f: &FE) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let h0 = -f0;
    let h1 = -f1;
    let h2 = -f2;
    let h3 = -f3;
    let h4 = -f4;
    let h5 = -f5;
    let h6 = -f6;
    let h7 = -f7;
    let h8 = -f8;
    let h9 = -f9;

    h[0] = h0;
    h[1] = h1;
    h[2] = h2;
    h[3] = h3;
    h[4] = h4;
    h[5] = h5;
    h[6] = h6;
    h[7] = h7;
    h[8] = h8;
    h[9] = h9;
}

pub fn fe_pow22523(out: &mut FE, z: &FE) {
    let mut t0 = [0i32; 10];
    let mut t1 = [0i32; 10];
    let mut t2 = [0i32; 10];

    // fe_sq(t0, z);
    fe_sq(&mut t0, z);

    // for (i = 1; i < 1; ++i) fe_sq(t0, t0);  // 0 itération -> rien

    // fe_sq(t1, t0);
    {
        let a = t0;
        fe_sq(&mut t1, &a);
    }

    // for (i = 1; i < 2; ++i) fe_sq(t1, t1);  // 1 itération
    {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // fe_mul(t1, z, t1);
    {
        let a = t1;
        fe_mul(&mut t1, z, &a);
    }

    // fe_mul(t0, t0, t1);
    {
        let a = t0;
        let b = t1;
        fe_mul(&mut t0, &a, &b);
    }

    // fe_sq(t0, t0);
    {
        let a = t0;
        fe_sq(&mut t0, &a);
    }

    // for (i = 1; i < 1; ++i) fe_sq(t0, t0);  // 0 itération -> rien

    // fe_mul(t0, t1, t0);
    {
        let a = t1;
        let b = t0;
        fe_mul(&mut t0, &a, &b);
    }

    // fe_sq(t1, t0);
    {
        let a = t0;
        fe_sq(&mut t1, &a);
    }

    // for (i = 1; i < 5; ++i) fe_sq(t1, t1);
    for _ in 1..5 {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // fe_mul(t0, t1, t0);
    {
        let a = t1;
        let b = t0;
        fe_mul(&mut t0, &a, &b);
    }

    // fe_sq(t1, t0);
    {
        let a = t0;
        fe_sq(&mut t1, &a);
    }

    // for (i = 1; i < 10; ++i) fe_sq(t1, t1);
    for _ in 1..10 {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // fe_mul(t1, t1, t0);
    {
        let a = t1;
        let b = t0;
        fe_mul(&mut t1, &a, &b);
    }

    // fe_sq(t2, t1);
    {
        let a = t1;
        fe_sq(&mut t2, &a);
    }

    // for (i = 1; i < 20; ++i) fe_sq(t2, t2);
    for _ in 1..20 {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // fe_mul(t1, t2, t1);
    {
        let a = t2;
        let b = t1;
        fe_mul(&mut t1, &a, &b);
    }

    // fe_sq(t1, t1);
    {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // for (i = 1; i < 10; ++i) fe_sq(t1, t1);
    for _ in 1..10 {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // fe_mul(t0, t1, t0);
    {
        let a = t1;
        let b = t0;
        fe_mul(&mut t0, &a, &b);
    }

    // fe_sq(t1, t0);
    {
        let a = t0;
        fe_sq(&mut t1, &a);
    }

    // for (i = 1; i < 50; ++i) fe_sq(t1, t1);
    for _ in 1..50 {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // fe_mul(t1, t1, t0);
    {
        let a = t1;
        let b = t0;
        fe_mul(&mut t1, &a, &b);
    }

    // fe_sq(t2, t1);
    {
        let a = t1;
        fe_sq(&mut t2, &a);
    }

    // for (i = 1; i < 100; ++i) fe_sq(t2, t2);
    for _ in 1..100 {
        let a = t2;
        fe_sq(&mut t2, &a);
    }

    // fe_mul(t1, t2, t1);
    {
        let a = t2;
        let b = t1;
        fe_mul(&mut t1, &a, &b);
    }

    // fe_sq(t1, t1);
    {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // for (i = 1; i < 50; ++i) fe_sq(t1, t1);
    for _ in 1..50 {
        let a = t1;
        fe_sq(&mut t1, &a);
    }

    // fe_mul(t0, t1, t0);
    {
        let a = t1;
        let b = t0;
        fe_mul(&mut t0, &a, &b);
    }

    // fe_sq(t0, t0);
    {
        let a = t0;
        fe_sq(&mut t0, &a);
    }

    // for (i = 1; i < 2; ++i) fe_sq(t0, t0);  // 1 itération
    {
        let a = t0;
        fe_sq(&mut t0, &a);
    }

    // fe_mul(out, t0, z);
    {
        let a = t0;
        fe_mul(out, &a, z);
    }
}

pub fn fe_sq(h: &mut FE, f: &FE) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let f0_2 = 2 * f0;
    let f1_2 = 2 * f1;
    let f2_2 = 2 * f2;
    let f3_2 = 2 * f3;
    let f4_2 = 2 * f4;
    let f5_2 = 2 * f5;
    let f6_2 = 2 * f6;
    let f7_2 = 2 * f7;
    let f5_38 = 38 * f5;
    let f6_19 = 19 * f6;
    let f7_38 = 38 * f7;
    let f8_19 = 19 * f8;
    let f9_38 = 38 * f9;
    let f0f0 = f0 as i64 * f0 as i64;
    let f0f1_2 = f0_2 as i64 * f1 as i64;
    let f0f2_2 = f0_2 as i64 * f2 as i64;
    let f0f3_2 = f0_2 as i64 * f3 as i64;
    let f0f4_2 = f0_2 as i64 * f4 as i64;
    let f0f5_2 = f0_2 as i64 * f5 as i64;
    let f0f6_2 = f0_2 as i64 * f6 as i64;
    let f0f7_2 = f0_2 as i64 * f7 as i64;
    let f0f8_2 = f0_2 as i64 * f8 as i64;
    let f0f9_2 = f0_2 as i64 * f9 as i64;
    let f1f1_2 = f1_2 as i64 * f1 as i64;
    let f1f2_2 = f1_2 as i64 * f2 as i64;
    let f1f3_4 = f1_2 as i64 * f3_2 as i64;
    let f1f4_2 = f1_2 as i64 * f4 as i64;
    let f1f5_4 = f1_2 as i64 * f5_2 as i64;
    let f1f6_2 = f1_2 as i64 * f6 as i64;
    let f1f7_4 = f1_2 as i64 * f7_2 as i64;
    let f1f8_2 = f1_2 as i64 * f8 as i64;
    let f1f9_76 = f1_2 as i64 * f9_38 as i64;
    let f2f2 = f2 as i64 * f2 as i64;
    let f2f3_2 = f2_2 as i64 * f3 as i64;
    let f2f4_2 = f2_2 as i64 * f4 as i64;
    let f2f5_2 = f2_2 as i64 * f5 as i64;
    let f2f6_2 = f2_2 as i64 * f6 as i64;
    let f2f7_2 = f2_2 as i64 * f7 as i64;
    let f2f8_38 = f2_2 as i64 * f8_19 as i64;
    let f2f9_38 = f2 as i64 * f9_38 as i64;
    let f3f3_2 = f3_2 as i64 * f3 as i64;
    let f3f4_2 = f3_2 as i64 * f4 as i64;
    let f3f5_4 = f3_2 as i64 * f5_2 as i64;
    let f3f6_2 = f3_2 as i64 * f6 as i64;
    let f3f7_76 = f3_2 as i64 * f7_38 as i64;
    let f3f8_38 = f3_2 as i64 * f8_19 as i64;
    let f3f9_76 = f3_2 as i64 * f9_38 as i64;
    let f4f4 = f4 as i64 * f4 as i64;
    let f4f5_2 = f4_2 as i64 * f5 as i64;
    let f4f6_38 = f4_2 as i64 * f6_19 as i64;
    let f4f7_38 = f4 as i64 * f7_38 as i64;
    let f4f8_38 = f4_2 as i64 * f8_19 as i64;
    let f4f9_38 = f4 as i64 * f9_38 as i64;
    let f5f5_38 = f5 as i64 * f5_38 as i64;
    let f5f6_38 = f5_2 as i64 * f6_19 as i64;
    let f5f7_76 = f5_2 as i64 * f7_38 as i64;
    let f5f8_38 = f5_2 as i64 * f8_19 as i64;
    let f5f9_76 = f5_2 as i64 * f9_38 as i64;
    let f6f6_19 = f6 as i64 * f6_19 as i64;
    let f6f7_38 = f6 as i64 * f7_38 as i64;
    let f6f8_38 = f6_2 as i64 * f8_19 as i64;
    let f6f9_38 = f6 as i64 * f9_38 as i64;
    let f7f7_38 = f7 as i64 * f7_38 as i64;
    let f7f8_38 = f7_2 as i64 * f8_19 as i64;
    let f7f9_76 = f7_2 as i64 * f9_38 as i64;
    let f8f8_19 = f8 as i64 * f8_19 as i64;
    let f8f9_38 = f8 as i64 * f9_38 as i64;
    let f9f9_38 = f9 as i64 * f9_38 as i64;
    let mut h0 = f0f0 + f1f9_76 + f2f8_38 + f3f7_76 + f4f6_38 + f5f5_38;
    let mut h1 = f0f1_2 + f2f9_38 + f3f8_38 + f4f7_38 + f5f6_38;
    let mut h2 = f0f2_2 + f1f1_2 + f3f9_76 + f4f8_38 + f5f7_76 + f6f6_19;
    let mut h3 = f0f3_2 + f1f2_2 + f4f9_38 + f5f8_38 + f6f7_38;
    let mut h4 = f0f4_2 + f1f3_4 + f2f2 + f5f9_76 + f6f8_38 + f7f7_38;
    let mut h5 = f0f5_2 + f1f4_2 + f2f3_2 + f6f9_38 + f7f8_38;
    let mut h6 = f0f6_2 + f1f5_4 + f2f4_2 + f3f3_2 + f7f9_76 + f8f8_19;
    let mut h7 = f0f7_2 + f1f6_2 + f2f5_2 + f3f4_2 + f8f9_38;
    let mut h8 = f0f8_2 + f1f7_4 + f2f6_2 + f3f5_4 + f4f4 + f9f9_38;
    let mut h9 = f0f9_2 + f1f8_2 + f2f7_2 + f3f6_2 + f4f5_2;

    let mut carry0 = (h0 + (1i64 << 25)) >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;
    let mut carry4 = (h4 + (1i64 << 25)) >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;
    let carry1 = (h1 + (1i64 << 24)) >> 25;
    h2 += carry1;
    h1 -= carry1 << 25;
    let carry5 = (h5 + (1i64 << 24)) >> 25;
    h6 += carry5;
    h5 -= carry5 << 25;
    let carry2 = (h2 + (1i64 << 25)) >> 26;
    h3 += carry2;
    h2 -= carry2 << 26;
    let carry6 = (h6 + (1i64 << 25)) >> 26;
    h7 += carry6;
    h6 -= carry6 << 26;
    let carry3 = (h3 + (1i64 << 24)) >> 25;
    h4 += carry3;
    h3 -= carry3 << 25;
    let carry7 = (h7 + (1i64 << 24)) >> 25;
    h8 += carry7;
    h7 -= carry7 << 25;
    carry4 = (h4 + (1i64 << 25)) >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;
    let carry8 = (h8 + (1i64 << 25)) >> 26;
    h9 += carry8;
    h8 -= carry8 << 26;
    let carry9 = (h9 + (1i64 << 24)) >> 25;
    h0 += carry9 * 19;
    h9 -= carry9 << 25;
    carry0 = (h0 + (1i64 << 25)) >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;
    h[0] = h0 as i32;
    h[1] = h1 as i32;
    h[2] = h2 as i32;
    h[3] = h3 as i32;
    h[4] = h4 as i32;
    h[5] = h5 as i32;
    h[6] = h6 as i32;
    h[7] = h7 as i32;
    h[8] = h8 as i32;
    h[9] = h9 as i32;
}

pub fn fe_sq2(h: &mut FE, f: &FE) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let f0_2 = 2 * f0;
    let f1_2 = 2 * f1;
    let f2_2 = 2 * f2;
    let f3_2 = 2 * f3;
    let f4_2 = 2 * f4;
    let f5_2 = 2 * f5;
    let f6_2 = 2 * f6;
    let f7_2 = 2 * f7;
    let f5_38 = 38 * f5;
    let f6_19 = 19 * f6;
    let f7_38 = 38 * f7;
    let f8_19 = 19 * f8;
    let f9_38 = 38 * f9;
    let f0f0 = f0 as i64 * f0 as i64;
    let f0f1_2 = f0_2 as i64 * f1 as i64;
    let f0f2_2 = f0_2 as i64 * f2 as i64;
    let f0f3_2 = f0_2 as i64 * f3 as i64;
    let f0f4_2 = f0_2 as i64 * f4 as i64;
    let f0f5_2 = f0_2 as i64 * f5 as i64;
    let f0f6_2 = f0_2 as i64 * f6 as i64;
    let f0f7_2 = f0_2 as i64 * f7 as i64;
    let f0f8_2 = f0_2 as i64 * f8 as i64;
    let f0f9_2 = f0_2 as i64 * f9 as i64;
    let f1f1_2 = f1_2 as i64 * f1 as i64;
    let f1f2_2 = f1_2 as i64 * f2 as i64;
    let f1f3_4 = f1_2 as i64 * f3_2 as i64;
    let f1f4_2 = f1_2 as i64 * f4 as i64;
    let f1f5_4 = f1_2 as i64 * f5_2 as i64;
    let f1f6_2 = f1_2 as i64 * f6 as i64;
    let f1f7_4 = f1_2 as i64 * f7_2 as i64;
    let f1f8_2 = f1_2 as i64 * f8 as i64;
    let f1f9_76 = f1_2 as i64 * f9_38 as i64;
    let f2f2 = f2 as i64 * f2 as i64;
    let f2f3_2 = f2_2 as i64 * f3 as i64;
    let f2f4_2 = f2_2 as i64 * f4 as i64;
    let f2f5_2 = f2_2 as i64 * f5 as i64;
    let f2f6_2 = f2_2 as i64 * f6 as i64;
    let f2f7_2 = f2_2 as i64 * f7 as i64;
    let f2f8_38 = f2_2 as i64 * f8_19 as i64;
    let f2f9_38 = f2 as i64 * f9_38 as i64;
    let f3f3_2 = f3_2 as i64 * f3 as i64;
    let f3f4_2 = f3_2 as i64 * f4 as i64;
    let f3f5_4 = f3_2 as i64 * f5_2 as i64;
    let f3f6_2 = f3_2 as i64 * f6 as i64;
    let f3f7_76 = f3_2 as i64 * f7_38 as i64;
    let f3f8_38 = f3_2 as i64 * f8_19 as i64;
    let f3f9_76 = f3_2 as i64 * f9_38 as i64;
    let f4f4 = f4 as i64 * f4 as i64;
    let f4f5_2 = f4_2 as i64 * f5 as i64;
    let f4f6_38 = f4_2 as i64 * f6_19 as i64;
    let f4f7_38 = f4 as i64 * f7_38 as i64;
    let f4f8_38 = f4_2 as i64 * f8_19 as i64;
    let f4f9_38 = f4 as i64 * f9_38 as i64;
    let f5f5_38 = f5 as i64 * f5_38 as i64;
    let f5f6_38 = f5_2 as i64 * f6_19 as i64;
    let f5f7_76 = f5_2 as i64 * f7_38 as i64;
    let f5f8_38 = f5_2 as i64 * f8_19 as i64;
    let f5f9_76 = f5_2 as i64 * f9_38 as i64;
    let f6f6_19 = f6 as i64 * f6_19 as i64;
    let f6f7_38 = f6 as i64 * f7_38 as i64;
    let f6f8_38 = f6_2 as i64 * f8_19 as i64;
    let f6f9_38 = f6 as i64 * f9_38 as i64;
    let f7f7_38 = f7 as i64 * f7_38 as i64;
    let f7f8_38 = f7_2 as i64 * f8_19 as i64;
    let f7f9_76 = f7_2 as i64 * f9_38 as i64;
    let f8f8_19 = f8 as i64 * f8_19 as i64;
    let f8f9_38 = f8 as i64 * f9_38 as i64;
    let f9f9_38 = f9 as i64 * f9_38 as i64;
    let mut h0 = f0f0 + f1f9_76 + f2f8_38 + f3f7_76 + f4f6_38 + f5f5_38;
    let mut h1 = f0f1_2 + f2f9_38 + f3f8_38 + f4f7_38 + f5f6_38;
    let mut h2 = f0f2_2 + f1f1_2 + f3f9_76 + f4f8_38 + f5f7_76 + f6f6_19;
    let mut h3 = f0f3_2 + f1f2_2 + f4f9_38 + f5f8_38 + f6f7_38;
    let mut h4 = f0f4_2 + f1f3_4 + f2f2 + f5f9_76 + f6f8_38 + f7f7_38;
    let mut h5 = f0f5_2 + f1f4_2 + f2f3_2 + f6f9_38 + f7f8_38;
    let mut h6 = f0f6_2 + f1f5_4 + f2f4_2 + f3f3_2 + f7f9_76 + f8f8_19;
    let mut h7 = f0f7_2 + f1f6_2 + f2f5_2 + f3f4_2 + f8f9_38;
    let mut h8 = f0f8_2 + f1f7_4 + f2f6_2 + f3f5_4 + f4f4 + f9f9_38;
    let mut h9 = f0f9_2 + f1f8_2 + f2f7_2 + f3f6_2 + f4f5_2;

    h0 += h0;
    h1 += h1;
    h2 += h2;
    h3 += h3;
    h4 += h4;
    h5 += h5;
    h6 += h6;
    h7 += h7;
    h8 += h8;
    h9 += h9;

    let mut carry0 = (h0 + (1i64 << 25)) >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;
    let mut carry4 = (h4 + (1i64 << 25)) >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;
    let carry1 = (h1 + (1i64 << 24)) >> 25;
    h2 += carry1;
    h1 -= carry1 << 25;
    let carry5 = (h5 + (1i64 << 24)) >> 25;
    h6 += carry5;
    h5 -= carry5 << 25;
    let carry2 = (h2 + (1i64 << 25)) >> 26;
    h3 += carry2;
    h2 -= carry2 << 26;
    let carry6 = (h6 + (1i64 << 25)) >> 26;
    h7 += carry6;
    h6 -= carry6 << 26;
    let carry3 = (h3 + (1i64 << 24)) >> 25;
    h4 += carry3;
    h3 -= carry3 << 25;
    let carry7 = (h7 + (1i64 << 24)) >> 25;
    h8 += carry7;
    h7 -= carry7 << 25;
    carry4 = (h4 + (1i64 << 25)) >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;
    let carry8 = (h8 + (1i64 << 25)) >> 26;
    h9 += carry8;
    h8 -= carry8 << 26;
    let carry9 = (h9 + (1i64 << 24)) >> 25;
    h0 += carry9 * 19;
    h9 -= carry9 << 25;
    carry0 = (h0 + (1i64 << 25)) >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;
    h[0] = h0 as i32;
    h[1] = h1 as i32;
    h[2] = h2 as i32;
    h[3] = h3 as i32;
    h[4] = h4 as i32;
    h[5] = h5 as i32;
    h[6] = h6 as i32;
    h[7] = h7 as i32;
    h[8] = h8 as i32;
    h[9] = h9 as i32;
}
pub fn fe_sub(h: &mut FE, f: &FE, g: &FE) {
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let g0 = g[0];
    let g1 = g[1];
    let g2 = g[2];
    let g3 = g[3];
    let g4 = g[4];
    let g5 = g[5];
    let g6 = g[6];
    let g7 = g[7];
    let g8 = g[8];
    let g9 = g[9];
    let h0 = f0 - g0;
    let h1 = f1 - g1;
    let h2 = f2 - g2;
    let h3 = f3 - g3;
    let h4 = f4 - g4;
    let h5 = f5 - g5;
    let h6 = f6 - g6;
    let h7 = f7 - g7;
    let h8 = f8 - g8;
    let h9 = f9 - g9;

    h[0] = h0;
    h[1] = h1;
    h[2] = h2;
    h[3] = h3;
    h[4] = h4;
    h[5] = h5;
    h[6] = h6;
    h[7] = h7;
    h[8] = h8;
    h[9] = h9;
}

pub fn fe_tobytes(s: &mut [u8; 32], h: &FE) {
    let mut h0 = h[0];
    let mut h1 = h[1];
    let mut h2 = h[2];
    let mut h3 = h[3];
    let mut h4 = h[4];
    let mut h5 = h[5];
    let mut h6 = h[6];
    let mut h7 = h[7];
    let mut h8 = h[8];
    let mut h9 = h[9];

    let mut q = (19 * h9 + (1i32 << 24)) >> 25;
    q = (h0 + q) >> 26;
    q = (h1 + q) >> 25;
    q = (h2 + q) >> 26;
    q = (h3 + q) >> 25;
    q = (h4 + q) >> 26;
    q = (h5 + q) >> 25;
    q = (h6 + q) >> 26;
    q = (h7 + q) >> 25;
    q = (h8 + q) >> 26;
    q = (h9 + q) >> 25;

    h0 += 19 * q;

    let carry0 = h0 >> 26;
    h1 += carry0;
    h0 -= carry0 << 26;
    let carry1 = h1 >> 25;
    h2 += carry1;
    h1 -= carry1 << 25;
    let carry2 = h2 >> 26;
    h3 += carry2;
    h2 -= carry2 << 26;
    let carry3 = h3 >> 25;
    h4 += carry3;
    h3 -= carry3 << 25;
    let carry4 = h4 >> 26;
    h5 += carry4;
    h4 -= carry4 << 26;
    let carry5 = h5 >> 25;
    h6 += carry5;
    h5 -= carry5 << 25;
    let carry6 = h6 >> 26;
    h7 += carry6;
    h6 -= carry6 << 26;
    let carry7 = h7 >> 25;
    h8 += carry7;
    h7 -= carry7 << 25;
    let carry8 = h8 >> 26;
    h9 += carry8;
    h8 -= carry8 << 26;
    let carry9 = h9 >> 25;
    h9 -= carry9 << 25;

    s[0] = h0 as u8;
    s[1] = (h0 >> 8) as u8;
    s[2] = (h0 >> 16) as u8;
    s[3] = ((h0 >> 24) | (h1 << 2)) as u8;
    s[4] = (h1 >> 6) as u8;
    s[5] = (h1 >> 14) as u8;
    s[6] = ((h1 >> 22) | (h2 << 3)) as u8;
    s[7] = (h2 >> 5) as u8;
    s[8] = (h2 >> 13) as u8;
    s[9] = ((h2 >> 21) | (h3 << 5)) as u8;
    s[10] = (h3 >> 3) as u8;
    s[11] = (h3 >> 11) as u8;
    s[12] = ((h3 >> 19) | (h4 << 6)) as u8;
    s[13] = (h4 >> 2) as u8;
    s[14] = (h4 >> 10) as u8;
    s[15] = (h4 >> 18) as u8;
    s[16] = h5 as u8;
    s[17] = (h5 >> 8) as u8;
    s[18] = (h5 >> 16) as u8;
    s[19] = ((h5 >> 24) | (h6 << 1)) as u8;
    s[20] = (h6 >> 7) as u8;
    s[21] = (h6 >> 15) as u8;
    s[22] = ((h6 >> 23) | (h7 << 3)) as u8;
    s[23] = (h7 >> 5) as u8;
    s[24] = (h7 >> 13) as u8;
    s[25] = ((h7 >> 21) | (h8 << 4)) as u8;
    s[26] = (h8 >> 4) as u8;
    s[27] = (h8 >> 12) as u8;
    s[28] = ((h8 >> 20) | (h9 << 6)) as u8;
    s[29] = (h9 >> 2) as u8;
    s[30] = (h9 >> 10) as u8;
    s[31] = (h9 >> 18) as u8;
}
