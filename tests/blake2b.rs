use cryptal::hash::blake2b;

fn blake2b_512_test(input: &[u8]) -> [u8; 64] {
    let got = blake2b(64, input);
    let mut arr = [0u8; 64];
    arr.copy_from_slice(&got);
    arr
}

fn expect_blake2b_512_eq(input: &[u8], expected: &[u8; 64]) {
    let got = blake2b_512_test(input);

    assert_eq!(
        &got, expected,
        "Digest mismatch for input {:?}\nExpected {:02x?}\nGot      {:02x?}",
        input, expected, got,
    );
}

#[test]
fn blake2b_empty_vector() {
    let out = [
        0x78, 0x6a, 0x02, 0xf7, 0x42, 0x01, 0x59, 0x03, 0xc6, 0xc6, 0xfd, 0x85, 0x25, 0x52, 0xd2,
        0x72, 0x91, 0x2f, 0x47, 0x40, 0xe1, 0x58, 0x47, 0x61, 0x8a, 0x86, 0xe2, 0x17, 0xf7, 0x1f,
        0x54, 0x19, 0xd2, 0x5e, 0x10, 0x31, 0xaf, 0xee, 0x58, 0x53, 0x13, 0x89, 0x64, 0x44, 0x93,
        0x4e, 0xb0, 0x4b, 0x90, 0x3a, 0x68, 0x5b, 0x14, 0x48, 0xb7, 0x55, 0xd5, 0x6f, 0x70, 0x1a,
        0xfe, 0x9b, 0xe2, 0xce,
    ];

    expect_blake2b_512_eq(&[], &out);
}

#[test]
fn blake2b_abc_vector() {
    let out = [
        0xba, 0x80, 0xa5, 0x3f, 0x98, 0x1c, 0x4d, 0x0d, 0x6a, 0x27, 0x97, 0xb6, 0x9f, 0x12, 0xf6,
        0xe9, 0x4c, 0x21, 0x2f, 0x14, 0x68, 0x5a, 0xc4, 0xb7, 0x4b, 0x12, 0xbb, 0x6f, 0xdb, 0xff,
        0xa2, 0xd1, 0x7d, 0x87, 0xc5, 0x39, 0x2a, 0xab, 0x79, 0x2d, 0xc2, 0x52, 0xd5, 0xde, 0x45,
        0x33, 0xcc, 0x95, 0x18, 0xd3, 0x8a, 0xa8, 0xdb, 0xf1, 0x92, 0x5a, 0xb9, 0x23, 0x86, 0xed,
        0xd4, 0x00, 0x99, 0x23,
    ];

    expect_blake2b_512_eq(b"abc", &out);
}

#[test]
fn blake2b_known_phrase() {
    let out = blake2b_512_test(b"The quick brown fox jumps over the lazy dog");
    let out2 = blake2b_512_test(b"The quick brown fox jumps over the lazy dog");

    assert_eq!(out, out2);
}

#[test]
fn blake2b_incremental_lengths() {
    let mut buf = Vec::with_capacity(256);
    for i in 0..256 {
        buf.push(i as u8);
        let _ = blake2b_512_test(&buf);
    }
}

#[test]
fn blake2b_zeroes_various_lengths() {
    for len in [1, 2, 4, 8, 16, 32, 64, 128, 255, 256] {
        let buf = vec![0u8; len];
        let _ = blake2b_512_test(&buf);
    }
}

#[test]
fn blake2b_ff_various_lengths() {
    for len in [1, 2, 4, 8, 16, 32, 64, 128, 255, 256] {
        let buf = vec![0xFF; len];
        let _ = blake2b_512_test(&buf);
    }
}

#[test]
fn blake2b_repeated_patterns() {
    let patterns = [
        vec![0x12],
        vec![0xAB, 0xCD],
        vec![0x00, 0x01, 0x02, 0x03],
        vec![0x10, 0x20, 0x30, 0x40, 0x50],
    ];

    for p in patterns {
        let mut buf = Vec::new();
        for _ in 0..200 {
            buf.extend(&p);
        }
        let _ = blake2b_512_test(&buf);
    }
}

#[test]
fn blake2b_large_multiblock() {
    let mut buf = Vec::new();
    for i in 0..5000 {
        buf.push((i % 256) as u8);
    }
    let _ = blake2b_512_test(&buf);
}

#[test]
fn blake2b_1mb_data() {
    let buf = vec![0xAAu8; 1_000_000];
    let _ = blake2b_512_test(&buf);
}

fn lcg(seed: &mut u64) -> u8 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    ((*seed >> 24) & 0xFF) as u8
}

#[test]
fn blake2b_fuzz_semi_random() {
    let mut seed = 0x123456789ABCDEF0u64;
    let mut buf = Vec::new();

    for _ in 0..500 {
        buf.push(lcg(&mut seed));
        let _ = blake2b_512_test(&buf);
    }
}

#[test]
fn blake2b_single_bytes() {
    for b in 0u8..=255 {
        let _ = blake2b_512_test(&[b]);
    }
}

#[test]
fn blake2b_block_boundary_128() {
    let buf = vec![0x11u8; 128];
    let _ = blake2b_512_test(&buf);
}

#[test]
fn blake2b_large_boundary_10k() {
    let buf = vec![0x55u8; 10_000];
    let _ = blake2b_512_test(&buf);
}
