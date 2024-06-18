use std::{collections::HashSet, fmt};

use crate::{index_to_xy, problem::Problem, structures::map::ArrayMap, Node};

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
}

#[derive(Clone)]
pub struct Printable {
    pub grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Printable {
    pub fn new(map: &ArrayMap) -> Self {
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
            height: map.get_height(),
        }
    }

    pub fn add_goal(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Cell::Goal;
    }
    pub fn add_start(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Cell::Start;
    }

    pub fn add_problem(&mut self, problem: &Problem) {
        self.grid[problem.start_y][problem.start_x] = Cell::Start;
        self.grid[problem.goal_y][problem.goal_x] = Cell::Goal;
    }
    pub fn add_path(&mut self, path: HashSet<(usize, usize)>) {
        for (x, y) in path {
            self.grid[y][x] = Cell::Path;
        }
    }
    pub fn add_current(&mut self, (x, y): (usize, usize)) {
        self.grid[y][x] = Cell::Current;
    }
    pub fn add_inopen(&mut self, node: usize) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::InOpen;
    }
    pub fn add_inclosed(&mut self, node: usize) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::InClosed;
    }
    pub fn add_inlater(&mut self, node: usize) {
        let (x, y) = index_to_xy(node, self.width);
        self.grid[y][x] = Cell::InLater;
    }
}

impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                result.push(match self.grid[y][x] {
                    Cell::Open => '⬛',
                    Cell::Wall => '⬜',
                    Cell::Start => '🏁',
                    Cell::Goal => '🏆',
                    Cell::Path => '🟩',
                    Cell::Current => '🟪',
                    Cell::InOpen => '❓',
                    Cell::InClosed => '✅',
                    Cell::InLater => '⭕',
                });
            }
            result.push('\n');
        }
        writeln!(f, "{}", result)
    }
}
