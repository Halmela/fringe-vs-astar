use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    pub mode: Mode,

    #[arg(value_name = "MAP FILE", value_parser = map_exists)]
    pub map_file: PathBuf,

    #[arg(short, long, value_name = "PROBLEM FILE", value_parser = problem_exists)]
    pub problem_file: Option<PathBuf>,

    #[arg(short = 'n', long, value_name = "PROBLEM NUMBER")]
    pub problem_number: Option<usize>,

    #[arg(short, long, default_value_t = false)]
    pub silent: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    Print,
    Solve,
}

fn map_exists(s: &str) -> Result<PathBuf, String> {
    if let Ok(true) = Path::new(s).try_exists() {
        Ok(Path::new(s).to_path_buf())
    } else {
        Err("Map file does not exist".to_string())
    }
}

fn problem_exists(s: &str) -> Result<PathBuf, String> {
    if let Ok(true) = Path::new(s).try_exists() {
        Ok(Path::new(s).to_path_buf())
    } else {
        Err("Supplied problem file does not exist".to_string())
    }
}
