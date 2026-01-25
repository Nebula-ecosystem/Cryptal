//! Cryptographic utilities and primitives for Nebula
//!
//! This crate provides low-level cryptographic building blocks used
//! throughout the Nebula ecosystem.
//!
//! The focus is on **clarity, predictability, and auditability**, rather
//! than on providing a large or high-level cryptographic API. All components
//! are designed to be dependency-free, explicit in their semantics, and
//! suitable for security-critical code.
//!
//! # Module overview
//!
//! - `utils`  
//!   Low-level, non-cryptographic utilities used by the rest of the crate.
//!   This module contains environment-facing helpers, byte-level utilities,
//!   and other foundational components required to support cryptographic
//!   code without polluting its APIs.
//!
//! - `hash`  
//!   Cryptographic hash functions and related utilities (e.g. SHA-256,
//!   SHA-512).
//!
//!   These hashes are **fast, deterministic, and CPU-bound**, and are intended
//!   for integrity checking, identifiers, protocol-level constructions, and
//!   internal hashing. They are **not** designed to protect low-entropy,
//!   human-provided secrets.
//!
//! - `primitives`  
//!   Fixed-size, low-level cryptographic primitives such as `U256` and
//!   `U512`. These types provide explicit, predictable semantics and are
//!   used as fundamental building blocks across the crate.
//!
//! - `rng`  
//!   Cryptographically secure pseudorandom number generators.
//!
//!   This module provides secure randomness expansion based on internal
//!   primitives. It is used to generate keys, seeds, nonces, and other
//!   high-entropy values. All secrets generated here are assumed to be
//!   **machine-generated**, not human-derived.
//!
//! - `keys`  
//!   Cryptographic key types and key-related operations.
//!
//!   This module defines algorithm-specific key representations (such as
//!   Ed25519 and X25519 keys), along with their derivation, serialization,
//!   and safe transformations. It provides a clear separation between
//!   **key material** and the cryptographic algorithms that operate on it
//!   (signatures, key exchange, etc.).
//!
//!   No signing, verification, or protocol logic lives here—only key
//!   structure and manipulation.
//!
//! - `derivation`  
//!   Key derivation functions (KDFs).
//!
//!   This module contains algorithms designed to derive **strong,
//!   cryptographic-quality keys** from inputs that are either low-entropy
//!   (e.g. human passwords or passphrases) or require controlled,
//!   parameterized transformation.
//!
//!   Unlike the `hash` module, derivation functions are intentionally
//!   **slow and resource-intensive**, making them suitable for protecting
//!   secrets against brute-force attacks.
//!
//!   The primary use case is **Argon2id**, which is used to:
//!   - derive strong keys from human-provided secrets
//!   - unlock encrypted key material locally
//!   - reconstruct identities across multiple devices without storing keys
//!   - optionally impose a one-time computational cost (e.g. anti-Sybil)
//!
//!   Derivation functions are **never used in network hot paths** and are
//!   strictly local to the machine performing the derivation.
//!
//! - `recovery`  
//!   Cryptographic recovery and survivability mechanisms.
//!
//!   This module contains primitives designed to protect, distribute, and
//!   recover sensitive material in the presence of partial data loss or
//!   compromise. It currently provides an implementation of
//!   **Shamir Secret Sharing (SSS)**, allowing a secret to be split into
//!   multiple shares such that only a configurable threshold of shares is
//!   required for reconstruction.
//!
//!   The recovery module is purely cryptographic: it does not perform any
//!   storage, networking, or policy decisions. Those concerns are handled
//!   at higher layers of the Nebula stack.
//!
//! # Design goals
//!
//! - Clear separation between hashing, key derivation, and key material
//! - No heap allocations in core primitives
//! - Minimal and explicit APIs
//! - Stable, well-defined semantics
//! - Cryptographic intent encoded in module structure
//!
//! This crate is not intended to replace full-featured, externally audited
//! cryptographic libraries, but to serve as a small, controlled foundation
//! for Nebula's internal cryptographic needs.

mod utils;

pub mod derivation;
pub mod encryption;
pub mod hash;
pub mod keys;
pub mod primitives;
pub mod recovery;
pub mod rng;
