//! Ed25519 cryptographic implementation.
//!
//! This module provides a complete, self-contained implementation of the
//! Ed25519 signature scheme and related primitives.
//!
//! The design follows a clear separation of concerns:
//! - **high-level API** exposed via the `core` module
//! - **low-level arithmetic** isolated in dedicated internal modules
//!
//! All cryptographic operations are implemented with constant-time
//! considerations in mind and closely follow the Ed25519 reference algorithms.

/// High-level Ed25519 API.
///
/// This module exposes the public-facing interface:
/// - key generation
/// - signing and verification
/// - scalar key tweaking
/// - Diffie–Hellman style key exchange
///
/// This is the only module that most users should interact with directly.
pub(crate) mod core;

/// Constant-time utilities.
///
/// This module contains helpers and traits for constant-time comparisons
/// and bit-level operations used throughout the implementation.
///
/// It prevents timing side-channel leaks in comparisons and conditional logic.
pub(crate) mod ct;

/// Finite field arithmetic.
///
/// Implements arithmetic over the prime field GF(2²⁵⁵ − 19) using a limb-based
/// representation.
///
/// This module provides:
/// - addition, subtraction, multiplication
/// - squaring and repeated squaring
/// - inversion and exponentiation
///
/// All operations are optimized to match the behavior of the Ed25519
/// reference implementation.
pub(crate) mod field;

/// Edwards curve group operations.
///
/// Implements point representations and operations on the Edwards curve
/// used by Ed25519.
///
/// Includes:
/// - point decompression
/// - scalar multiplication
/// - point addition and doubling
/// - internal cached representations for efficiency
pub(crate) mod group;

/// Scalar arithmetic.
///
/// Implements arithmetic modulo the Ed25519 group order ℓ.
///
/// Used for:
/// - private key handling
/// - signature computations
/// - scalar reduction and combination
pub(crate) mod scalar;

/// Precomputed tables.
///
/// Contains precomputed constants and tables used to accelerate
/// scalar multiplication and group operations.
///
/// These values are static and derived from the Ed25519 specification.
pub(crate) mod table;

// Re-export the public API at the `ed25519` level
pub use core::*;
