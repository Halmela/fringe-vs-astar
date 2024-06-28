use crate::algorithms::{AStar, Algorithm, FringeSearch, Result, Solver};
use crate::cli::{Cli, Mode};
use crate::printable::Printable;
use crate::problem::{Problem, Problems};
use crate::structures::{Graph, Map};
use crate::Node;

use std::time::Duration;
use std::time::Instant;

/// Holds all relevant information of map and problems and handles pathfinders
pub struct Context {
    graph: Graph,
    problems: Problems,
    mode: Mode,
    printable: Printable,
    print_level: usize,
}

impl Context {
    /// This is mainly for testing purposes.
    /// `run()` should be used usually.
    /// These are the same, but this does nothing but build automatically
    /// Will not print, but can panic for malformed files
    #[must_use]
    pub fn new(cli: Cli) -> Option<Self> {
        let scenario_file = cli
            .problem_file
            .unwrap_or_else(|| Problems::deduce_problem_file(cli.map_file.clone()));

        if cli.silent <= 2 {
            println!("Using scenario file {}", scenario_file.to_str().unwrap());
        }
        let problems =
            Problems::new(scenario_file, cli.problem_number).expect("Error loading problems");

        // Early exit
        if matches!(cli.mode, Mode::Print) && cli.silent == 1 {
            println!("{problems}");
            return None;
        }

        let map_name = cli.map_file.to_str().unwrap().to_owned();
        if cli.silent <= 2 {
            println!("Loading map {map_name}");
        }
        let map = Map::new(cli.map_file);
        let mut printable = Printable::new(&map);
        printable.add_header("Map", map_name);

        if matches!(cli.mode, Mode::PrintMap) {
            println!("{printable}");
            return None;
        }

        if cli.silent <= 2 {
            println!("Map loaded, creating graph");
        }
        let graph = Graph::new(map);
        printable.add_header("Branching", graph.average_branching());

        Some(Context {
            graph,
            problems,
            mode: cli.mode,
            print_level: cli.silent as usize,
            printable,
        })
    }

    /// Strip down everything unnecessary and return [`BareContext`] that is more suitable for benchmarking
    #[must_use]
    pub fn bare(self) -> BareContext {
        let bare_problems = self.problems.iter().map(|p| (p.start, p.goal)).collect();
        BareContext {
            graph: self.graph,
            bare_problems,
        }
    }

    /// Create self from CLI and run commands as specified.
    pub fn run(self) {
        match self.mode {
            Mode::Print => {
                self.print_mode();
            }
            Mode::PrintMap => {}
            _ => {
                self.solve_mode();
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
                    println!("Comparing A* and Fringe search");
                }
                _ => {}
            }
        }

        if self.problems.is_empty() {
            panic!("No problems to solve")
        } else if let Some(problem) = self.problems.single_problem() {
            if self.mode == Mode::Compare {
                let _ = self.solve(problem);
            } else {
                self.use_solver(problem);
            }
        } else {
            self.solve_full();
        }
    }

    fn print_mode(self) {
        match (self.print_level, self.problems.len()) {
            (0, _) => {
                self.print_problems();
            }
            // (1, 0) => {
            //     println!(
            //         "Map width: {}\n    height: {}",
            //         self.map.get_width(),
            //         self.map.get_height()
            //     );
            // }
            (1, _) => {
                println!("{}", self.problems);
            }
            _ => {}
        }
    }

    fn use_solver(self, problem: Problem) {
        let mut printable = self.printable.clone();
        printable.add_problem(&problem);
        printable.add_spacing();

        let algorithm = match self.mode {
            Mode::AStar => {
                printable.add_header("Algorithm", "A*");
                Algorithm::AStar
            }
            Mode::Fringe => {
                printable.add_header("Algorithm", "Fringe search");
                Algorithm::Fringe
            }
            _ => panic!("use_solver does not support this mode of operation"),
        };

        let result = match self.print_level {
            0 => Result::EndState(printable),
            1 => {
                printable.suppress_print();
                Result::EndState(printable)
            }
            2 => Result::Time(printable),
            3 => {
                printable.suppress_print();
                Result::Time(printable)
            }
            4 => Result::Full(printable),
            _ => {
                printable.suppress_print();
                Result::Full(printable)
            }
        };

        let solver = Solver::new(algorithm, result, problem, &self.graph);
        solver.run();
    }

    /// Read `n`th (INDEXING STARTS FROM 1!!!) problem from file to the struct.
    pub fn solve_full(&mut self) -> f32 {
        let mut error = 0.0;
        let mut count = 0.0;

        if self.print_level <= 2 {
            println!("Solving {} problems...", self.problems.len());
        }

        for problem in self.problems.iter() {
            let result = self
                .solve(*problem)
                .unwrap_or_else(|| panic!("Could not find solution for:\n{problem}"));
            let expected = problem.length;
            count += expected.map_or_else(|| 0.0, |_| 1.0);
            if let Some(expected) = problem.length {
                count += 1.0;
                error += (result - expected).abs();
            }
        }
        let average = error / count;
        if self.print_level <= 2 {
            println!("Average error: {average}");
        }
        average
    }

    /// Solve currently loaded problem.
    #[must_use]
    pub fn solve(&self, problem: Problem) -> Option<f32> {
        if self.print_level <= 1 {
            println!("{problem}");
        }

        match self.mode {
            Mode::AStar => {
                let (solution, duration) = self.timed_astar(&problem);
                self.print_solution(solution, problem, duration)
            }
            Mode::Fringe => {
                let (solution, duration) = self.timed_fringe(&problem);
                self.print_solution(solution, problem, duration)
            }
            Mode::Compare => {
                println!("Solving using A*");
                let (a_solution, a_duration) = self.timed_astar(&problem);
                self.print_timing(a_duration);

                println!("Solving using Fringe search");
                let (f_solution, f_duration) = self.timed_fringe(&problem);
                self.print_timing(f_duration);

                match (a_duration, f_duration) {
                    (Some(a), Some(f)) if a < f => {
                        println!("A* was {:?} faster than Fringe search", f - a);
                    }
                    (Some(a), Some(f)) if f < a => {
                        println!("Fringe search was {:?} faster than A*", a - f);
                    }
                    _ => {
                        println!("Error in timing");
                    }
                }
                Some((a_solution.unwrap().1 - f_solution.unwrap().1).abs())
            }
            _ => {
                panic!("We can't solve in print mode");
            }
        }
    }

    fn timed_astar(&self, problem: &Problem) -> (Option<(Vec<Node>, f32)>, Option<Duration>) {
        let now = Instant::now();

        let astar = AStar::new(problem.start, problem.goal, &self.graph);
        let solution = astar.solve();

        let done = Instant::now();
        let duration = done.checked_duration_since(now);

        (solution, duration)
    }

    fn timed_fringe(&self, problem: &Problem) -> (Option<(Vec<Node>, f32)>, Option<Duration>) {
        let now = Instant::now();

        let fringe = FringeSearch::new(problem.start, problem.goal, &self.graph);
        let solution = fringe.solve();

        let done = Instant::now();
        let duration = done.checked_duration_since(now);

        (solution, duration)
    }

    fn print_timing(&self, duration: Option<Duration>) {
        if self.print_level <= 2 {
            if let Some(d) = duration {
                println!("Solved in {d:?}");
            } else {
                println!("Error in timing");
            }
        }
    }

    /// Print solution, `full` specifies if map is printed
    fn print_solution(
        &self,
        solution: Option<(Vec<Node>, f32)>,
        problem: Problem,
        duration: Option<Duration>,
    ) -> Option<f32> {
        let _path;
        let path_length;

        if let Some((p, l)) = solution {
            _path = p;
            path_length = l;
        } else {
            println!("No path found");
            return None;
        }

        if self.print_level == 0 {
            let mut printable = self.printable.clone();
            // printable.add_path(path);
            // printable.add_problem(&problem);
            // printable.add_header("Branching", self.graph.average_branching());
            // printable.add_spacing();

            if let Some(d) = duration {
                printable.add_header("Duration", format!("{d:?}"));
            }
            printable.add_header("Length", path_length);
            if let Some(l) = problem.length {
                printable.add_header("Difference", path_length - l);
            }

            println!("{printable}\n");
        } else if self.print_level == 1 {
            println!("Result:\n\t{path_length}");
            if let Some(l) = problem.length {
                println!("Difference:\n\t{}\n", path_length - l);
            }
        }
        Some(path_length)
    }

    /// Print a problems in a map
    pub fn print_problems(&self) {
        for problem in self.problems.iter() {
            println!("{}", self.problem_in_map(problem));
        }
    }

    fn problem_in_map(&self, problem: &Problem) -> Printable {
        let mut result = self.printable.clone();
        result.add_problem(problem);

        result
    }
}

/// Stripped down version of [`Context`] used only for benchmarking.
/// Contains only the graph and a list of pre processed problems.
pub struct BareContext {
    graph: Graph,
    bare_problems: Vec<(Node, Node)>,
}

impl BareContext {
    /// Solve problems using A* and drop the results
    #[allow(unused_must_use)]
    pub fn astar(&self) {
        for (start, goal) in &self.bare_problems {
            let astar = AStar::new(*start, *goal, &self.graph);

            astar.solve();
        }
    }

    /// Solve problems using Fringe search and drop the results
    #[allow(unused_must_use)]
    pub fn fringe(&self) {
        for (start, goal) in &self.bare_problems {
            let fringe = FringeSearch::new(*start, *goal, &self.graph);

            fringe.solve();
        }
    }
}
