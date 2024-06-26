use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

mod common;

fn fringe_berlin_256(c: &mut Criterion) {
    let context = common::berlin256_context("fringe");

    c.bench_function("fringe_Berlin_256", |b| b.iter(|| context.fringe()));
}
fn fringe_berlin_512(c: &mut Criterion) {
    let context = common::berlin512_context("fringe");

    c.bench_function("fringe_Berlin_512", |b| b.iter(|| context.fringe()));
}
fn fringe_berlin_1024(c: &mut Criterion) {
    let context = common::berlin1024_context("fringe");

    c.bench_function("fringe_Berlin_1024", |b| b.iter(|| context.fringe()));
}

criterion_group!(
    fringe,
    fringe_berlin_256,
    fringe_berlin_512,
    fringe_berlin_1024
);
criterion_main!(fringe);
