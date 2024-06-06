use crate::algorithms::*;
use crate::cli::*;
use crate::problem::Problem;
use crate::structures::map::map_builder;
use crate::structures::AdjacencyListGraph;
use crate::structures::{
    graph::{graph_builder, GraphType},
    map::{ArrayMap, MapType},
};
use crate::{index_to_xy, xy_to_index};

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;

/// Holds all relevant information of map and problems and handles pathfinders
pub struct Context {
    map: ArrayMap,
    graph: AdjacencyListGraph,
    problems: Vec<Problem>,
    mode: Mode,
    print_level: usize,
}

impl Context {
    /// This is mainly for testing purposes.
    /// run() should be used usually.
    /// These are the same, but this does nothing but build automatically
    /// Will not print, but can panic for malformed files
    pub fn new(cli: Cli) -> Self {
        let map_type = MapType::ArrayMap;
        let graph_type = GraphType::AdjacencyListGraph;

        let map = map_builder(cli.map_file.clone(), map_type).expect("invalid map");

        let graph = graph_builder(&map, graph_type);

        let problems = load_problems(
            cli.problem_file,
            cli.map_file.clone(),
            cli.problem_number,
            cli.silent as usize,
        )
        .expect("Error loading problems");

        Context {
            map,
            graph,
            problems,
            mode: cli.mode,
            print_level: cli.silent as usize,
        }
    }

    /// Create self from CLI and run commands as specified.
    pub fn run(cli: Cli) {
        // Early exit
        if matches!(cli.mode, Mode::Print) && cli.silent == 1 {
            let problems = load_problems(
                cli.problem_file,
                cli.map_file.clone(),
                cli.problem_number,
                cli.silent as usize,
            )
            .expect("Error loading problems");

            for problem in &problems {
                println!("{}", problem);
            }
            return;
        }

        // default for now
        let map_type = MapType::ArrayMap;
        let graph_type = GraphType::AdjacencyListGraph;

        if cli.silent <= 2 {
            println!("Loading map {}", cli.map_file.to_str().unwrap());
        }
        let map = map_builder(cli.map_file.clone(), map_type).expect("invalid map");

        if cli.silent <= 2 {
            println!("Map loaded, creating graph");
        }
        let graph = graph_builder(&map, graph_type);

        let problems = load_problems(
            cli.problem_file,
            cli.map_file.clone(),
            cli.problem_number,
            cli.silent as usize,
        )
        .expect("Error loading problems");

        let context = Context {
            map,
            graph,
            problems,
            mode: cli.mode,
            print_level: cli.silent as usize,
        };

        match context.mode {
            Mode::Print => {
                context.print_mode();
            }
            _ => {
                context.solve_mode();
            }
        }
    }

    fn solve_mode(mut self) {
        if self.print_level <= 2 {
            match self.mode {
                Mode::AStar => {
                    println!("Solving using A*");
                }
                Mode::Fringe => {
                    println!("Solving using Fringe search");
                }
                Mode::Compare => {
                    println!("Comparing A* and Fringe search")
                }
                _ => {}
            }
        }

        if self.problems.is_empty() {
            panic!("No problems to solve")
        } else if self.problems.len() == 1 {
            self.solve(0);
        } else {
            self.solve_full();
        }
    }

    fn print_mode(self) {
        match (self.print_level, self.problems.len()) {
            (0, 0) => {
                println!("{}", self.map);
            }
            (0, _) => {
                self.print_problems();
            }
            (1, 0) => {
                println!(
                    "Map width: {}\n    height: {}",
                    self.map.get_width(),
                    self.map.get_height()
                );
            }
            (1, _) => {
                for problem in self.problems {
                    println!("{}", problem);
                }
            }
            _ => {}
        }
    }

    /// Set problem
    pub fn set_problem(&mut self, problem: Problem) {
        self.problems = vec![problem];
    }

    /// Read `n`th (INDEXING STARTS FROM 1!!!) problem from file to the struct.
    pub fn solve_full(&mut self) -> f64 {
        let mut error = 0.0;
        let mut len = 0.0;

        if self.print_level <= 2 {
            println!("Solving {} problems...", self.problems.len());
        }

        for (i, problem) in self.problems.iter().enumerate() {
            let result = self
                .solve(i)
                .unwrap_or_else(|| panic!("Could not find solution for:\n{}", problem));
            let expected = problem.length;
            len += expected.map_or_else(|| 0.0, |_| 1.0);
            if let Some(expected) = problem.length {
                len += 1.0;
                error += (result - expected).abs();
            }
        }
        let average = error / len;
        if self.print_level <= 2 {
            println!("Average error: {}", average);
        }
        average
    }

    /// Solve currently loaded problem. `full_print` handles if results should be printed with a map
    pub fn solve(&self, problem: usize) -> Option<f64> {
        if self.print_level <= 1 {
            println!("{}", self.problems[problem]);
        }
        let Problem {
            start_x,
            start_y,
            goal_x,
            goal_y,
            ..
        } = self.problems[problem];
        let start = xy_to_index(start_x, start_y, self.graph.get_width());
        let goal = xy_to_index(goal_x, goal_y, self.graph.get_width());

        // let heuristic_lookup: Vec<Option<f64>> =
        //     (0..self.map.get_width())
        //     .zip(0..self.map.get_height())
        //         .filter(|(x,y)| self.map.get_cell(x,y).is_some_and(|b| b))
        //         .map(|(x,y)| )

        match self.mode {
            Mode::AStar => {
                let now = Instant::now();

                let astar = AStar::new(start, goal, &self.graph);
                let solution = astar.solve();

                let done = Instant::now();
                let duration = done.checked_duration_since(now);
                if self.print_level <= 2 {
                    if let Some(d) = duration {
                        println!("Solved in {:?}", d);
                    } else {
                        println!("Error in timing");
                    }
                }
                if let Some((path, length)) = solution {
                    self.print_solution(
                        path.iter()
                            .map(|i| index_to_xy(*i, self.graph.get_width()))
                            .collect(),
                        length,
                        problem,
                    );
                    Some(length)
                } else {
                    println!("No path found");
                    None
                }
            }
            Mode::Fringe => {
                let now = Instant::now();
                let fringe = FringeSearch::new(start, goal, &self.graph);
                let solution = fringe.solve();

                let done = Instant::now();
                let duration = done.checked_duration_since(now);
                if self.print_level <= 2 {
                    if let Some(d) = duration {
                        println!("Solved in {:?}", d);
                    } else {
                        println!("Error in timing");
                    }
                }
                if let Some((path, length)) = solution {
                    self.print_solution(
                        path.iter()
                            .map(|i| index_to_xy(*i, self.graph.get_width()))
                            .collect(),
                        length,
                        problem,
                    );
                    Some(length)
                } else {
                    println!("No path found");
                    None
                }
            }
            Mode::Compare => {
                println!("Solving using A*");
                let a_now = Instant::now();

                let astar = AStar::new(start, goal, &self.graph);
                let a_solution = astar.solve();

                let a_done = Instant::now();
                let a_duration = a_done.checked_duration_since(a_now);
                if self.print_level <= 2 {
                    if let Some(d) = a_duration {
                        println!("Solved in {:?}", d);
                    } else {
                        println!("Error in timing");
                    }
                }

                println!("Solving using Fringe search");

                let f_now = Instant::now();
                let fringe = FringeSearch::new(start, goal, &self.graph);
                let f_solution = fringe.solve();

                let f_done = Instant::now();
                let f_duration = f_done.checked_duration_since(f_now);
                if self.print_level <= 2 {
                    if let Some(d) = f_duration {
                        println!("Solved in {:?}", d);
                    } else {
                        println!("Error in timing");
                    }
                }

                match (a_duration, f_duration) {
                    (Some(a), Some(f)) if a < f => {
                        println!("A* was {:?} faster than Fringe search", f - a);
                    }
                    (Some(a), Some(f)) if f < a => {
                        println!("Fringe search was {:?} than A*", a - f);
                    }
                    _ => {
                        println!("Error in timing")
                    }
                }
                Some((a_solution.unwrap().1 - f_solution.unwrap().1).abs())
            }
            _ => {
                panic!("We can't solve in print mode");
            }
        }
    }

    /// Print solution, `full` specifies if map is printed
    fn print_solution(&self, mut path: Vec<(usize, usize)>, path_length: f64, problem: usize) {
        let Problem {
            start_x,
            start_y,
            goal_x,
            goal_y,
            length,
            ..
        } = self.problems[problem];

        if self.print_level == 0 {
            let path: HashSet<(usize, usize)> = path.drain(..).collect();
            let mut result = String::new();
            for y in 0..self.map.get_height() {
                for x in 0..self.map.get_width() {
                    if (x, y) == (start_x, start_y) {
                        result.push('🏁');
                    } else if (x, y) == (goal_x, goal_y) {
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
            println!("{}", self.problems[problem]);
            println!("Result:\n\t{}", path_length);
            if let Some(l) = length {
                println!("Difference:\n\t{}\n", path_length - l);
            }
        } else if self.print_level == 1 {
            println!("Result:\n\t{}", path_length);
            if let Some(l) = length {
                println!("Difference:\n\t{}\n", path_length - l);
            }
        }
    }

    /// Print a problems in a map
    pub fn print_problems(&self) {
        for (
            i,
            Problem {
                start_x,
                start_y,
                goal_x,
                goal_y,
                ..
            },
        ) in self.problems.iter().enumerate()
        {
            let mut result = String::new();
            for y in 0..self.map.get_height() {
                for x in 0..self.map.get_width() {
                    if (x, y) == (*start_x, *start_y) {
                        result.push('🏁');
                    } else if (x, y) == (*goal_x, *goal_y) {
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
            println!("{}\n", self.problems[i]);
        }
    }
}

fn load_problems(
    problem_file: Option<PathBuf>,
    map_file: PathBuf,
    problem_number: Option<usize>,
    print_level: usize,
) -> anyhow::Result<Vec<Problem>> {
    let mut path;

    if let Some(problem) = problem_file {
        path = problem;
    } else {
        path = map_file;
        path.set_extension("map.scenario");
        if !path.as_path().try_exists().is_ok_and(|b| b) {
            path.set_extension("scen");
            if !path.as_path().try_exists().is_ok_and(|b| b) {
                panic!("Could not find a default problem file for map with extensions .scenario and .scen");
            }
        }
    }
    if print_level <= 2 {
        if let Some(s) = path.to_str() {
            println!("Using scenario file {s}");
        }
    }

    if let Some(n) = problem_number {
        println!("Loading problem number {n}");
        Ok(vec![read_problem_from_file(&path, n)
            .expect("Could not find a problem with supplied number")])
    } else {
        Ok(read_full_problem_file(path)?)
    }
}

fn read_problem_from_file(problem_file: &PathBuf, problem: usize) -> anyhow::Result<Problem> {
    let f = File::open(problem_file)?;
    let mut content = BufReader::new(f).lines();

    Problem::parse_problem(content.nth(problem).unwrap()?, problem)
}

fn read_full_problem_file(file_path: PathBuf) -> anyhow::Result<Vec<Problem>> {
    let f = File::open(file_path)?;
    let mut content = BufReader::new(f).lines().enumerate();
    content.next();
    let problems: Vec<Problem> = content
        .flat_map(|(i, row)| {
            Problem::parse_problem(
                row.unwrap_or_else(|_| panic!("Error parsing problem {}", i)),
                i,
            )
        })
        .collect();
    Ok(problems)
}
