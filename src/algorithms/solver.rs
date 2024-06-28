use super::{AStar, FringeSearch, State};
use crate::printable::Printable;
use crate::problem::Problem;
use crate::structures::Graph;
use std::cmp::max;
use std::fmt;
use std::fmt::Display;
use std::time::{Duration, Instant};

/// Different algorithms as enums
#[derive(Clone, Copy)]
pub enum Algorithm {
    AStar,
    Fringe,
}

/// Represent what is wanted as the result of a solving process.
#[derive(Clone)]
pub enum Result {
    Time(Printable),
    EndState(Printable),
    Full(Printable),
}

impl Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Result::Time(p) | Result::EndState(p) | Result::Full(p) => write!(f, "{p}"),
        }
    }
}

/// Runner for A* and Fringe search. Handles printing according to `Result` value.
pub struct Solver<'a> {
    algorithm: Algorithm,
    result: Result,
    problem: Problem,
    graph: &'a Graph,
}

impl<'a> Solver<'a> {
    /// Initialize self
    #[must_use]
    pub fn new(algorithm: Algorithm, result: Result, problem: Problem, graph: &'a Graph) -> Self {
        Solver {
            algorithm,
            result,
            problem,
            graph,
        }
    }

    /// Run the algorithm with wanted printing mode
    pub fn run(self) {
        match (self.algorithm, self.result.clone()) {
            (Algorithm::AStar, Result::EndState(p)) => self.full_astar(p, false),
            (Algorithm::AStar, Result::Full(p)) => self.full_astar(p, true),
            (Algorithm::AStar, Result::Time(p)) => self.timed_astar(p),
            (Algorithm::Fringe, Result::EndState(p)) => self.printed_fringe(p, false),
            (Algorithm::Fringe, Result::Full(p)) => self.printed_fringe(p, true),
            (Algorithm::Fringe, Result::Time(p)) => self.timed_fringe(p),
        }
    }

    /// Run A* search and add path and timing to [`Printable`] before printing it.
    fn timed_astar(self, mut printable: Printable) {
        let start = self.problem.start;
        let goal = self.problem.goal;
        let now = Instant::now();

        let astar = AStar::new(start, goal, self.graph);
        let solution = astar.solve();

        let done = Instant::now();
        let duration = done.checked_duration_since(now);

        printable.add_header(
            "Duration",
            duration.map_or_else(|| "Error in timing".to_string(), |d| format!("{d:?}")),
        );
        if let Some((path, length)) = solution {
            printable.add_path(path);
            printable.add_header("Length", length);
        }
        println!("{printable}");
    }

    /// Run Fringe search and add path and timing to [`Printable`] before printing it.
    fn timed_fringe(self, mut printable: Printable) {
        let start = self.problem.start;
        let goal = self.problem.goal;
        let now = Instant::now();

        let fringe = FringeSearch::new(start, goal, self.graph);
        let solution = fringe.solve();

        let done = Instant::now();
        let duration = done.checked_duration_since(now);

        printable.add_header(
            "Duration",
            duration.map_or_else(|| "Error in timing".to_string(), |d| format!("{d:?}")),
        );

        if let Some((path, length)) = solution {
            printable.add_path(path);
            printable.add_header("Length", length);
        }
        println!("{printable}");
    }

    /// Run A* search and collect statistics and inner state.
    /// `full` indicates if every state of solving process should be printed.
    fn full_astar(self, printable: Printable, full: bool) {
        let mut astar = AStar::new(self.problem.start, self.problem.goal, self.graph);
        let mut operations = 0;
        let mut max_open = 0;
        let mut total_duration = Duration::ZERO;

        println!("{printable}");

        loop {
            operations += 1;
            let earlier = Instant::now();
            let state = astar.progress();
            let now = Instant::now();
            let duration = now.duration_since(earlier);
            total_duration += duration;
            match state {
                State::Processing(node) => {
                    max_open = max(max_open, astar.size());
                    if full {
                        let mut print = printable.clone();
                        print.add_header("Operation", operations);
                        print.add_header("  Δ", format!("{:?}", duration));
                        print.add_header("  ΣΔ", format!("{:?}", total_duration));
                        print.add_header("  μΔ", format!("{:?}", total_duration / operations));
                        print = astar.add_to_printable(print);
                        print.add_current(node, astar.get_cost(node), astar.get_estimate(node));
                        print.add_spacing();
                        println!("{print}");
                    }
                }
                State::Finished((path, cost)) => {
                    let mut print = printable.clone();
                    print.add_header("Operations", operations);
                    print.add_header("  Δ", format!("{:?}", duration));
                    print.add_header("  ΣΔ", format!("{:?}", total_duration));
                    print.add_header("  μΔ", format!("{:?}", total_duration / operations));
                    print = astar.add_to_printable(print);
                    print.add_path(path);
                    print.add_header("Length", cost);
                    print.add_spacing();
                    print.add_header("Max |Open|", max_open);
                    println!("{print}");
                    break;
                }
                State::NotFound => {
                    println!("Path not found");
                    break;
                }
            }
        }
    }

    /// Run Fringe search and collect statistics and inner state.
    /// `full` indicates if every state of solving process should be printed.
    fn printed_fringe(self, printable: Printable, full: bool) {
        let mut fringe = FringeSearch::new(self.problem.start, self.problem.goal, self.graph);
        let mut operations = 0;
        let mut max_now = 0;
        let mut max_current = 0;
        let mut max_later = 0;
        let mut total_duration = Duration::ZERO;
        // let mut closed_hit = 0;

        println!("{printable}");

        loop {
            operations += 1;
            let earlier = Instant::now();
            let state = fringe.progress();
            let now = Instant::now();
            let duration = now.duration_since(earlier);
            total_duration += duration;
            match state {
                State::Processing(node) => {
                    max_now = max(max_now, fringe.now_size());
                    max_current = max(max_current, fringe.bucket_size());
                    max_later = max(max_later, fringe.later_size());

                    // if fringe.next_is_closed() {
                    //     closed_hit += 1;
                    // }

                    if full {
                        let mut print = printable.clone();
                        print.add_header("Operation", operations);
                        print.add_header("  Δ", format!("{:?}", duration));
                        print.add_header("  ΣΔ", format!("{:?}", total_duration));
                        print.add_header("  μΔ", format!("{:?}", total_duration / operations));
                        print = fringe.add_to_printable(print);
                        print.add_current(node, fringe.get_cost(node), fringe.get_estimate(node));
                        print.add_spacing();
                        // print.add_header("closed?", fringe.next_is_closed());
                        println!("{print}");
                    }
                }
                State::Finished((path, cost)) => {
                    let mut print = printable.clone();
                    print.add_header("Operations", operations);
                    print.add_header("  Δ", format!("{:?}", duration));
                    print.add_header("  ΣΔ", format!("{:?}", total_duration));
                    print.add_header("  μΔ", format!("{:?}", total_duration / operations));
                    print = fringe.add_to_printable(print);
                    print.add_path(path);
                    print.add_header("Length", cost);
                    print.add_spacing();
                    print.add_header("Max", "");
                    print.add_header("  |Now|", max_now);
                    print.add_header("  |Bucket|", max_current);
                    print.add_header("  |Later|", max_later);
                    // print.add_header("Closed", closed_hit);
                    println!("{print}");
                    break;
                }
                State::NotFound => {
                    println!("Path not found");
                    break;
                }
            }
        }
    }
}
