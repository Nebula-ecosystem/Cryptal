/// Lightweight cryptography primitives and hash functions.
///
/// Cette crate fournit des primitives cryptographiques minimalistes pour `no_std`,
/// dont une implémentation SHA-256 pure Rust et un entier 256 bits (`U256`).
///
/// # Features
///
/// - SHA-256 hash function
/// - 256-bit unsigned integer (`U256`) with bitwise and shift operations
///
/// # Exemple
///
/// ```
/// use cryptography::hash::sha256;
/// let hash = sha256(b"hello world");
/// ```
pub mod hash;
pub mod primitives;

pub use primitives::U256;
