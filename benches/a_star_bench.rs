use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

mod common;

fn a_star_berlin_256(c: &mut Criterion) {
    let context = common::berlin256_context("a-star");

    c.bench_function("a_star_Berlin_256", |b| b.iter(|| context.astar()));
}

criterion_group!(a_star, a_star_berlin_256);
criterion_main!(a_star);
