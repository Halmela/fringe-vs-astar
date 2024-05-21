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
}

/// Graph represented as a HashMap with key `(x,y)`
#[derive(Debug)]
pub struct AdjacencyMapGraph {
    adjacency_map: HashMap<(usize, usize), Vec<((usize, usize), f32)>>,
}

impl Graph for AdjacencyMapGraph {
    fn new(map: Box<dyn Map>) -> AdjacencyMapGraph {
        let diagonal_cost = 2.0_f32.sqrt();

        let mut adjacency_map = HashMap::new();

        let some_tuple = |x: Option<usize>, y: Option<usize>| x.and_then(|x1| y.map(|y1| (x1, y1)));

        let neighbors = |x: usize, y: usize| {
            vec![
                // (x-1, y-1)
                some_tuple(x.checked_sub(1), y.checked_sub(1)).map(|i| (i, diagonal_cost)),
                // (x, y-1)
                some_tuple(Some(x), y.checked_sub(1)).map(|i| (i, 1.0)),
                // (x+1, y-1)
                some_tuple(Some(x + 1), y.checked_sub(1)).map(|i| (i, diagonal_cost)),
                // (x+1, y)
                Some(((x + 1, y), 1.0)),
                // (x+1, y+1)
                Some(((x + 1, y + 1), diagonal_cost)),
                // (x, y+1)
                Some(((x, y + 1), 1.0)),
                // (x-1, y+1),
                some_tuple(x.checked_sub(1), Some(y + 1)).map(|i| (i, diagonal_cost)),
                // (x-1, y),
                some_tuple(x.checked_sub(1), Some(y)).map(|i| (i, 1.0)),
            ]
            .drain(..)
            .flatten()
            .filter(|((x, y), _)| matches!(map.get_cell(*x, *y), Some(true)))
            .collect()
        };

        for y in 0..map.get_height() {
            for x in 0..map.get_width() {
                if let Some(true) = map.get_cell(x, y) {
                    adjacency_map.insert((x, y), neighbors(x, y));
                }
            }
        }

        AdjacencyMapGraph { adjacency_map }
    }

    fn neighbors(&self, x: usize, y: usize) -> Option<&Vec<((usize, usize), f32)>> {
        self.adjacency_map.get(&(x, y))
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
