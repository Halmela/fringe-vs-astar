use criterion::{criterion_group, criterion_main, Criterion};

mod common;

fn compare_berlin_256(c: &mut Criterion) {
    let fringe_context = common::berlin256_context("fringe");
    let astar_context = common::berlin256_context("a-star");

    let mut group = c.benchmark_group("Comparison_Berlin_256");
    group.bench_function("A*", |b| b.iter(|| astar_context.astar()));
    group.bench_function("Fringe", |b| b.iter(|| fringe_context.fringe()));

    group.finish();
}
fn compare_berlin_512(c: &mut Criterion) {
    let fringe_context = common::berlin512_context("fringe");
    let astar_context = common::berlin512_context("a-star");

    let mut group = c.benchmark_group("Comparison_Berlin_512");
    group.bench_function("A*", |b| b.iter(|| astar_context.astar()));
    group.bench_function("Fringe", |b| b.iter(|| fringe_context.fringe()));

    group.finish();
}
fn compare_berlin_1024(c: &mut Criterion) {
    let fringe_context = common::berlin1024_context("fringe");
    let astar_context = common::berlin1024_context("a-star");

    let mut group = c.benchmark_group("Comparison_Berlin_1024");
    group.bench_function("A*", |b| b.iter(|| astar_context.astar()));
    group.bench_function("Fringe", |b| b.iter(|| fringe_context.fringe()));

    group.finish();
}

criterion_group!(
    comparison,
    compare_berlin_256,
    compare_berlin_512,
    compare_berlin_1024
);
criterion_main!(comparison);
