use crate::map::*;
use std::collections::HashMap;
use std::fmt;

/// Representation of terrain map as a graph
pub trait Graph {
    /// Constructor
    fn new(map: Box<dyn Map>) -> Self;
    /// Get neighbors of a node.
    /// Making it a `usize` is something I have to consider again.
    /// Returns a vector of nodes with their weight
    fn neighbors(&self, x: usize, y: usize) -> Option<&Vec<((usize, usize), f32)>>;
    /// Map height
    fn get_height(&self) -> usize;
    /// Map width
    fn get_width(&self) -> usize;
}

/// Graph represented as a HashMap with key `(x,y)`
#[derive(Debug)]
pub struct AdjacencyMapGraph {
    adjacency_map: HashMap<(usize, usize), Vec<((usize, usize), f32)>>,
    height: usize,
    width: usize,
}

impl Graph for AdjacencyMapGraph {
    fn new(map: Box<dyn Map>) -> AdjacencyMapGraph {
        let diagonal_cost = 2.0_f32.sqrt();

        let mut adjacency_map = HashMap::new();

        for y in 0..map.get_height() {
            for x in 0..map.get_width() {
                if let Some(true) = map.get_cell(x, y) {
                    adjacency_map.insert((x, y), generate_neighbors(x, y, &map, diagonal_cost));
                }
            }
        }

        AdjacencyMapGraph {
            adjacency_map,
            height: map.get_height(),
            width: map.get_width(),
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Option<&Vec<((usize, usize), f32)>> {
        self.adjacency_map.get(&(x, y))
    }

    fn get_height(&self) -> usize {
        self.height
    }
    fn get_width(&self) -> usize {
        self.width
    }
}

impl fmt::Display for AdjacencyMapGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (cell, neighbors) in &self.adjacency_map {
            result.push_str(&format!("{:?} {:?}\n", cell, neighbors));
        }
        write!(f, "{}", result)
    }
}

/// Graph represented as a 3D matrix that can be keyed with x,y
#[derive(Debug)]
pub struct AdjacencyGridGraph {
    adjacency_grid: Vec<Vec<Vec<((usize, usize), f32)>>>,
    height: usize,
    width: usize,
}

impl Graph for AdjacencyGridGraph {
    fn new(map: Box<dyn Map>) -> AdjacencyGridGraph {
        // So that the value is same for everyone
        let diagonal_cost = 2.0_f32.sqrt();

        let mut adjacency_grid: Vec<Vec<Vec<((usize, usize), f32)>>> =
            Vec::with_capacity(map.get_height());

        for y in 0..map.get_height() {
            adjacency_grid.push(vec![]);
            for x in 0..map.get_width() {
                adjacency_grid[y].push(vec![]);
                if let Some(true) = map.get_cell(x, y) {
                    adjacency_grid[y][x] = generate_neighbors(x, y, &map, diagonal_cost);
                }
            }
        }

        AdjacencyGridGraph {
            adjacency_grid,
            height: map.get_height(),
            width: map.get_width(),
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Option<&Vec<((usize, usize), f32)>> {
        Some(&self.adjacency_grid[y][x])
    }
    fn get_height(&self) -> usize {
        self.height
    }
    fn get_width(&self) -> usize {
        self.width
    }
}

impl fmt::Display for AdjacencyGridGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                result.push_str(&format!("{:?} {:?}\n", (x, y), self.adjacency_grid[y][x]));
            }
        }
        write!(f, "{}", result)
    }
}

/// Provide a list of neighbors for given cell in a grid.
/// Makes sure that path does not cut through corners of unpassable cells.
fn generate_neighbors(
    x: usize,
    y: usize,
    map: &Box<dyn Map>,
    diagonal_cost: f32,
) -> Vec<((usize, usize), f32)> {
    // We are dealing with usize here, so x-1 will always be checked to avoid underflow errors.
    // Side-effects of this are that the coordinates will be correct and this can be harder to read.

    // (x, y-1)
    let north = y.checked_sub(1).map(|y1| (map.get_cell(x, y1), (x, y1)));
    // (x+1, y)
    let east = Some((map.get_cell(x + 1, y), (x + 1, y)));
    // (x, y+1)
    let south = Some((map.get_cell(x, y + 1), (x, y + 1)));
    // (x-1, y)
    let west = x.checked_sub(1).map(|x1| (map.get_cell(x1, y), (x1, y)));

    // (x+1, y-1)
    let mut north_east = y
        .checked_sub(1)
        .map(|y1| (map.get_cell(x + 1, y1), (x + 1, y1)));
    // (x+1, y+1)
    let mut south_east = Some((map.get_cell(x + 1, y + 1), (x + 1, y + 1)));
    // (x-1, y+1)
    let mut south_west = x
        .checked_sub(1)
        .map(|x1| (map.get_cell(x1, y + 1), (x1, y + 1)));
    // (x-1, y-1)
    let mut north_west = x
        .checked_sub(1)
        .map(|x1| y.checked_sub(1).map(|y1| (map.get_cell(x1, y1), (x1, y1))))
        .flatten();

    match (north, east, north_east) {
        (Some((Some(true), _)), Some((Some(true), _)), Some((Some(true), _))) => {}
        _ => {
            north_east = None;
        }
    }
    match (south, east, south_east) {
        (Some((Some(true), _)), Some((Some(true), _)), Some((Some(true), _))) => {}
        _ => {
            south_east = None;
        }
    }
    match (south, west, south_west) {
        (Some((Some(true), _)), Some((Some(true), _)), Some((Some(true), _))) => {}
        _ => {
            south_west = None;
        }
    }
    match (north, west, north_west) {
        (Some((Some(true), _)), Some((Some(true), _)), Some((Some(true), _))) => {}
        _ => {
            north_west = None;
        }
    }

    vec![north, east, south, west]
        .drain(..)
        .flatten()
        .map(|(b, xy)| (b, xy, 1.0))
        .chain(
            vec![north_east, south_east, south_west, north_west]
                .drain(..)
                .flatten()
                .map(|(b, xy)| (b, xy, diagonal_cost)),
        )
        .filter(|(b, _, _)| b.is_some_and(|b1| b1))
        .map(|(_, xy, w)| (xy, w))
        .collect::<Vec<((usize, usize), f32)>>()
}

/*
/// Graph where each node has its neighbors as a vector
struct AdjacencyListedGraph {
    adjacency_list: Vec<Vec<(usize, f32)>>,
    width: usize,
}


impl Graph for AdjacencyListedGraph {
    fn new<M: Map>(map: M) -> AdjacencyListedGraph {
        let diagonal_cost = 2.0_f32.sqrt();

        let mut adjacency_list: Vec<Vec<(usize, f32)>> =
            vec![vec!(); map.get_height() * map.get_width()];

        let to_index = |x: usize, y: usize| y * map.get_width() + x;

        let some_to_index =
            |x: Option<usize>, y: Option<usize>| x.and_then(|x1| y.map(|y1| to_index(x1, y1)));

        let neighbors = |x: usize, y: usize| {
            vec![
                //(x - 1, y - 1),
                some_to_index(x.checked_sub(1), y.checked_sub(1)).map(|i| (i, diagonal_cost)),
                // (x, y - 1),
                some_to_index(Some(x), y.checked_sub(1)).map(|i| (i, 1.0)),
                // (x + 1, y - 1),
                some_to_index(Some(x + 1), y.checked_sub(1)).map(|i| (i, diagonal_cost)),
                // (x - 1, y),
                some_to_index(x.checked_sub(1), Some(y)).map(|i| (i, 1.0)),
                Some((to_index(x + 1, y), 1.0)),
                //(x - 1, y + 1),
                some_to_index(x.checked_sub(1), Some(y + 1)).map(|i| (i, diagonal_cost)),
                Some((to_index(x, y + 1), 1.0)),
                Some((to_index(x + 1, y + 1), diagonal_cost)),
            ]
            .drain(..)
            .flatten()
            .collect()
        };

        for y in 0..map.get_height() {
            for x in 0..map.get_width() {
                if let Some(true) = map.get_cell(x, y) {
                    adjacency_list.push(neighbors(x, y));
                }
            }
        }

        AdjacencyListedGraph {
            adjacency_list,
            width: map.get_width(),
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> &Vec<((usize, usize), f32)> {
        &self.adjacency_list[self.index(x, y)]
    }
}
*/
