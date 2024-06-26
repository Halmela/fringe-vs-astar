use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

mod common;

fn a_star_berlin_256(c: &mut Criterion) {
    let context = common::berlin256_context("a-star");

    c.bench_function("a_star_Berlin_256", |b| b.iter(|| context.astar()));
}
fn a_star_berlin_512(c: &mut Criterion) {
    let context = common::berlin512_context("a-star");

    c.bench_function("a_star_Berlin_512", |b| b.iter(|| context.astar()));
}
fn a_star_berlin_1024(c: &mut Criterion) {
    let context = common::berlin1024_context("a-star");

    c.bench_function("a_star_Berlin_1024", |b| b.iter(|| context.astar()));
}

criterion_group!(
    a_star,
    a_star_berlin_256,
    a_star_berlin_512,
    a_star_berlin_1024
);
criterion_main!(a_star);
