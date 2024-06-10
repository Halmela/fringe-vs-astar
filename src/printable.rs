use std::{collections::HashSet, fmt};

use crate::structures::map::ArrayMap;

#[derive(Clone, Copy)]
pub enum Cell {
    Open,
    Wall,
    Start,
    Goal,
    Path,
}

pub struct Printable {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Printable {
    pub fn new(map: &ArrayMap) -> Self {
        let mut grid = vec![vec!(Cell::Wall; map.get_width()); map.get_height()];

        for y in 0..map.get_height() {
            for x in 0..map.get_width() {
                if let Some(true) = map.get_cell(x, y) {
                    grid[y][x] = Cell::Open;
                }
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
    pub fn add_path(&mut self, path: HashSet<(usize, usize)>) {
        for (x, y) in path {
            self.grid[y][x] = Cell::Path;
        }
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
                });
            }
            result.push('\n');
        }
        writeln!(f, "{}", result)
    }
}
