use fringe_vs_astar::cli::Cli;
use fringe_vs_astar::context::Context;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    Context::run(cli);

    Ok(())
}
