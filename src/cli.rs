use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

/// Pathfinders for gridmaps
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// How program is executed.
    ///
    /// print-map prints the map
    ///
    /// print prints the map with problems
    ///
    /// a-star solves using A*
    ///
    /// fringe solves using Fringe Search
    ///
    /// compare compares a-star and fringe
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

    /// Different outputting modes, default level is 0
    ///
    /// 0 prints map and information before and after solving. Does not time.
    ///
    /// 1 prints information before and after solving. Does not time.
    ///
    /// 2 prints map and some information after solving. Times execution time.
    ///
    /// 3 prints some information after solving. Times execution time.
    ///
    /// 4+ prints map and information before and after solving and during each step. Does not time.
    /// It is recommended to pipe this to less or some other pager or file because this produces a long print.
    #[arg(short, long, default_value_t = 0, action = clap::ArgAction::Count)]
    pub silent: u8,
}

/// Different modes for executing the program
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    Print,
    PrintMap,
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
