use criterion::{black_box, criterion_group, criterion_main, Criterion};
use md5lib::find_smallest_number;
use md5lib::find_smallest_number_with_six_zeroes;

pub fn criterion_benchmark_five_zeroes(_c: &mut Criterion) {
    let mut c_prime = Criterion::default().sample_size(20);
    c_prime.bench_function("Smallest with 00000", |b| b.iter(|| find_smallest_number(black_box("iwrupvqb"))));
}

pub fn criterion_benchmark_six_zeroes(_c: &mut Criterion) {
    let mut c_prime = Criterion::default().sample_size(10);
    c_prime.bench_function("Smallest with 000000", |b| b.iter(|| find_smallest_number_with_six_zeroes(black_box("iwrupvqb"))));
}

criterion_group!(benches, criterion_benchmark_five_zeroes, criterion_benchmark_six_zeroes);
criterion_main!(benches);