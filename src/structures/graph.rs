use std::vec::Vec;

use crate::structures::map::Map;
use crate::Node;
use crate::DIAGONAL_COST;

/// Adjacency listed representation of a [`Map`].
#[derive(Clone)]
pub struct Graph {
    adjacency_list: Vec<Vec<(Node, f32)>>,
    height: usize,
    width: usize,
}
impl Graph {
    /// Constructor
    #[must_use]
    pub fn new(map: Map) -> Graph {
        Graph {
            adjacency_list: map
                .iter()
                .zip(0..)
                .map(|(b, i)| generate_neighbors(i, b, &map))
                .collect(),
            height: map.get_height(),
            width: map.get_width(),
        }
    }
    /// Return neighbors of a `Node`
    pub fn neighbors(&self, i: Node) -> std::slice::Iter<'_, (Node, f32)> {
        self.adjacency_list[i as usize].iter()
    }

    /// Get height of map
    #[must_use]
    pub fn get_height(&self) -> usize {
        self.height
    }
    /// Get width of map
    #[must_use]
    pub fn get_width(&self) -> usize {
        self.width
    }
    /// Get size of map
    #[must_use]
    pub fn map_size(&self) -> usize {
        self.width * self.height
    }

    /// Get size of graph
    pub fn size(&self) -> usize {
        self.adjacency_list.iter().filter(|a| !a.is_empty()).count()
    }

    /// Average branching factor of the graph.
    /// Only nodes with some neighbors are counted
    #[must_use]
    pub fn average_branching(&self) -> f32 {
        let (total, n) = self
            .adjacency_list
            .iter()
            .filter(|v| !v.is_empty())
            .map(Vec::len)
            .fold((0, 0), |acc, l| (acc.0 + l, acc.1 + 1));

        total as f32 / n as f32
    }
}

/// Provide a list of neighbors for given cell in a grid.
/// Makes sure that path does not cut through corners of unpassable cells.
fn generate_neighbors(node: Node, generate: &bool, map: &Map) -> Vec<(Node, f32)> {
    /*
       |--|--|--|    |--|--|--|
       |-4|-3|-2|    | 0| 1| 2|
       |--|--|--|    |--|--|--|
       |-1| 0| 1| -> | 3| 4| 5|
       |--|--|--|    |--|--|--|
       | 2| 3| 4|    | 6| 7| 8|
       |--|--|--|    |--|--|--|

    */

    if !generate {
        return vec![];
    }

    let n = node as i32;
    let w = map.get_width() as i32;
    let mut v: Vec<(i32, f32, bool)> = [
        n - w - 1, // 0
        n - w,     // 1
        n - w + 1, // 2
        n - 1,     // 3
        n,         // 4
        n + 1,     // 5
        n + w - 1, // 6
        n + w,     // 7
        n + w + 1, // 8
    ]
    .iter()
    .map(|i| (*i, 1.0, map.get(*i)))
    .collect();

    // Prevent wrapping around when in border of map
    v[3].2 = v[3].2 && v[4].0 % w != 0;
    v[5].2 = v[5].2 && v[4].0 % w != w - 1;

    // Check for passable diagonals
    v[0].2 = v[0].2 && v[1].2 && v[3].2;
    v[2].2 = v[2].2 && v[1].2 && v[5].2;
    v[6].2 = v[6].2 && v[3].2 && v[7].2;
    v[8].2 = v[8].2 && v[5].2 && v[7].2;
    v[4].2 = false;
    v[0].1 = DIAGONAL_COST;
    v[2].1 = DIAGONAL_COST;
    v[6].1 = DIAGONAL_COST;
    v[8].1 = DIAGONAL_COST;

    v.iter()
        .filter(|(_, _, b)| *b)
        .map(|(i, c, _)| (TryInto::<Node>::try_into(*i).unwrap(), *c))
        .collect()
}
