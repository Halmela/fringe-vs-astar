use crate::structures::map::Map;
use crate::Node;
use crate::{xy_to_index, DIAGONAL_COST};

/// Adjacency listed representation of a [`Map`].
#[derive(Clone)]
pub struct Graph {
    adjacency_list: Vec<Vec<(Node, f32)>>,
    height: usize,
    width: usize,
}
impl Graph {
    /// Constructor
    #[must_use] pub fn new(map: &Map) -> Graph {
        let mut adjacency_list: Vec<Vec<(Node, f32)>> =
            Vec::with_capacity(map.get_height() * map.get_width());

        for y in 0..map.get_height() {
            for x in 0..map.get_width() {
                adjacency_list.push(vec![]);
                let i = xy_to_index(x, y, map.get_width());
                if let Some(true) = map.get_cell(x, y) {
                    adjacency_list[i as usize] = generate_neighbors(x, y, map)
                        .drain(..)
                        .map(|((x1, y1), f)| (xy_to_index(x1, y1, map.get_width()), f))
                        .collect();
                }
            }
        }

        Graph {
            adjacency_list,
            height: map.get_height(),
            width: map.get_width(),
        }
    }
    /// Return neighbors of a `Node`
    pub fn neighbors(&self, i: Node) -> std::slice::Iter<'_, (Node, f32)> {
        self.adjacency_list[i as usize].iter()
    }

    /// Get height of map
    #[must_use] pub fn get_height(&self) -> usize {
        self.height
    }
    /// Get width of map
    #[must_use] pub fn get_width(&self) -> usize {
        self.width
    }

    /// Average branching factor of the graph.
    /// Only nodes with some neighbors are counted
    #[must_use] pub fn average_branching(&self) -> f32 {
        let (total, n) = self
            .adjacency_list
            .iter()
            .filter(|v| !v.is_empty())
            .map(std::vec::Vec::len)
            .fold((0, 0), |acc, l| (acc.0 + l, acc.1 + 1));

        total as f32 / n as f32
    }
}

/// Provide a list of neighbors for given cell in a grid.
/// Makes sure that path does not cut through corners of unpassable cells.
fn generate_neighbors(x: usize, y: usize, map: &Map) -> Vec<((usize, usize), f32)> {
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
        .and_then(|x1| y.checked_sub(1).map(|y1| (map.get_cell(x1, y1), (x1, y1))));

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
                .map(|(b, xy)| (b, xy, DIAGONAL_COST)),
        )
        .filter(|(b, _, _)| b.is_some_and(|b1| b1))
        .map(|(_, xy, w)| (xy, w))
        .collect::<Vec<((usize, usize), f32)>>()
}
