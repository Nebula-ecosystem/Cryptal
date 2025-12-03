use cryptography::hash::sha256::core::sha256;

use criterion::{Criterion, criterion_group, criterion_main};
use sha2::{Digest, Sha256};
use std::hint::black_box;

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

criterion_group!(benches, bench_sha256, bench_sha2_crate);
criterion_main!(benches);
