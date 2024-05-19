use crate::map::*;

/// Representation of terrain map as a graph
pub trait Graph {
    /// Constructor
    fn new<M: Map>(map: M) -> impl Graph;
    /// Get neighbors of a node.
    /// Making it a `usize` is something I have to consider again.
    /// Returns a vector of nodes with their weight
    fn neighbors(&self, node: usize) -> &Vec<(usize, f32)>;
}

/// Graph where each node has its neighbors as a vector
struct AdjacencyListedGraph {
    adjacency_list: Vec<Vec<(usize, f32)>>,
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

        AdjacencyListedGraph { adjacency_list }
    }

    fn neighbors(&self, id: usize) -> &Vec<(usize, f32)> {
        &self.adjacency_list[id]
    }
}
