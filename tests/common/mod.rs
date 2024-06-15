use fringe_vs_astar::cli::*;
use fringe_vs_astar::context::*;

use clap::Parser;

pub fn full_lak104d_context(mode: &str) -> Context {
    let arguments = ["", "-sss", mode, "maps/lak104d.map"];
    let cli = Cli::parse_from(arguments.iter());
    Context::new(cli).unwrap()
}
