use criterion::{criterion_group, criterion_main, Criterion};
use advent2021::problems::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 1, part 1", |b| b.iter(|| problem1::part1()));
    c.bench_function("Day 1, part 2", |b| b.iter(|| problem1::part2()));
    c.bench_function("Day 2, part 1", |b| b.iter(|| problem2::part1()));
    c.bench_function("Day 2, part 2", |b| b.iter(|| problem2::part2()));
    c.bench_function("Day 3, part 1", |b| b.iter(|| problem3::part1()));
    c.bench_function("Day 3, part 2", |b| b.iter(|| problem3::part2()));
    c.bench_function("Day 4, part 1", |b| b.iter(|| problem4::part1()));
    c.bench_function("Day 4, part 2", |b| b.iter(|| problem4::part2()));
    c.bench_function("Day 5, part 1", |b| b.iter(|| problem5::part1()));
    c.bench_function("Day 5, part 2", |b| b.iter(|| problem5::part2()));
    c.bench_function("Day 6, part 1", |b| b.iter(|| problem6::part1()));
    c.bench_function("Day 6, part 2", |b| b.iter(|| problem6::part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);