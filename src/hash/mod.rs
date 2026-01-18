//! Hashing primitives
//!
//! This module groups cryptographic hash functions used throughout the
//! Nebula ecosystem.
//!
//! The hash functions provided here are designed to be:
//! - cryptographically secure
//! - deterministic and portable
//! - free of external dependencies
//! - suitable for identifiers, integrity checks, and higher-level protocols
//!
//! These implementations are low-level primitives intended for internal
//! use within Nebula. They do not provide streaming or incremental APIs,
//! and higher-level constructions should be built on top of them.
//!
//! Currently supported hash functions:
//! - SHA-256
//! - SHA-512
//!
//! The module is structured to allow additional hash functions to be added
//! in the future without breaking existing users.

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
