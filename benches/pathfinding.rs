use clap::Parser;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

use fringe_vs_astar::cli::*;
use fringe_vs_astar::context::*;

fn berlin256_context(mode: &str) -> Context {
    let arguments = vec!["", "-sss", "-n", "910", mode, "maps/Berlin_1_256.map"];
    let cli = Cli::parse_from(arguments.iter());
    Context::new(cli)
}

fn fringe_berlin_256(c: &mut Criterion) {
    let mut context = berlin256_context("fringe");

    c.bench_function("fringe_Berlin_256", |b| b.iter(|| context.solve_full()));
}

fn a_star_berlin_256(c: &mut Criterion) {
    let mut context = berlin256_context("a-star");

    c.bench_function("a_star_Berlin_256", |b| b.iter(|| context.solve_full()));
}

criterion_group!(benches, a_star_berlin_256, fringe_berlin_256);
criterion_main!(benches);
