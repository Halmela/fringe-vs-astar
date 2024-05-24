use crate::algorithms::*;
use crate::problem::Problem;
use crate::structures::map::map_builder;
use crate::structures::{
    graph::{graph_builder, Graph, GraphType},
    map::{Map, MapType},
};

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Context {
    map: Box<dyn Map>,
    graph: Box<dyn Graph>,
    problem: Option<Problem>,
}

impl Context {
    pub fn new(file_path: &str, map_type: MapType, graph_type: GraphType) -> Self {
        let map = map_builder(file_path, map_type).unwrap();
        let graph = graph_builder(&map, graph_type);

        Context {
            map,
            graph,
            problem: None,
        }
    }

    pub fn set_problem(&mut self, problem: Problem) {
        self.problem = Some(problem);
    }

    /// Read `n`th (INDEXING STARTS FROM 1!!!) problem from file to the struct.
    pub fn read_problem_from_file(
        &mut self,
        file_path: &str,
        problem: usize,
    ) -> anyhow::Result<()> {
        let f = File::open(file_path)?;
        let mut content = BufReader::new(f).lines();
        self.set_problem(Problem::parse_problem(content.nth(problem).unwrap()?)?);

        Ok(())
    }

    pub fn run_full_file(&mut self, file_path: &str, print: bool) -> anyhow::Result<()> {
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
                println!("{}\n", self.problem.as_ref().unwrap());
                println!("Result:\n\t{}", path_length);
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
