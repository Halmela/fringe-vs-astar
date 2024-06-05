use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

mod common;

fn fringe_berlin_256(c: &mut Criterion) {
    let mut context = common::berlin256_context("fringe");

    c.bench_function("fringe_Berlin_256", |b| b.iter(|| context.solve_full()));
}

criterion_group!(fringe, fringe_berlin_256);
criterion_main!(fringe);
