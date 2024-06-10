use criterion::{criterion_group, criterion_main, Criterion};

mod common;

fn compare_berlin_256(c: &mut Criterion) {
    let fringe_context = common::berlin256_context("fringe");
    let astar_context = common::berlin256_context("a-star");

    let mut group = c.benchmark_group("Comparison");
    group.bench_function("A*", |b| b.iter(|| astar_context.astar()));
    group.bench_function("Fringe", |b| b.iter(|| fringe_context.fringe()));

    group.finish();
}

criterion_group!(comparison, compare_berlin_256);
criterion_main!(comparison);
