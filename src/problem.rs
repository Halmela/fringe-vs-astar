use crate::{index_to_xy, xy_to_index, Node};
use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Problem to be solved. Length is optional, because it might not always be present.
///
/// This is built around formatting as described by [MovingAI](https://www.movingai.com/benchmarks/formats.html).
#[derive(Clone, Copy)]
pub struct Problem {
    pub start: Node,
    pub goal: Node,
    pub map_width: usize,
    pub length: Option<f32>,
    pub number: usize,
}

impl Problem {
    /// Create problem
    pub fn new(
        map_width: usize,
        start_x: usize,
        start_y: usize,
        goal_x: usize,
        goal_y: usize,
        length: Option<f32>,
        number: usize,
    ) -> Problem {
        let start = xy_to_index(start_x, start_y, map_width);
        let goal = xy_to_index(goal_x, goal_y, map_width);
        Problem {
            start,
            goal,
            length,
            number,
            map_width,
        }
    }

    /// Parse `.scenario` row as a problem.
    pub fn parse(value: String, number: usize) -> anyhow::Result<Problem> {
        let fields: Vec<&str> = value.split_ascii_whitespace().collect();
        let wanted = fields.split_at(fields.len() - 7).1;

        Ok(Problem::new(
            wanted[0].parse()?,     // width
            wanted[2].parse()?,     // start x
            wanted[3].parse()?,     // start y
            wanted[4].parse()?,     // goal x
            wanted[5].parse()?,     // goal y
            wanted[6].parse().ok(), // length
            number,
        ))
    }

    /// Read single problem from a .scenario file
    pub fn from_file(problem_file: &PathBuf, problem: usize) -> anyhow::Result<Problem> {
        let f = File::open(problem_file)?;
        let mut content = BufReader::new(f).lines();

        Problem::parse(content.nth(problem).unwrap()?, problem)
    }

    /// Provide start as (x, y) -coordinates
    pub fn start_xy(&self) -> (usize, usize) {
        index_to_xy(self.start, self.map_width)
    }
    /// Provide goal as (x, y) -coordinates
    pub fn goal_xy(&self) -> (usize, usize) {
        index_to_xy(self.goal, self.map_width)
    }

    /// Pretty printing for coordinates
    pub fn coordinates(&self) -> String {
        let (start_x, start_y) = self.start_xy();
        let (goal_x, goal_y) = self.goal_xy();
        format!("({}, {}) -> ({}, {})", start_x, start_y, goal_x, goal_y)
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!(
            "Problem {}:\n\t{}",
            self.number,
            self.coordinates()
        ));
        if let Some(l) = self.length {
            result.push_str(&format!("\t{l}"));
        }
        writeln!(f, "{}", result)
    }
}

/// List of [`Problem`]s. This is read from a file. Usually only one of Problems is used or every Problem is used.
pub struct Problems {
    problems: Vec<Problem>,
    file: String,
}

impl Problems {
    /// Initialize from a file.
    /// If a problem number is supplied, it will be the only [`Problem`].
    pub fn new(problem_file: PathBuf, problem_number: Option<usize>) -> anyhow::Result<Problems> {
        if let Some(n) = problem_number {
            Ok(Problems {
                problems: vec![Problem::from_file(&problem_file, n)
                    .expect("Could not find a problem with supplied number")],
                file: problem_file.to_str().unwrap().to_string(),
            })
        } else {
            Ok(Problems::from_file(problem_file)?)
        }
    }

    /// Read the supplied scenario file and parse the problems.
    /// Panics if the scenario file is malformed.
    pub fn from_file(file_path: PathBuf) -> anyhow::Result<Problems> {
        let f = File::open(&file_path)?;
        let mut content = BufReader::new(f).lines().enumerate();
        content.next();
        let problems: Vec<Problem> = content
            .flat_map(|(i, row)| {
                Problem::parse(
                    row.unwrap_or_else(|_| panic!("Error parsing problem {}", i)),
                    i,
                )
            })
            .collect();
        Ok(Problems {
            problems,
            file: file_path.to_str().unwrap().to_string(),
        })
    }

    /// Returns `true` if it contains no [`Problem`]s.
    pub fn is_empty(&self) -> bool {
        self.problems.is_empty()
    }

    /// Get a [`Problem`] number `i` if it exists
    pub fn get(&self, i: usize) -> Option<&Problem> {
        self.problems.get(i)
    }

    /// Returns the number of [`Problem`]s.
    pub fn len(&self) -> usize {
        self.problems.len()
    }

    /// Provide the only [`Problem`].
    /// Returns None, if there are less or more than 1 Problems.
    pub fn single_problem(&mut self) -> Option<Problem> {
        if self.problems.len() == 1 {
            self.problems.pop()
        } else {
            None
        }
    }

    /// Wrapper for iterating over self.
    pub fn iter(&self) -> impl Iterator<Item = &Problem> {
        self.problems.iter()
    }

    /// Try to find scenario file with `.scenario` or `.scen` extension, panic neither is found.
    /// This is used if a separate scenario file is not supplied.
    pub fn deduce_problem_file(mut path: PathBuf) -> PathBuf {
        path.set_extension("map.scenario");
        if path.as_path().try_exists().is_ok_and(|b| b) {
            return path;
        }

        path.set_extension("scen");
        if path.as_path().try_exists().is_ok_and(|b| b) {
            return path;
        }

        panic!("Could not find a default problem file for map with extensions .scenario or .scen");
    }
}

impl fmt::Display for Problems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!(
            "{} problems from {}:\n\n",
            self.problems.len(),
            self.file
        ));
        let problems: String = self.problems.iter().map(|p| p.to_string()).collect();
        result.push_str(&problems);
        writeln!(f, "{}", result)
    }
}
