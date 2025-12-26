/// 256-bit unsigned integer type for cryptographic operations.
///
/// This module defines the [`U256`] type, une structure 256 bits (32 octets) non signée
/// avec opérations de base (bitwise, décalages, conversions). Utilisée comme brique de base
/// pour les primitives cryptographiques et fonctions de hachage.
///
/// # Exemple
///
/// ```
/// use cryptography::primitives::U256;
/// let zero = U256::ZERO;
/// let one = U256::ONE;
/// assert!(zero < one);
/// ```
use std::fmt::{Display, Formatter, Result};

/// 256-bit unsigned integer stored as 32-byte big-endian.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct U256(pub [u8; 32]);

impl U256 {
    /// The value zero.
    pub const ZERO: Self = Self([0u8; 32]);

    /// The value one.
    pub const ONE: Self = Self::one_be();

    /// The maximum value.
    pub const MAX: Self = Self([255u8; 32]);

    /// Returns the value one as big-endian.
    pub const fn one_be() -> Self {
        let mut out = [0u8; 32];
        out[31] = 1;
        U256(out)
    }

    /// Counts the number of leading zero bits in the big-endian representation.
    ///
    /// # Example
    ///
    /// ```
    /// use cryptography::primitives::U256;
    /// // Only the 4th byte is non-zero (0x01), so 3*8 + 7 = 31 leading zeros
    /// let mut arr = [0u8; 32];
    /// arr[3] = 1;
    /// let value = U256(arr);
    /// assert_eq!(value.leading_zeros(), 31);
    /// ```
    pub fn leading_zeros(&self) -> u32 {
        let mut count = 0u32;

        for &byte in self.0.iter() {
            if byte == 0 {
                count += 8;
            } else {
                count += byte.leading_zeros();

                return count;
            }
        }

        count
    }
}

impl Display for U256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (i, byte) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(":")?;
            }

            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}
