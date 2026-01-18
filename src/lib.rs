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
//! - `os`  
//!   Operating system integration for cryptography-related functionality.
//!   This module provides access to system entropy sources and other
//!   OS-backed primitives required to bootstrap secure randomness.
//!
//! - `hash`  
//!   Cryptographic hash functions and related utilities (e.g. SHA-256,
//!   SHA-512). These implementations are intended for internal use and
//!   protocol-level constructions.
//!
//! - `primitives`  
//!   Fixed-size, low-level cryptographic primitives such as `U256` and
//!   `U512`. These types provide explicit, predictable semantics and are
//!   used as fundamental building blocks across the crate.
//!
//! - `rng`  
//!   Cryptographically secure pseudorandom number generators built from
//!   internal primitives (e.g. ChaCha20-based CSPRNG). These generators
//!   rely on the `os` module for initial entropy and provide deterministic,
//!   auditable randomness expansion.
//!
//! # Design goals
//!
//! - No heap allocations in core primitives
//! - Minimal and explicit APIs
//! - Stable, well-defined semantics
//! - Clear separation between primitives and higher-level constructions
//!
//! This crate is not intended to replace full-featured, externally audited
//! cryptographic libraries, but to serve as a small, controlled foundation
//! for Nebula's internal cryptographic needs.

mod os;

pub mod hash;
pub mod primitives;
pub mod rng;
