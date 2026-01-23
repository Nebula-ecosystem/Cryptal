use std::array;
use std::ops::{Add, Mul, Neg, Sub};

pub(crate) const D: FieldElement = FieldElement([
    -10913610, 13857413, -15372611, 6949391, 114729, -8787816, -6275908, -3247719, -18696448,
    -12055116,
]);

pub(crate) const SQRTM1: FieldElement = FieldElement([
    -32595792, -7943725, 9377950, 3500415, 12389472, -272473, -25146209, -2005654, 326686, 11406482,
]);

pub(crate) const D2: FieldElement = FieldElement([
    -21827239, -5839606, -30745221, 13898782, 229458, 15978800, -12551817, -6495438, 29715968,
    9444199,
]);

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

#[derive(Clone, Copy)]
pub(crate) struct FieldElement(pub(crate) [i32; 10]);

impl FieldElement {
    pub(crate) const ZERO: Self = FieldElement([0i32; 10]);
    pub(crate) const ONE: Self = FieldElement([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    pub(crate) fn swap(&mut self, rhs: &mut Self, condition: u32) {
        let mask = -(condition as i32);

        for (s, r) in self.0.iter_mut().zip(rhs.0.iter_mut()) {
            let tmp = (*s ^ *r) & mask;

            *s ^= tmp;
            *r ^= tmp;
        }
    }

    pub(crate) fn conditional_move(&mut self, rhs: &Self, condition: u32) {
        let mask = -(condition as i32);

        for (s, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            let tmp = (*s ^ r) & mask;

            *s ^= tmp;
        }
    }

    pub(crate) fn from_bytes(input: &[u8; 32]) -> FieldElement {
        let mut output = [
            load_4(&input[0..]) as i64,
            (load_3(&input[4..]) << 6) as i64,
            (load_3(&input[7..]) << 5) as i64,
            (load_3(&input[10..]) << 3) as i64,
            (load_3(&input[13..]) << 2) as i64,
            load_4(&input[16..]) as i64,
            (load_3(&input[20..]) << 7) as i64,
            (load_3(&input[23..]) << 5) as i64,
            (load_3(&input[26..]) << 4) as i64,
            ((load_3(&input[29..]) & 8_388_607) << 2) as i64,
        ];
        let mut carries = [0i64; 10];

        carries[9] = (output[9] + (1i64 << 24)) >> 25;
        output[0] += carries[9] * 19;
        output[9] -= carries[9] << 25;

        carries[1] = (output[1] + (1i64 << 24)) >> 25;
        output[2] += carries[1];
        output[1] -= carries[1] << 25;

        carries[3] = (output[3] + (1i64 << 24)) >> 25;
        output[4] += carries[3];
        output[3] -= carries[3] << 25;

        carries[5] = (output[5] + (1i64 << 24)) >> 25;
        output[6] += carries[5];
        output[5] -= carries[5] << 25;

        carries[7] = (output[7] + (1i64 << 24)) >> 25;
        output[8] += carries[7];
        output[7] -= carries[7] << 25;

        carries[0] = (output[0] + (1i64 << 25)) >> 26;
        output[1] += carries[0];
        output[0] -= carries[0] << 26;

        carries[2] = (output[2] + (1i64 << 25)) >> 26;
        output[3] += carries[2];
        output[2] -= carries[2] << 26;

        carries[4] = (output[4] + (1i64 << 25)) >> 26;
        output[5] += carries[4];
        output[4] -= carries[4] << 26;

        carries[6] = (output[6] + (1i64 << 25)) >> 26;
        output[7] += carries[6];
        output[6] -= carries[6] << 26;

        carries[8] = (output[8] + (1i64 << 25)) >> 26;
        output[9] += carries[8];
        output[8] -= carries[8] << 26;

        FieldElement(array::from_fn(|i| output[i] as i32))
    }

    pub(crate) fn to_bytes(self) -> [u8; 32] {
        let mut input = self.0.map(|x| x as i64);
        let mut carry;

        carry = (19 * input[9] + (1i64 << 24)) >> 25;
        carry = (input[0] + carry) >> 26;
        carry = (input[1] + carry) >> 25;
        carry = (input[2] + carry) >> 26;
        carry = (input[3] + carry) >> 25;
        carry = (input[4] + carry) >> 26;
        carry = (input[5] + carry) >> 25;
        carry = (input[6] + carry) >> 26;
        carry = (input[7] + carry) >> 25;
        carry = (input[8] + carry) >> 26;
        carry = (input[9] + carry) >> 25;

        input[0] += 19 * carry;

        carry = input[0] >> 26;
        input[1] += carry;
        input[0] -= carry << 26;

        carry = input[1] >> 25;
        input[2] += carry;
        input[1] -= carry << 25;

        carry = input[2] >> 26;
        input[3] += carry;
        input[2] -= carry << 26;

        carry = input[3] >> 25;
        input[4] += carry;
        input[3] -= carry << 25;

        carry = input[4] >> 26;
        input[5] += carry;
        input[4] -= carry << 26;

        carry = input[5] >> 25;
        input[6] += carry;
        input[5] -= carry << 25;

        carry = input[6] >> 26;
        input[7] += carry;
        input[6] -= carry << 26;

        carry = input[7] >> 25;
        input[8] += carry;
        input[7] -= carry << 25;

        carry = input[8] >> 26;
        input[9] += carry;
        input[8] -= carry << 26;

        carry = input[9] >> 25;
        input[9] -= carry << 25;

        let mut output = [0u8; 32];

        output[0] = input[0] as u8;
        output[1] = (input[0] >> 8) as u8;
        output[2] = (input[0] >> 16) as u8;
        output[3] = ((input[0] >> 24) | (input[1] << 2)) as u8;
        output[4] = (input[1] >> 6) as u8;
        output[5] = (input[1] >> 14) as u8;
        output[6] = ((input[1] >> 22) | (input[2] << 3)) as u8;
        output[7] = (input[2] >> 5) as u8;
        output[8] = (input[2] >> 13) as u8;
        output[9] = ((input[2] >> 21) | (input[3] << 5)) as u8;
        output[10] = (input[3] >> 3) as u8;
        output[11] = (input[3] >> 11) as u8;
        output[12] = ((input[3] >> 19) | (input[4] << 6)) as u8;
        output[13] = (input[4] >> 2) as u8;
        output[14] = (input[4] >> 10) as u8;
        output[15] = (input[4] >> 18) as u8;
        output[16] = input[5] as u8;
        output[17] = (input[5] >> 8) as u8;
        output[18] = (input[5] >> 16) as u8;
        output[19] = ((input[5] >> 24) | (input[6] << 1)) as u8;
        output[20] = (input[6] >> 7) as u8;
        output[21] = (input[6] >> 15) as u8;
        output[22] = ((input[6] >> 23) | (input[7] << 3)) as u8;
        output[23] = (input[7] >> 5) as u8;
        output[24] = (input[7] >> 13) as u8;
        output[25] = ((input[7] >> 21) | (input[8] << 4)) as u8;
        output[26] = (input[8] >> 4) as u8;
        output[27] = (input[8] >> 12) as u8;
        output[28] = ((input[8] >> 20) | (input[9] << 6)) as u8;
        output[29] = (input[9] >> 2) as u8;
        output[30] = (input[9] >> 10) as u8;
        output[31] = (input[9] >> 18) as u8;

        output
    }

    pub(crate) fn is_non_zero(&self) -> i32 {
        let s = self.to_bytes();

        let mut r = 0;
        for item in s {
            r |= item;
        }

        (r != 0) as i32
    }

    pub(crate) fn is_negative(&self) -> i32 {
        (self.to_bytes()[0] & 1) as i32
    }

    pub(crate) fn mul121666(&self) -> Self {
        let input = self.0.map(|x| x as i64);
        let mut output = input.map(|x| x * 121_666i64);

        let mut carry;

        carry = (output[9] + (1i64 << 24)) >> 25;
        output[0] += carry * 19;
        output[9] -= carry << 25;

        carry = (output[1] + (1i64 << 24)) >> 25;
        output[2] += carry;
        output[1] -= carry << 25;

        carry = (output[3] + (1i64 << 24)) >> 25;
        output[4] += carry;
        output[3] -= carry << 25;

        carry = (output[5] + (1i64 << 24)) >> 25;
        output[6] += carry;
        output[5] -= carry << 25;

        carry = (output[7] + (1i64 << 24)) >> 25;
        output[8] += carry;
        output[7] -= carry << 25;

        carry = (output[0] + (1i64 << 25)) >> 26;
        output[1] += carry;
        output[0] -= carry << 26;

        carry = (output[2] + (1i64 << 25)) >> 26;
        output[3] += carry;
        output[2] -= carry << 26;

        carry = (output[4] + (1i64 << 25)) >> 26;
        output[5] += carry;
        output[4] -= carry << 26;

        carry = (output[6] + (1i64 << 25)) >> 26;
        output[7] += carry;
        output[6] -= carry << 26;

        carry = (output[8] + (1i64 << 25)) >> 26;
        output[9] += carry;
        output[8] -= carry << 26;

        FieldElement(output.map(|x| x as i32))
    }

    pub(crate) fn pow22523(&self) -> Self {
        let mut t0 = self.sq();
        let mut t1 = t0.sq();
        t1 = t1.sq();

        t1 = *self * t1;
        t0 = t0 * t1;

        t0 = t0.sq();
        t0 = t1 * t0;

        t1 = t0.sq();
        for _ in 1..5 {
            t1 = t1.sq();
        }
        t0 = t1 * t0;

        t1 = t0.sq();
        for _ in 1..10 {
            t1 = t1.sq();
        }
        t1 = t1 * t0;

        let mut t2 = t1.sq();
        for _ in 1..20 {
            t2 = t2.sq();
        }
        t1 = t2 * t1;

        t1 = t1.sq();
        for _ in 1..10 {
            t1 = t1.sq();
        }
        t0 = t1 * t0;

        t1 = t0.sq();
        for _ in 1..50 {
            t1 = t1.sq();
        }
        t1 = t1 * t0;

        t2 = t1.sq();
        for _ in 1..100 {
            t2 = t2.sq();
        }
        t1 = t2 * t1;

        t1 = t1.sq();
        for _ in 1..50 {
            t1 = t1.sq();
        }
        t0 = t1 * t0;

        t0 = t0.sq();
        t0 = t0.sq();

        t0 * *self
    }

    pub(crate) fn sq(self) -> FieldElement {
        let f = self.0;

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

        FieldElement([
            h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32, h5 as i32, h6 as i32, h7 as i32,
            h8 as i32, h9 as i32,
        ])
    }

    pub(crate) fn sq2(self) -> FieldElement {
        let f = self.0;

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

        // ⬅️ différence clé avec sq()
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

        FieldElement([
            h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32, h5 as i32, h6 as i32, h7 as i32,
            h8 as i32, h9 as i32,
        ])
    }

    pub(crate) fn invert(&self) -> Self {
        let mut t0 = self.sq();
        let mut t1 = t0.sq();
        t1 = t1.sq();

        t1 = *self * t1;
        t0 = t0 * t1;

        let mut t2 = t0.sq();
        t1 = t1 * t2;

        t2 = t1.sq();
        for _ in 1..5 {
            t2 = t2.sq();
        }
        t1 = t2 * t1;

        t2 = t1.sq();
        for _ in 1..10 {
            t2 = t2.sq();
        }
        t2 = t2 * t1;

        let mut t3 = t2.sq();
        for _ in 1..20 {
            t3 = t3.sq();
        }
        t2 = t3 * t2;

        t2 = t2.sq();
        for _ in 1..10 {
            t2 = t2.sq();
        }
        t1 = t2 * t1;

        t2 = t1.sq();
        for _ in 1..50 {
            t2 = t2.sq();
        }
        t2 = t2 * t1;

        t3 = t2.sq();
        for _ in 1..100 {
            t3 = t3.sq();
        }
        t2 = t3 * t2;

        t2 = t2.sq();
        for _ in 1..50 {
            t2 = t2.sq();
        }
        t1 = t2 * t1;

        t1 = t1.sq();
        for _ in 1..5 {
            t1 = t1.sq();
        }

        t1 * t0
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = FieldElement::ZERO;

        for (out, (a, b)) in output.0.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
            *out = (*a as i64 + *b as i64) as i32;
        }

        output
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = FieldElement::ZERO;

        for (out, (a, b)) in output.0.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
            *out = (*a as i64 - *b as i64) as i32;
        }

        output
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        let f0 = self.0[0];
        let f1 = self.0[1];
        let f2 = self.0[2];
        let f3 = self.0[3];
        let f4 = self.0[4];
        let f5 = self.0[5];
        let f6 = self.0[6];
        let f7 = self.0[7];
        let f8 = self.0[8];
        let f9 = self.0[9];

        let g0 = rhs.0[0];
        let g1 = rhs.0[1];
        let g2 = rhs.0[2];
        let g3 = rhs.0[3];
        let g4 = rhs.0[4];
        let g5 = rhs.0[5];
        let g6 = rhs.0[6];
        let g7 = rhs.0[7];
        let g8 = rhs.0[8];
        let g9 = rhs.0[9];

        let g1_19 = g1 * 19;
        let g2_19 = g2 * 19;
        let g3_19 = g3 * 19;
        let g4_19 = g4 * 19;
        let g5_19 = g5 * 19;
        let g6_19 = g6 * 19;
        let g7_19 = g7 * 19;
        let g8_19 = g8 * 19;
        let g9_19 = g9 * 19;

        let f1_2 = f1 * 2;
        let f3_2 = f3 * 2;
        let f5_2 = f5 * 2;
        let f7_2 = f7 * 2;
        let f9_2 = f9 * 2;

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

        let mut h1 = f0g1
            + f1g0
            + f2g9_19
            + f3g8_19
            + f4g7_19
            + f5g6_19
            + f6g5_19
            + f7g4_19
            + f8g3_19
            + f9g2_19;

        let mut h2 = f0g2
            + f1g1_2
            + f2g0
            + f3g9_38
            + f4g8_19
            + f5g7_38
            + f6g6_19
            + f7g5_38
            + f8g4_19
            + f9g3_38;

        let mut h3 =
            f0g3 + f1g2 + f2g1 + f3g0 + f4g9_19 + f5g8_19 + f6g7_19 + f7g6_19 + f8g5_19 + f9g4_19;

        let mut h4 =
            f0g4 + f1g3_2 + f2g2 + f3g1_2 + f4g0 + f5g9_38 + f6g8_19 + f7g7_38 + f8g6_19 + f9g5_38;

        let mut h5 =
            f0g5 + f1g4 + f2g3 + f3g2 + f4g1 + f5g0 + f6g9_19 + f7g8_19 + f8g7_19 + f9g6_19;

        let mut h6 =
            f0g6 + f1g5_2 + f2g4 + f3g3_2 + f4g2 + f5g1_2 + f6g0 + f7g9_38 + f8g8_19 + f9g7_38;

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

        FieldElement([
            h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32, h5 as i32, h6 as i32, h7 as i32,
            h8 as i32, h9 as i32,
        ])
    }
}

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FieldElement(self.0.map(|x| -x))
    }
}
