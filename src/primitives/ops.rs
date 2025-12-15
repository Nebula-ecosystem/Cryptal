//! Bitwise and shift operations for `U256`.
//!
//! Implements XOR/AND plus logical shifts.

use super::U256;
use std::ops::{Add, BitAnd, BitXor, Div, Mul, Shl, Shr, Sub};

impl BitXor<U256> for U256 {
    type Output = U256;

    fn bitxor(self, rhs: U256) -> Self::Output {
        let mut out = [0u8; 32];

        for (o, (l, r)) in out.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
            *o = l ^ r;
        }

        U256(out)
    }
}

impl BitAnd<U256> for U256 {
    type Output = U256;
    fn bitand(self, rhs: U256) -> Self::Output {
        let mut out = [0u8; 32];

        for (o, (l, r)) in out.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
            *o = l & r;
        }

        U256(out)
    }
}

impl Shl<U256> for U256 {
    type Output = U256;

    fn shl(self, rhs: U256) -> Self::Output {
        let shift = (((rhs.0[30] as u32) << 8) | rhs.0[31] as u32) as usize;

        if shift == 0 {
            return self;
        }
        if shift >= 256 {
            return U256([0; 32]);
        }

        let byte_shift = shift >> 3;
        let bit_shift = (shift & 7) as u8;

        let mut out = [0u8; 32];
        for (i, o) in out.iter_mut().enumerate() {
            let src = i + byte_shift;
            *o = if src < 32 { self.0[src] } else { 0 };
        }

        if bit_shift != 0 {
            let carry_bits = 8 - bit_shift;

            for i in 0..32 {
                let hi = out[i] << bit_shift;
                let c = if i > 0 { out[i - 1] >> carry_bits } else { 0 };

                out[i] = hi | c;
            }
        }

        U256(out)
    }
}

impl Shr<U256> for U256 {
    type Output = U256;

    fn shr(self, rhs: U256) -> Self::Output {
        let shift = (((rhs.0[30] as u32) << 8) | rhs.0[31] as u32) as usize;

        if shift == 0 {
            return self;
        }
        if shift >= 256 {
            return U256([0; 32]);
        }

        let byte_shift = shift >> 3;
        let bit_shift = (shift & 7) as u8;

        let mut out = [0u8; 32];
        for (i, o) in out.iter_mut().enumerate() {
            *o = if i >= byte_shift {
                self.0[i - byte_shift]
            } else {
                0
            };
        }

        if bit_shift != 0 {
            let carry_bits = 8 - bit_shift;
            let prev = out;

            for (i, o) in out.iter_mut().enumerate() {
                let lo = prev[i] >> bit_shift;

                let c = if i + 1 < 32 {
                    prev[i + 1] << carry_bits
                } else {
                    0
                };

                *o = lo | c;
            }
        }

        U256(out)
    }
}

impl Add for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> Self::Output {
        let mut out = [0u8; 32];
        let mut carry = 0u16;

        for i in (0..32).rev() {
            let s = self.0[i] as u16 + rhs.0[i] as u16 + carry;

            out[i] = (s & 0xFF) as u8;

            carry = s >> 8;
        }

        U256(out)
    }
}

impl Sub for U256 {
    type Output = U256;

    fn sub(self, rhs: U256) -> Self::Output {
        let mut out = [0u8; 32];
        let mut borrow = 0i16;

        for i in (0..32).rev() {
            let lhs = self.0[i] as i16;
            let s = rhs.0[i] as i16 + borrow;

            if lhs >= s {
                out[i] = (lhs - s) as u8;
                borrow = 0;
            } else {
                out[i] = (lhs + 256 - s) as u8;
                borrow = 1;
            }
        }

        U256(out)
    }
}

impl Mul<U256> for U256 {
    type Output = U256;

    fn mul(self, rhs: U256) -> Self::Output {
        // Work in little-endian 64-bit limbs to make carry propagation straightforward.
        let lhs_be: [u64; 4] = self.into();
        let rhs_be: [u64; 4] = rhs.into();

        let mut lhs = [0u64; 4];
        let mut rhs = [0u64; 4];

        for i in 0..4 {
            lhs[i] = lhs_be[3 - i];
            rhs[i] = rhs_be[3 - i];
        }

        let mut acc = [0u128; 8];

        for i in 0..4 {
            for j in 0..4 {
                acc[i + j] += lhs[i] as u128 * rhs[j] as u128;
            }
        }

        for k in 0..7 {
            let carry = acc[k] >> 64;
            acc[k] &= 0xFFFF_FFFF_FFFF_FFFF;
            acc[k + 1] += carry;
        }

        let mut out_le = [0u64; 4];
        for i in 0..4 {
            out_le[i] = acc[i] as u64;
        }

        let mut out_be = [0u64; 4];
        for i in 0..4 {
            out_be[i] = out_le[3 - i];
        }

        U256::from(out_be)
    }
}

impl Div<U256> for U256 {
    type Output = U256;

    /// Long division using shift-and-subtract (binary long division).
    ///
    /// Processes the dividend bit-by-bit from most significant to least significant.
    /// For each bit, shifts the remainder left by one, brings down the next bit from
    /// the dividend, and subtracts the divisor if the remainder is large enough.
    fn div(self, rhs: U256) -> Self::Output {
        assert!(rhs != U256::ZERO, "division by zero");

        if self < rhs {
            return U256::ZERO;
        }

        let mut quotient = [0u8; 32];
        let mut remainder = U256::ZERO;

        for bit in 0..256 {
            let byte_idx = bit >> 3;
            let bit_in_byte = 7 - (bit & 7);

            let incoming = (self.0[byte_idx] >> bit_in_byte) & 1;

            remainder = remainder << U256::from(1u8);

            let mut rem_bytes: [u8; 32] = remainder.into();
            rem_bytes[31] = (rem_bytes[31] & 0xFE) | incoming;
            remainder = U256(rem_bytes);

            if remainder >= rhs {
                remainder = remainder - rhs;
                quotient[byte_idx] |= 1 << bit_in_byte;
            }
        }

        U256(quotient)
    }
}
