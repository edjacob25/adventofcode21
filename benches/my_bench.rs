use criterion::{criterion_group, criterion_main, Criterion};
use advent2021::problems::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut day = c.benchmark_group("Day 1");
    day.bench_function("1", |b| b.iter(|| problem1::part1()));
    day.bench_function("2", |b| b.iter(|| problem1::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 2");
    day.bench_function("1", |b| b.iter(|| problem2::part1()));
    day.bench_function("2", |b| b.iter(|| problem2::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 3");
    day.bench_function("1", |b| b.iter(|| problem3::part1()));
    day.bench_function("2", |b| b.iter(|| problem3::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 4");
    day.bench_function("1", |b| b.iter(|| problem4::part1()));
    day.bench_function("2", |b| b.iter(|| problem4::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 5");
    day.bench_function("1", |b| b.iter(|| problem5::part1()));
    day.bench_function("2", |b| b.iter(|| problem5::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 6");
    day.bench_function("1", |b| b.iter(|| problem6::part1()));
    day.bench_function("2", |b| b.iter(|| problem6::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 7");
    day.bench_function("1", |b| b.iter(|| problem7::part1()));
    day.bench_function("2", |b| b.iter(|| problem7::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 8");
    day.bench_function("1", |b| b.iter(|| problem8::part1()));
    day.bench_function("2", |b| b.iter(|| problem8::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 9");
    day.bench_function("1", |b| b.iter(|| problem9::part1()));
    day.bench_function("2", |b| b.iter(|| problem9::part2()));
    day.finish();

    let mut day = c.benchmark_group("Day 10");
    day.bench_function("1", |b| b.iter(|| problem10::part1()));
    day.bench_function("2", |b| b.iter(|| problem10::part2()));
    day.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);