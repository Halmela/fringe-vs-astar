use crate::algorithms::*;
use crate::structures::map::map_builder;
use crate::structures::{
    graph::{graph_builder, Graph, GraphType},
    map::{Map, MapType},
};

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Problem {
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    length: Option<f64>,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str("Problem:\n");
        result.push_str(&format!("\t({}, {}) -> ", self.start_x, self.start_y));
        result.push_str(&format!("({}, {})", self.goal_x, self.goal_y));
        result.push_str("\nExpected cost:\n");
        if let Some(l) = self.length {
            result.push_str(&format!("\t{l}"));
        } else {
            result.push_str("\tNot submitted");
        }
        write!(f, "{}", result)
    }
}

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

    /// Read `n`th (INDEXING STARTS FROM 1!!!) problem from file to the struct.
    pub fn read_problem_from_file(
        &mut self,
        file_path: &str,
        problem: usize,
    ) -> anyhow::Result<()> {
        let f = File::open(file_path)?;
        let mut content = BufReader::new(f).lines();
        let row: String = content.nth(problem).unwrap()?;
        let fields: Vec<&str> = row.split_ascii_whitespace().collect();

        let (_, wanted) = fields.split_at(fields.len() - 5);
        self.problem = Some(Problem {
            start_x: wanted[0].parse()?,
            start_y: wanted[1].parse()?,
            goal_x: wanted[2].parse()?,
            goal_y: wanted[3].parse()?,
            length: Some(wanted[4].parse()?),
        });

        println!("{}\n", self.problem.as_ref().unwrap());

        Ok(())
    }

    pub fn solve(&self) {
        if let Some(Problem {
            start_x,
            start_y,
            goal_x,
            goal_y,
            ..
        }) = self.problem.as_ref()
        {
            let astar = AStar::new(*start_x, *start_y, *goal_x, *goal_y, &self.graph);
            astar.solve();
        } else {
            println!("Nothing to solve");
            return;
        }
    }
}
