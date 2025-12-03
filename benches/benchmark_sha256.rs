use cryptography::hash::sha256::H256_INIT;
use cryptography::hash::sha256::computations::{
    all_rounds, big_sigma0, big_sigma1, ch, expand_w, maj, small_sigma0, small_sigma1,
};
use cryptography::hash::sha256::core::{compress, sha256};

use criterion::{Criterion, criterion_group, criterion_main};
use sha2::{Digest, Sha256};
use std::hint::black_box;

#[inline(never)]
fn bench_small_sigma0_wrapper(x: u32) -> u32 {
    small_sigma0(x)
}

#[inline(never)]
fn bench_small_sigma1_wrapper(x: u32) -> u32 {
    small_sigma1(x)
}

#[inline(never)]
fn bench_big_sigma0_wrapper(x: u32) -> u32 {
    big_sigma0(x)
}

#[inline(never)]
fn bench_big_sigma1_wrapper(x: u32) -> u32 {
    big_sigma1(x)
}

#[inline(never)]
fn bench_ch_wrapper(e: u32, f: u32, g: u32) -> u32 {
    ch(e, f, g)
}

#[inline(never)]
fn bench_maj_wrapper(a: u32, b: u32, c: u32) -> u32 {
    maj(a, b, c)
}

#[inline(never)]
fn bench_expand_w_wrapper(w: &mut [u32; 64]) {
    expand_w(w)
}

#[inline(never)]
fn bench_all_rounds_wrapper(state: [u32; 8], w: &[u32; 64]) -> [u32; 8] {
    all_rounds(state, w)
}

#[inline(never)]
fn bench_compress_wrapper(block: &[u8; 64], state: &mut [u32; 8]) {
    compress(block, state)
}

pub fn bench_sigmas(c: &mut Criterion) {
    c.bench_function("small_sigma0", |b| {
        b.iter(|| bench_small_sigma0_wrapper(black_box(0x12345678)))
    });

    c.bench_function("small_sigma1", |b| {
        b.iter(|| bench_small_sigma1_wrapper(black_box(0xabcdef01)))
    });

    c.bench_function("big_sigma0", |b| {
        b.iter(|| bench_big_sigma0_wrapper(black_box(0x99887766)))
    });

    c.bench_function("big_sigma1", |b| {
        b.iter(|| bench_big_sigma1_wrapper(black_box(0xdeadbeef)))
    });
}

pub fn bench_logic(c: &mut Criterion) {
    c.bench_function("ch", |b| {
        b.iter(|| {
            bench_ch_wrapper(
                black_box(0x11111111),
                black_box(0x22222222),
                black_box(0x33333333),
            )
        })
    });

    c.bench_function("maj", |b| {
        b.iter(|| {
            bench_maj_wrapper(
                black_box(0x44444444),
                black_box(0x55555555),
                black_box(0x66666666),
            )
        })
    });
}

pub fn bench_expand(c: &mut Criterion) {
    c.bench_function("expand_w", |b| {
        b.iter(|| {
            let mut w = [0u32; 64];
            bench_expand_w_wrapper(black_box(&mut w));
        })
    });
}

pub fn bench_rounds(c: &mut Criterion) {
    c.bench_function("all_rounds", |b| {
        b.iter(|| {
            let mut w = [0u32; 64];
            expand_w(&mut w);
            bench_all_rounds_wrapper(black_box(H256_INIT), black_box(&w))
        })
    });
}

pub fn bench_compress_fn(c: &mut Criterion) {
    c.bench_function("compress", |b| {
        b.iter(|| {
            let mut state = H256_INIT;
            let block = [0u8; 64];
            bench_compress_wrapper(black_box(&block), black_box(&mut state))
        })
    });
}

pub fn bench_sha256(c: &mut Criterion) {
    c.bench_function("sha256 64 bytes", |b| {
        b.iter(|| sha256(black_box(&[0u8; 64])))
    });
}

pub fn bench_sha2_crate(c: &mut Criterion) {
    let data = [0u8; 64];

    c.bench_function("sha2::Sha256 64 bytes", |b| {
        b.iter(|| {
            let mut hasher = Sha256::new();
            hasher.update(black_box(&data));
            let _ = hasher.finalize();
        })
    });
}

criterion_group!(
    benches,
    bench_sigmas,
    bench_logic,
    bench_expand,
    bench_rounds,
    bench_compress_fn,
    bench_sha256,
    bench_sha2_crate
);
criterion_main!(benches);
