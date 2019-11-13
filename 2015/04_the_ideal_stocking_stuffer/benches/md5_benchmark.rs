use criterion::{black_box, criterion_group, criterion_main, Criterion};
use md5lib::find_smallest_number;
use md5lib::find_smallest_number_with_six_zeroes;

pub fn criterion_benchmark_five_zeroes(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion_benchmark_five_zeroes");
    group.sample_size(20);
    group.bench_function("Smallest with 00000", |b| b.iter(|| find_smallest_number(black_box("iwrupvqb"))));
    group.finish();
}

pub fn criterion_benchmark_six_zeroes(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion_benchmark_six_zeroes");
    group.sample_size(10);
    group.bench_function("Smallest with 000000", |b| b.iter(|| find_smallest_number_with_six_zeroes(black_box("iwrupvqb"))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark_five_zeroes, criterion_benchmark_six_zeroes);
criterion_main!(benches);