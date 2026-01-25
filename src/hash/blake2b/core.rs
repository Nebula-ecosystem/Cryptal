use super::consts::{BLAKE2B_BLOCK_BYTES, BLAKE2B_OUT_MAX, IV, ROUNDS, SIGMA};
use super::utils::{g, load_u64_le, store_u64_le};

/// Blake2b hashing state.
///
/// This structure implements the Blake2b cryptographic hash function as
/// specified in RFC 7693, using a block-based compression function with
/// optional keyed initialization.
///
/// The implementation supports incremental updates and produces the
/// full Blake2b output on finalization.
#[derive(Clone)]
pub(crate) struct Blake2b {
    /// Internal chaining value (hash state).
    h: [u64; 8],

    /// Total number of bytes processed so far.
    ///
    /// This counter is encoded into the compression function to ensure
    /// domain separation between blocks.
    t: u128,

    /// Message buffer for partial blocks.
    buf: [u8; BLAKE2B_BLOCK_BYTES],

    /// Number of bytes currently stored in `buf`.
    buflen: usize,
}

impl Blake2b {
    /// Creates a new Blake2b hashing state.
    ///
    /// `out_len` specifies the desired output length in bytes (up to
    /// `BLAKE2B_OUT_MAX`).
    /// `key` optionally enables keyed hashing (MAC mode).
    ///
    /// When a key is provided, it is processed as the first full block
    /// according to the Blake2b specification.
    pub(crate) fn new(out_len: usize, key: &[u8]) -> Self {
        assert!(out_len <= BLAKE2B_OUT_MAX);

        let mut h = IV;
        let param = 0x0101_0000u64 ^ ((key.len() as u64) << 8) ^ (out_len as u64);
        h[0] ^= param;

        let mut st = Self {
            h,
            t: 0,
            buf: [0u8; BLAKE2B_BLOCK_BYTES],
            buflen: 0,
        };

        if !key.is_empty() {
            st.buf[..key.len()].copy_from_slice(key);
            st.buflen = BLAKE2B_BLOCK_BYTES;
            st.compress(false);
            st.buf = [0u8; BLAKE2B_BLOCK_BYTES];
            st.buflen = 0;
        }

        st
    }

    /// Updates the hash state with additional input data.
    ///
    /// This method may be called multiple times with arbitrary input sizes.
    /// Full blocks are compressed immediately, while partial blocks are
    /// buffered until enough data is available.
    pub(crate) fn update(&mut self, mut input: &[u8]) {
        if input.is_empty() {
            return;
        }

        if self.buflen > 0 {
            let want = BLAKE2B_BLOCK_BYTES - self.buflen;
            let take = want.min(input.len());
            self.buf[self.buflen..self.buflen + take].copy_from_slice(&input[..take]);
            self.buflen += take;
            input = &input[take..];

            if self.buflen == BLAKE2B_BLOCK_BYTES {
                self.compress(false);
                self.buflen = 0;
            }
        }

        while input.len() >= BLAKE2B_BLOCK_BYTES {
            self.buf.copy_from_slice(&input[..BLAKE2B_BLOCK_BYTES]);
            self.buflen = BLAKE2B_BLOCK_BYTES;
            self.compress(false);
            self.buflen = 0;
            input = &input[BLAKE2B_BLOCK_BYTES..];
        }

        if !input.is_empty() {
            self.buf[..input.len()].copy_from_slice(input);
            self.buflen = input.len();
        }
    }

    /// Finalizes the hash computation and returns the digest.
    ///
    /// This consumes the hashing state and produces the full Blake2b
    /// output (`BLAKE2B_OUT_MAX` bytes).
    pub(crate) fn finalize(mut self) -> [u8; BLAKE2B_OUT_MAX] {
        for b in self.buf[self.buflen..].iter_mut() {
            *b = 0;
        }
        self.compress(true);

        let mut out = [0u8; BLAKE2B_OUT_MAX];
        for (i, word) in self.h.iter().enumerate() {
            store_u64_le(&mut out[i * 8..i * 8 + 8], *word);
        }
        out
    }

    /// Compresses the current message block into the chaining value.
    ///
    /// If `is_last` is true, the finalization flag is set, marking this
    /// block as the last block of the message.
    fn compress(&mut self, is_last: bool) {
        self.t = self.t.wrapping_add(self.buflen as u128);

        let mut m = [0u64; 16];
        for (i, chunk) in self.buf.chunks_exact(8).enumerate().take(16) {
            m[i] = load_u64_le(chunk);
        }

        let mut v = [0u64; 16];
        v[..8].copy_from_slice(&self.h);
        v[8..].copy_from_slice(&IV);

        let t0 = self.t as u64;
        let t1 = (self.t >> 64) as u64;
        v[12] ^= t0;
        v[13] ^= t1;

        if is_last {
            v[14] ^= u64::MAX;
        }

        for s in SIGMA.iter().take(ROUNDS) {
            for i in 0..4 {
                g(
                    &mut v,
                    i,
                    i + 4,
                    i + 8,
                    i + 12,
                    m[s[2 * i]],
                    m[s[2 * i + 1]],
                );
            }

            for i in 0..4 {
                g(
                    &mut v,
                    i,
                    (i + 1) % 4 + 4,
                    (i + 2) % 4 + 8,
                    (i + 3) % 4 + 12,
                    m[s[8 + 2 * i]],
                    m[s[8 + 2 * i + 1]],
                );
            }
        }

        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }

        self.buflen = 0;
    }
}
