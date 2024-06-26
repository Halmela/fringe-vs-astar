use clap::Parser;
use fringe_vs_astar::cli::Cli;
use fringe_vs_astar::context::{BareContext, Context};

pub fn berlin256_context(mode: &str) -> BareContext {
    let arguments = ["", "-sss", "-n", "910", mode, "maps/Berlin_1_256.map"];
    let cli = Cli::parse_from(arguments.iter());
    Context::new(cli).unwrap().bare()
}

pub fn berlin512_context(mode: &str) -> BareContext {
    let arguments = ["", "-sss", "-n", "1950", mode, "maps/Berlin_1_512.map"];
    let cli = Cli::parse_from(arguments.iter());
    Context::new(cli).unwrap().bare()
}
pub fn berlin1024_context(mode: &str) -> BareContext {
    let arguments = ["", "-sss", "-n", "1920", mode, "maps/Berlin_1_1024.map"];
    let cli = Cli::parse_from(arguments.iter());
    Context::new(cli).unwrap().bare()
}
