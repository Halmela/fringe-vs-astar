use std::fmt;

/// Problem to be solved. Length is optional, because it might not always be present
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
    pub fn parse_problem(value: String, number: usize) -> anyhow::Result<Problem> {
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
        write!(f, "{}", result)
    }
}
