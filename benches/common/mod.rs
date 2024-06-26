use clap::Parser;
use fringe_vs_astar::cli::Cli;
use fringe_vs_astar::context::{BareContext, Context};

pub fn berlin256_context(mode: &str) -> BareContext {
    let arguments = ["", "-sss", "-n", "910", mode, "maps/Berlin_1_256.map"];
    let cli = Cli::parse_from(arguments.iter());
    Context::new(cli).unwrap().bare()
}
