//! Blake2b hash function implementation.
//!
//! This module provides a low-level, self-contained implementation of the
//! Blake2b cryptographic hash function as specified in RFC 7693.
//!
//! The implementation is split into well-defined submodules:
//! - `consts`: algorithm constants (IV, permutation schedule, block size)
//! - `core`: stateful Blake2b compression and incremental hashing logic
//! - `hash`: one-shot and extendable-output (XOF) convenience functions
//! - `utils`: internal helpers used by the compression function
//!
//! All components are designed to be deterministic, portable, and free of
//! external dependencies. This module exposes only the necessary primitives
//! for higher-level constructions and does not provide streaming I/O APIs.

pub(crate) mod consts;
mod core;
pub(crate) mod hash;
pub(crate) mod utils;
