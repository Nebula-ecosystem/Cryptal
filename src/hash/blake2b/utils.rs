/// Blake2b mixing function `G`.
///
/// This function is the core non-linear transformation used by the Blake2b
/// compression function. It operates on four 64-bit words of the working
/// vector `v` and mixes them using modular additions, XORs, and fixed
/// rotations.
///
/// The parameters `a`, `b`, `c`, and `d` are indices into the state vector,
/// while `x` and `y` are message words selected according to the `SIGMA`
/// permutation schedule.
///
/// This implementation follows the Blake2b specification exactly and is
/// designed to be inlined for performance.
#[inline(always)]
pub(crate) fn g(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
    v[d] = (v[d] ^ v[a]).rotate_right(32);

    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(24);

    v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
    v[d] = (v[d] ^ v[a]).rotate_right(16);

    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(63);
}

/// Loads a 64-bit unsigned integer from a little-endian byte slice.
///
/// The input slice must be exactly 8 bytes long. This function is used to
/// parse message blocks into 64-bit words as required by the Blake2b
/// compression function.
#[inline(always)]
pub(crate) fn load_u64_le(b: &[u8]) -> u64 {
    u64::from_le_bytes(b.try_into().unwrap())
}

/// Stores a 64-bit unsigned integer into a byte slice in little-endian order.
///
/// The output slice must be exactly 8 bytes long. This function is used to
/// serialize the internal state into the final hash output.
#[inline(always)]
pub(crate) fn store_u64_le(out: &mut [u8], v: u64) {
    out.copy_from_slice(&v.to_le_bytes());
}
