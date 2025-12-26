/// Primitives for cryptographic operations.
///
/// This module expose des entiers fixes et utilitaires pour les algorithmes cryptographiques.
/// Le type principal est [`U256`], un entier non signé 256 bits.
///
/// Les helpers de conversion sont dans [`conv`], les opérateurs dans [`ops`].
pub mod conv;
pub mod ops;
pub mod u256;

pub use u256::U256;
