use criterion::{Criterion, criterion_group, criterion_main};
use sha2::{Digest, Sha256};
use std::hint::black_box;

pub fn bench_sha2_ref(c: &mut Criterion) {
    let mut g = c.benchmark_group("sha256_ref");

    g.sample_size(1);
    g.warm_up_time(std::time::Duration::ZERO);
    g.measurement_time(std::time::Duration::from_millis(50));

    g.bench_function("sha256_ref", |b| {
        b.iter(|| {
            let mut hasher = Sha256::new();
            hasher.update(black_box(&[0u8; 64]));
            let _ = hasher.finalize();
        })
    });

    g.finish();
}

criterion_group!(benches, bench_sha2_ref);
criterion_main!(benches);
