//! Digital signature schemes.
//!
//! This module groups implementations of digital signature algorithms
//! built on top of the crate’s cryptographic primitives and hash
//! functions.
//!
//! Each submodule corresponds to a specific signature scheme and is
//! responsible for its own key types, signing logic, and verification
//! rules. Implementations are intentionally explicit and self-contained,
//! favoring clarity, auditability, and specification-level correctness
//! over abstraction.
//!
//! ## Ed25519
//!
//! The `ed25519` module is a **Rust reimplementation** inspired by the
//! well-known reference implementation by Orson Peters:
//!
//! <https://github.com/orlp/ed25519>
//!
//! While rewritten from scratch in Rust, this implementation closely
//! follows the original design and mathematical structure, including:
//! - limb-based field arithmetic
//! - explicit carry handling
//! - constant-time operations
//! - faithful adherence to the Ed25519 specification
//!
//! Any deviations from the original implementation are intentional and
//! result from adapting the code to Rust’s type system, ownership model,
//! and safety guarantees.

pub mod ed25519;
