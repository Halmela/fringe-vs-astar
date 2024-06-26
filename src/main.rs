use fringe_vs_astar::cli::Cli;
use fringe_vs_astar::context::Context;

use clap::Parser;

fn main() {
    // For some printings the full context is not needed
    if let Some(context) = Context::new(Cli::parse()) {
        context.run();
    }
}
