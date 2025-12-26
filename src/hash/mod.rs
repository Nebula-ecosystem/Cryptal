/// Hash algorithms exposed by the crate.
///
/// Ce module expose les fonctions de hachage cryptographiques implémentées en Rust pur.
/// Actuellement, seul SHA-256 est fourni.
///
/// # Exemple
///
/// ```
/// use cryptography::hash::sha256;
/// let hash = sha256(b"hello world");
/// ```
pub mod sha256;

pub use sha256::core::sha256;
