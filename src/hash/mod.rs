//! Hashing primitives
//!
//! This module groups low-level cryptographic hash functions used throughout
//! the Nebula ecosystem.
//!
//! The primitives provided here are designed to be:
//! - cryptographically secure
//! - deterministic and portable
//! - free of external dependencies
//! - suitable for identifiers, integrity checks, and higher-level constructions
//!
//! This module intentionally exposes **hashing primitives only**.
//! It does not provide streaming, incremental, or stateful APIs.
//! More complex constructions (KDFs, MACs, password hashing, etc.) are
//! expected to be built on top of these functions.
//!
//! Currently supported primitives:
//! - SHA-256
//! - SHA-512
//! - Blake2b (fixed-length and extendable-output variants)
//!
//! Blake2b is provided both as a standard cryptographic hash function
//! (up to 512-bit output) and as an extendable-output function (XOF),
//! suitable for internal expansion, key material generation, and
//! protocol-level use.
//!
//! The module is structured to allow additional hash functions to be added
//! in the future without breaking existing users.

mod blake2b;
mod sha256;
mod sha512;

/// Computes the SHA-256 hash of the given input.
///
/// This is one of the primary hashing entry points exposed by this module.
pub use sha256::core::sha256;

/// Computes the SHA-512 hash of the given input.
///
/// This function is suitable for applications requiring a wider hash
/// output or higher collision resistance.
pub use sha512::core::sha512;

/// Computes a Blake2b hash with a configurable output length (up to 64 bytes).
///
/// This is the standard Blake2b hash function as defined in RFC 7693.
pub use blake2b::hash::blake2b;

/// Computes a Blake2b-based extendable-output hash (XOF).
///
/// This function expands an input into an arbitrary-length output using
/// chained Blake2b-512 invocations. It is a low-level primitive commonly
/// used in higher-level constructions (e.g. Argon2 initialization).
pub use blake2b::hash::blake2b_long;
