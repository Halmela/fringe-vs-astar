use std::fmt;
use std::iter::repeat;

use crate::{index_to_xy, problem::Problem, structures::map::Map, Node};

#[derive(Clone, Copy)]
pub enum Cell {
    Open,
    Wall,
    Start,
    Goal,
    Path,
    Current,
    InOpen,
    InClosed,
    InLater,
    OldLater,
    First,
    Second,
    Third,
    InFrontier,
}

#[derive(Clone)]
pub struct Printable {
    pub grid: Vec<Vec<Cell>>,
    headers: Vec<(String, String)>,
    pub width: usize,
    print_map: bool,
}

impl Printable {
    pub fn new(map: &Map) -> Self {
        let mut grid = vec![vec!(Cell::Wall; map.get_width()); map.get_height()];

        for (y, x) in
            (0..map.get_height()).flat_map(|y| std::iter::repeat(y).zip(0..map.get_width()))
        {
            if let Some(true) = map.get_cell(x, y) {
                grid[y][x] = Cell::Open;
            }
        }

        Printable {
            grid,
            width: map.get_width(),
            headers: Default::default(),
            print_map: true,
        }
    }

    pub fn add_goal(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::Goal;
    }
    pub fn add_start(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::Start;
    }

    pub fn add_problem(&mut self, problem: &Problem) {
        self.grid[problem.start_y][problem.start_x] = Cell::Start;
        self.grid[problem.goal_y][problem.goal_x] = Cell::Goal;
        self.add_header("Problem", format!("{}", problem.number));
        self.add_header(
            "",
            format!(
                "({}, {}) -> ({}, {})",
                problem.start_x, problem.start_y, problem.goal_x, problem.goal_y
            ),
        );
        if let Some(l) = problem.length {
            self.add_header("Expected", l);
        }
    }
    pub fn add_path(&mut self, mut path: Vec<Node>) {
        let path: Vec<(usize, usize)> = path
            .drain(..)
            .map(|i| index_to_xy(i, self.width))
            .filter(|(x, y)| !matches!(self.grid[*y][*x], Cell::Start | Cell::Goal))
            .collect();

        for (x, y) in path {
            self.grid[y][x] = Cell::Path;
        }
    }
    pub fn add_current(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::Current;
    }
    pub fn add_inopen(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::InOpen;
    }
    pub fn add_inclosed(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::InClosed;
    }
    pub fn add_inlater(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::InLater;
    }
    pub fn add_oldlater(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::OldLater;
    }
    pub fn add_first(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::First;
    }
    pub fn add_second(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::Second;
    }
    pub fn add_third(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::Third;
    }
    pub fn add_infrontier(&mut self, node: Node) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::InFrontier;
    }

    pub fn add_header<T: ToString, U: ToString>(&mut self, key: T, value: U) {
        self.headers.push((key.to_string(), value.to_string()));
    }
    pub fn add_spacing(&mut self) {
        self.headers.push((String::new(), String::new()));
    }

    fn map_longer_than_headers(&self) -> String {
        self.grid
            .iter()
            .zip(
                self.headers
                    .iter()
                    .map(|(k, v)| format!("\t{:<10} {}\n", k, v))
                    .chain(std::iter::repeat(String::from("\n"))),
            )
            .flat_map(|(row, header)| {
                row.iter()
                    .map(|cell| char::from(*cell))
                    .chain(header.chars().collect::<Vec<char>>())
            })
            .collect()
    }

    fn headers_longer_than_map(&self) -> String {
        self.headers
            .iter()
            .map(|(k, v)| format!("\t{:<10} {}\n", k, v))
            .zip(
                self.grid
                    .iter()
                    .map(|row| row.iter().map(|cell| char::from(*cell)).collect::<String>())
                    .chain(repeat(repeat('➖').take(self.width).collect::<String>())),
            )
            .map(|(header, row)| format!("{row}{header}"))
            .collect()
    }

    fn big_map(&self) -> String {
        let map: String = self
            .grid
            .iter()
            .flat_map(|row| {
                row.iter()
                    .map(|cell| char::from(*cell))
                    .chain(std::iter::once('\n'))
            })
            .collect();

        format!("{map}\n{}", self.headers())
    }

    fn headers(&self) -> String {
        self.headers
            .iter()
            .map(|(k, v)| format!("{:<10} {}\n", k, v))
            .collect()
    }

    pub fn suppress_print(&mut self) {
        self.print_map = false;
    }
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Open => '⬛',
            Cell::Wall => '⬜',
            Cell::Start => '🏁',
            Cell::Goal => '🏆',
            Cell::Path => '🟦',
            Cell::Current => '🟪',
            Cell::InOpen => '❔',
            Cell::InLater => '❓',
            Cell::OldLater => '🚫',
            Cell::InClosed => '✅',
            Cell::First => '❕',
            Cell::Second => '❔',
            Cell::Third => '❓',
            Cell::InFrontier => '⭕',
        }
    }
}

impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.print_map {
            return write!(f, "{}", self.headers());
        }
        if self.width > 80 {
            return write!(f, "{}", self.big_map());
        }
        if self.grid.len() > self.headers.len() {
            write!(f, "{}", self.map_longer_than_headers())
        } else {
            write!(f, "{}", self.headers_longer_than_map())
        }
    }
}
