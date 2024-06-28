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
    #[must_use]
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
            headers: Vec::default(),
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
        self.add_start(problem.start);
        self.add_goal(problem.goal);
        self.add_header("Problem", format!("{}", problem.number));
        self.add_header("", problem.coordinates());
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
    pub fn add_current(&mut self, node: Node, cost: f32, estimate: f32) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::Current;

        self.add_header("Current", "");
        self.add_header("  node", format!("{node}\t ({x}, {y})"));
        self.add_header("  cost", cost);
        self.add_header("  estimate", estimate);
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

    fn _big_map(&self) -> String {
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
            .fold(String::new(), |acc, (k, v)| format!("{acc}{k:<10} {v}\n"))
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
            Cell::InOpen | Cell::Second => '❔',
            Cell::InLater | Cell::Third => '❓',
            Cell::OldLater | Cell::InFrontier => '⭕',
            Cell::InClosed => '✅',
            Cell::First => '❕',
        }
    }
}

impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.print_map {
            return write!(f, "#\n{}", self.headers());
        }

        let result = self
            .grid
            .iter()
            .map(|row| row.iter().map(|cell| char::from(*cell)).collect())
            .chain(repeat("➖".repeat(self.width)))
            .zip(
                self.headers
                    .iter()
                    .map(|(k, v)| format!("\t{k:<10} {v}\n"))
                    .chain(std::iter::repeat(String::from("\n"))),
            )
            .take_while(|(row, header)| !(row.contains('➖') && header == "\n"))
            .fold(String::new(), |acc, (row, header)| {
                format!("{acc}{row}{header}")
            });

        write!(f, "#\n{result}")
    }
}
