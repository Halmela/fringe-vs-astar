use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Problem to be solved. Length is optional, because it might not always be present
#[derive(Clone, Copy)]
pub struct Problem {
    pub start_x: usize,
    pub start_y: usize,
    pub goal_x: usize,
    pub goal_y: usize,
    pub length: Option<f64>,
    pub number: usize,
}

impl Problem {
    /// Create problem
    pub fn new(
        start_x: usize,
        start_y: usize,
        goal_x: usize,
        goal_y: usize,
        length: Option<f64>,
        number: usize,
    ) -> Problem {
        Problem {
            start_x,
            start_y,
            goal_x,
            goal_y,
            length,
            number,
        }
    }

    /// Parse `.scenario` row as a problem
    pub fn parse(value: String, number: usize) -> anyhow::Result<Problem> {
        let fields: Vec<&str> = value.split_ascii_whitespace().collect();
        let wanted = fields.split_at(fields.len() - 5).1;

        Ok(Problem {
            start_x: wanted[0].parse()?,
            start_y: wanted[1].parse()?,
            goal_x: wanted[2].parse()?,
            goal_y: wanted[3].parse()?,
            length: Some(wanted[4].parse()?),
            number,
        })
    }

    pub fn from_file(problem_file: &PathBuf, problem: usize) -> anyhow::Result<Problem> {
        let f = File::open(problem_file)?;
        let mut content = BufReader::new(f).lines();

        Problem::parse(content.nth(problem).unwrap()?, problem)
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("Problem {}:\n", self.number));
        result.push_str(&format!("\t({}, {}) -> ", self.start_x, self.start_y));
        result.push_str(&format!("({}, {})", self.goal_x, self.goal_y));
        if let Some(l) = self.length {
            result.push_str(&format!("\t{l}"));
        }
        writeln!(f, "{}", result)
    }
}

pub struct Problems {
    problems: Vec<Problem>,
    file: String,
}

impl Problems {
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

    pub fn is_empty(&self) -> bool {
        self.problems.is_empty()
    }

    pub fn get(&self, i: usize) -> Option<&Problem> {
        self.problems.get(i)
    }

    pub fn len(&self) -> usize {
        self.problems.len()
    }

    pub fn single_problem(&mut self) -> Option<Problem> {
        if self.problems.len() == 1 {
            self.problems.pop()
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Problem> {
        self.problems.iter()
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
