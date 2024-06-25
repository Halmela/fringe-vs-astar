use crate::algorithms::AStar;
use crate::algorithms::FringeSearch;
use crate::algorithms::State;
use crate::printable::Printable;
use crate::problem::Problem;
use crate::structures::Graph;
use crate::xy_to_index;
use std::cmp::max;
use std::fmt;
use std::fmt::Display;
use std::time::Instant;

#[derive(Clone, Copy)]
pub enum Algorithm {
    AStar,
    Fringe,
}

#[derive(Clone)]
pub enum Result {
    Time(Printable),
    EndState(Printable),
    Full(Printable),
}

impl Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Result::Time(p) => write!(f, "{p}"),
            Result::EndState(p) => write!(f, "{p}"),
            Result::Full(p) => write!(f, "{p}"),
        }
    }
}

pub struct Solver {
    algorithm: Algorithm,
    result: Result,
    problem: Problem,
    graph: Graph,
    width: usize,
}

impl Solver {
    pub fn new(
        algorithm: Algorithm,
        result: Result,
        problem: Problem,
        graph: Graph,
        width: usize,
    ) -> Self {
        Solver {
            algorithm,
            result,
            problem,
            graph,
            width,
        }
    }

    pub fn run(self) {
        match (self.algorithm, self.result.to_owned()) {
            (Algorithm::AStar, Result::EndState(p)) => self.full_astar(p, false),
            (Algorithm::AStar, Result::Full(p)) => self.full_astar(p, true),
            (Algorithm::AStar, Result::Time(p)) => self.timed_astar(p),
            (Algorithm::Fringe, Result::EndState(p)) => self.printed_fringe(p, false),
            (Algorithm::Fringe, Result::Full(p)) => self.printed_fringe(p, true),
            (Algorithm::Fringe, Result::Time(p)) => self.timed_fringe(p),
        }
    }

    fn timed_astar(self, mut printable: Printable) {
        let start = xy_to_index(self.problem.start_x, self.problem.start_y, self.width);
        let goal = xy_to_index(self.problem.goal_x, self.problem.goal_y, self.width);
        let now = Instant::now();

        let astar = AStar::new(start, goal, &self.graph);
        let solution = astar.solve();

        let done = Instant::now();
        let duration = done.checked_duration_since(now);

        printable.add_header(
            "Duration",
            duration
                .map(|d| format!("{:?}", d))
                .unwrap_or_else(|| "Error in timing".to_string()),
        );
        if let Some((path, length)) = solution {
            printable.add_path(path);
            printable.add_header("Length", length);
        }
        println!("{printable}");
    }

    fn timed_fringe(self, mut printable: Printable) {
        let start = xy_to_index(self.problem.start_x, self.problem.start_y, self.width);
        let goal = xy_to_index(self.problem.goal_x, self.problem.goal_y, self.width);
        let now = Instant::now();

        let fringe = FringeSearch::new(start, goal, &self.graph);
        let solution = fringe.solve();

        let done = Instant::now();
        let duration = done.checked_duration_since(now);

        printable.add_header(
            "Duration",
            duration
                .map(|d| format!("{:?}", d))
                .unwrap_or_else(|| "Error in timing".to_string()),
        );

        if let Some((path, length)) = solution {
            printable.add_path(path);
            printable.add_header("Length", length);
        }
        println!("{printable}");
    }

    fn printed_fringe(self, printable: Printable, full: bool) {
        let start = xy_to_index(self.problem.start_x, self.problem.start_y, self.width);
        let goal = xy_to_index(self.problem.goal_x, self.problem.goal_y, self.width);

        let mut fringe = FringeSearch::new(start, goal, &self.graph);
        let mut iterations = 0;
        let mut max_now = 0;
        let mut max_current = 0;
        let mut max_later = 0;

        println!("{printable}");

        loop {
            iterations += 1;
            match fringe.progress() {
                State::Processing(node) => {
                    max_now = max(max_now, fringe.now_size());
                    max_current = max(max_current, fringe.bucket_size());
                    max_later = max(max_later, fringe.later_size());

                    if full {
                        let mut print = printable.clone();
                        print.add_header("Iteration", iterations);
                        print = fringe.add_to_printable(print);
                        print.add_current(node);
                        print.add_spacing();
                        print.add_header("Current", "");
                        print.add_header("  cost", fringe.get_cost(node));
                        print.add_header("  estimate", fringe.get_estimate(node));
                        println!("{print}");
                    }
                }
                State::Finished((path, cost)) => {
                    let mut print = printable.clone();
                    print.add_header("Iteration", iterations);
                    print = fringe.add_to_printable(print);
                    print.add_path(path);
                    print.add_header("Length", cost);
                    print.add_spacing();
                    print.add_header("Max |Now|", max_now);
                    print.add_header("Max |Bucket|", max_current);
                    print.add_header("Max |Later|", max_later);
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
    fn full_astar(self, printable: Printable, full: bool) {
        let start = xy_to_index(self.problem.start_x, self.problem.start_y, self.width);
        let goal = xy_to_index(self.problem.goal_x, self.problem.goal_y, self.width);

        let mut astar = AStar::new(start, goal, &self.graph);
        let mut actions = 0;
        let mut max_open = 0;

        println!("{printable}");

        loop {
            actions += 1;
            match astar.progress() {
                State::Processing(node) => {
                    max_open = max(max_open, astar.size());
                    if full {
                        let mut print = printable.clone();
                        print.add_header("Iteration", actions);
                        print = astar.add_to_printable(print);
                        print.add_current(node);
                        print.add_spacing();
                        print.add_header("Current", "");
                        print.add_header("  cost", astar.get_cost(node));
                        print.add_header("  estimate", astar.get_estimate(node));
                        println!("{print}");
                    }
                }
                State::Finished((path, cost)) => {
                    let mut print = printable.clone();
                    print.add_header("Iteration", actions);
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
}
