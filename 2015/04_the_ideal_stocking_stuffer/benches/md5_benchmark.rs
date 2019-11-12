use criterion::{black_box, criterion_group, criterion_main, Criterion};
use md5lib::find_smallest_number;

pub fn criterion_benchmark(_c: &mut Criterion) {
    let mut c_prime = Criterion::default().sample_size(10);
    c_prime.bench_function("Smallest with 00000", |b| b.iter(|| find_smallest_number(black_box("iwrupvqb"), black_box("00000"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);