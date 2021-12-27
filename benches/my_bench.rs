use advent2021::problems::*;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_day {
    ($i:ident, $day:literal, $day_path:ident) => {
        let mut day = $i.benchmark_group(concat!("Day ", $day));
        day.bench_function("1", |b| b.iter(|| $day_path::part1()));
        day.bench_function("2", |b| b.iter(|| $day_path::part2()));
        day.finish();
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_day!(c, "01", problem1);
    bench_day!(c, "02", problem2);
    bench_day!(c, "03", problem3);
    bench_day!(c, "04", problem4);
    bench_day!(c, "05", problem5);
    bench_day!(c, "06", problem6);
    bench_day!(c, "07", problem7);
    bench_day!(c, "08", problem8);
    bench_day!(c, "09", problem9);
    bench_day!(c, "10", problem10);
    bench_day!(c, "11", problem11);
    bench_day!(c, "12", problem12);
    bench_day!(c, "13", problem13);
    bench_day!(c, "14", problem14);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
