const BLAKE2B_BLOCK_BYTES: usize = 128;
const BLAKE2B_OUT_MAX: usize = 64;
const ROUNDS: usize = 12;

const IV: [u64; 8] = [
    0x6A09_E667_F3BC_C908,
    0xBB67_AE85_84CA_A73B,
    0x3C6E_F372_FE94_F82B,
    0xA54F_F53A_5F1D_36F1,
    0x510E_527F_ADE6_82D1,
    0x9B05_688C_2B3E_6C1F,
    0x1F83_D9AB_FB41_BD6B,
    0x5BE0_CD19_137E_2179,
];

const SIGMA: [[usize; 16]; 12] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
];

#[inline(always)]
fn load_u64_le(b: &[u8]) -> u64 {
    u64::from_le_bytes(b.try_into().unwrap())
}

#[inline(always)]
fn store_u64_le(out: &mut [u8], v: u64) {
    out.copy_from_slice(&v.to_le_bytes());
}

#[derive(Clone)]
struct Blake2b {
    h: [u64; 8],
    t: u128,
    buf: [u8; BLAKE2B_BLOCK_BYTES],
    buflen: usize,
}

impl Blake2b {
    fn new(out_len: usize, key: &[u8]) -> Self {
        assert!((1..=BLAKE2B_OUT_MAX).contains(&out_len));
        assert!(key.len() <= BLAKE2B_OUT_MAX);

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

    fn update(&mut self, mut input: &[u8]) {
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

    fn finalize(mut self) -> [u8; BLAKE2B_OUT_MAX] {
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
            g(&mut v, 0, 4, 8, 12, m[s[0]], m[s[1]]);
            g(&mut v, 1, 5, 9, 13, m[s[2]], m[s[3]]);
            g(&mut v, 2, 6, 10, 14, m[s[4]], m[s[5]]);
            g(&mut v, 3, 7, 11, 15, m[s[6]], m[s[7]]);

            g(&mut v, 0, 5, 10, 15, m[s[8]], m[s[9]]);
            g(&mut v, 1, 6, 11, 12, m[s[10]], m[s[11]]);
            g(&mut v, 2, 7, 8, 13, m[s[12]], m[s[13]]);
            g(&mut v, 3, 4, 9, 14, m[s[14]], m[s[15]]);
        }

        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }

        self.buflen = 0;
    }
}

#[inline(always)]
fn g(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
    v[a] = blamka(v[a], v[b]).wrapping_add(x);
    v[d] = (v[d] ^ v[a]).rotate_right(32);

    v[c] = blamka(v[c], v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(24);

    v[a] = blamka(v[a], v[b]).wrapping_add(y);
    v[d] = (v[d] ^ v[a]).rotate_right(16);

    v[c] = blamka(v[c], v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(63);
}

#[inline(always)]
fn blamka(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}

pub(crate) fn blake2b(out_len: usize, input: &[u8]) -> [u8; BLAKE2B_OUT_MAX] {
    let mut h = Blake2b::new(out_len, &[]);
    h.update(input);
    h.finalize()
}

pub(crate) fn blake2b_512(input: &[u8]) -> [u8; 64] {
    let full = blake2b(64, input);
    full[..64].try_into().unwrap()
}

pub(crate) fn blake2b_long(out_len: usize, input: &[u8]) -> Vec<u8> {
    assert!(out_len >= 4);

    let t_le = (out_len as u32).to_le_bytes();

    if out_len <= 64 {
        let mut buf = Vec::with_capacity(4 + input.len());
        buf.extend_from_slice(&t_le);
        buf.extend_from_slice(input);
        let out = blake2b(out_len, &buf);
        return out[..out_len].to_vec();
    }

    let r = out_len.div_ceil(32).saturating_sub(2);

    let mut x = Vec::with_capacity(4 + input.len());
    x.extend_from_slice(&t_le);
    x.extend_from_slice(input);

    let mut v_prev = blake2b_512(&x).to_vec();
    let mut out = Vec::with_capacity(out_len);

    for _ in 0..r {
        out.extend_from_slice(&v_prev[..32]);
        v_prev = blake2b_512(&v_prev).to_vec();
    }

    let last_len = out_len - 32 * r;
    let v_last_full = blake2b(last_len, &v_prev);
    out.extend_from_slice(&v_last_full[..last_len]);

    out
}
