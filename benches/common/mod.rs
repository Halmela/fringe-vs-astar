use clap::Parser;
use fringe_vs_astar::cli::*;
use fringe_vs_astar::context::*;

pub fn berlin256_context(mode: &str) -> Context {
    let arguments = vec!["", "-sss", "-n", "910", mode, "maps/Berlin_1_256.map"];
    let cli = Cli::parse_from(arguments.iter());
    Context::new(cli)
}
