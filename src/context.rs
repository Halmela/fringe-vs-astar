use crate::algorithms::*;
use crate::cli::*;
use crate::problem::Problem;
use crate::structures::map::map_builder;
use crate::structures::{
    graph::{graph_builder, Graph, GraphType},
    map::{Map, MapType},
};

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Holds all relevant information of map and problems and handles pathfinders
pub struct Context {
    map: Box<dyn Map>,
    graph: Box<dyn Graph>,
    problem: Option<Problem>,
}

impl Context {
    /// Create self from CLI and run commands as specified.
    pub fn run(cli: Cli) {
        // default for now
        let map_type = MapType::GridMap;
        let graph_type = GraphType::AdjacencyGridGraph;

        println!("Loading map {}", cli.map_file.to_str().unwrap());
        let map = map_builder(cli.map_file.clone(), map_type).expect("invalid map");

        println!("Map loaded, creating graph");
        let graph = graph_builder(&map, graph_type);

        let mut context = Context {
            map,
            graph,
            problem: None,
        };

        let mut problem_file = Default::default();
        if let Some(problem) = cli.problem_file {
            problem_file = problem;
        } else {
            problem_file = cli.map_file;
            problem_file.set_extension("map.scenario");
            if !problem_file.as_path().try_exists().is_ok_and(|b| b) {
                problem_file.set_extension("scen");
                if !problem_file.as_path().try_exists().is_ok_and(|b| b) {
                    println!("Could not find a default problem file for map with extensions .scenario and .scen");
                    return;
                }
            }
        }

        if let Some(n) = cli.problem_number {
            context
                .read_problem_from_file(&problem_file, n)
                .expect("Could not find a problem with supplied number");
        }

        match cli.mode {
            Mode::Print => {
                if context.problem.is_some() {
                    if cli.silent {
                        println!("{}", context.problem.as_ref().unwrap());
                    } else {
                        context.print_problem();
                    }
                } else {
                    println!("{}", context.map);
                }
            }
            Mode::Solve => {
                if context.problem.is_some() {
                    context.solve(!cli.silent);
                } else {
                    context
                        .run_full_file(problem_file, !cli.silent)
                        .expect("something went wrong running the file");
                }
            }
        }
    }

    /// Set problem
    pub fn set_problem(&mut self, problem: Problem) {
        self.problem = Some(problem);
    }

    /// Read `n`th (INDEXING STARTS FROM 1!!!) problem from file to the struct.
    pub fn read_problem_from_file(
        &mut self,
        problem_file: &PathBuf,
        problem: usize,
    ) -> anyhow::Result<()> {
        let f = File::open(problem_file)?;
        let mut content = BufReader::new(f).lines();
        self.set_problem(Problem::parse_problem(content.nth(problem).unwrap()?)?);

        Ok(())
    }

    /// Read a problem file and run everyone. `print` handles if results should be printed with a map
    pub fn run_full_file(&mut self, file_path: PathBuf, print: bool) -> anyhow::Result<()> {
        let f = File::open(file_path)?;
        let mut content = BufReader::new(f).lines();
        content.next();
        let problems: Vec<Problem> = content
            .map(|row| Problem::parse_problem(row.unwrap()))
            .flatten() // haha lets just get rid of the errors
            .collect();

        for problem in problems {
            self.set_problem(problem);
            self.solve(print);
            println!("");
        }

        Ok(())
    }

    /// Solve currently loaded problem. `full_print` handles if results should be printed with a map
    pub fn solve(&self, full_print: bool) {
        if let Some(Problem {
            start_x,
            start_y,
            goal_x,
            goal_y,
            ..
        }) = self.problem.as_ref()
        {
            println!("{}", self.problem.as_ref().unwrap());
            let astar = AStar::new(*start_x, *start_y, *goal_x, *goal_y, &self.graph);
            if let Some((path, length)) = astar.solve() {
                self.print_solution(path, length, full_print);
            } else {
                println!("No path found");
            }
        }
    }

    /// Print solution, `full` specifies if map is printed
    fn print_solution(&self, mut path: Vec<(usize, usize)>, path_length: f64, full: bool) {
        if let Some(Problem {
            start_x,
            start_y,
            goal_x,
            goal_y,
            length,
        }) = self.problem.as_ref()
        {
            if full {
                let path: HashSet<(usize, usize)> = path.drain(..).collect();
                let mut result = String::new();
                for y in 0..self.map.get_height() {
                    for x in 0..self.map.get_width() {
                        if (&x, &y) == (start_x, start_y) {
                            result.push('🏁');
                        } else if (&x, &y) == (goal_x, goal_y) {
                            result.push('🏆');
                        } else if path.contains(&(x, y)) {
                            result.push('🟩');
                        } else if let Some(true) = self.map.get_cell(x, y) {
                            result.push('⬛');
                        } else {
                            result.push('⬜');
                        }
                    }
                    result.push('\n');
                }
                println!("{}", result);
                println!("{}", self.problem.as_ref().unwrap());
                println!("Result:\n\t{}", path_length);
                if let Some(l) = length {
                    println!("Difference:\n\t{}", path_length - l);
                }
            } else {
                println!("Result:\n\t{}", path_length);
                if let Some(l) = length {
                    println!("Difference:\n\t{}", path_length - l);
                }
            }
        } else {
            println!("No problem to print\n");
        }
    }

    /// Print a problem as a map
    pub fn print_problem(&self) {
        if let Some(Problem {
            start_x,
            start_y,
            goal_x,
            goal_y,
            ..
        }) = self.problem.as_ref()
        {
            let mut result = String::new();
            for y in 0..self.map.get_height() {
                for x in 0..self.map.get_width() {
                    if (&x, &y) == (start_x, start_y) {
                        result.push('🏁');
                    } else if (&x, &y) == (goal_x, goal_y) {
                        result.push('🏆');
                    } else if let Some(true) = self.map.get_cell(x, y) {
                        result.push('⬛');
                    } else {
                        result.push('⬜');
                    }
                }
                result.push('\n');
            }
            println!("{}", result);
            println!("{}\n", self.problem.as_ref().unwrap());
        } else {
            println!("No problem to print\n");
        }
    }
}
