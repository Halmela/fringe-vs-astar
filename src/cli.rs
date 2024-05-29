use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

/// Pathfinder comparison. Currently only A* is supported
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// How program is executed. print only prints the map and possible problem, a-star solves using A*, fringe solves using Fringe Search
    #[arg(value_enum)]
    pub mode: Mode,

    /// Path to a file that contains a map
    #[arg(value_name = "MAP FILE", value_parser = map_exists)]
    pub map_file: PathBuf,

    /// Path to a file that contains a set of problems. Default is MAP FILE.scen(ario)
    #[arg(short, long, value_name = "PROBLEM FILE", value_parser = problem_exists)]
    pub problem_file: Option<PathBuf>,

    /// 1 indexed indentifier for a problem
    #[arg(short = 'n', long, value_name = "PROBLEM NUMBER")]
    pub problem_number: Option<usize>,

    /// Suppress output. First removes printing of maps, second removes printing of problems, third removes printing of everything.
    #[arg(short, long, default_value_t = 0, action = clap::ArgAction::Count)]
    pub silent: u8,
}

/// Different modes for executing the program
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    Print,
    AStar,
    Fringe,
    Compare,
}

/// Make sure that map-file exists
fn map_exists(s: &str) -> Result<PathBuf, String> {
    if let Ok(true) = Path::new(s).try_exists() {
        Ok(Path::new(s).to_path_buf())
    } else {
        Err("Map file does not exist".to_string())
    }
}

/// Make sure that specified scenario-file exists
fn problem_exists(s: &str) -> Result<PathBuf, String> {
    if let Ok(true) = Path::new(s).try_exists() {
        Ok(Path::new(s).to_path_buf())
    } else {
        Err("Supplied problem file does not exist".to_string())
    }
}
