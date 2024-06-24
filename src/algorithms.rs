// use heuristic::Heuristic;

/// A* path finder
pub mod astar;
pub use crate::algorithms::astar::AStar;

/// Fringe path finder
pub mod fringesearch;
pub use crate::algorithms::fringesearch::FringeSearch;

use crate::{index_to_xy, Node, DIAGONAL_COST};

/// Octile distance calculator for a set goal node.
/// ```
/// // ... | 012
/// // .g. | 345
/// // ... | 678
/// use fringe_vs_astar::DIAGONAL_COST;
/// # use fringe_vs_astar::algorithms::Heuristic;
///
/// let h = Heuristic::new(4, 3);    
///
/// assert_eq!(h.calc(3), 1.0);
/// assert_eq!(h.calc(2), DIAGONAL_COST);
/// ```
pub struct Heuristic {
    goal: (usize, usize),
    width: usize,
}

impl Heuristic {
    /// Initialize self with goal and width
    pub fn new(goal: Node, width: usize) -> Self {
        Heuristic {
            goal: index_to_xy(goal, width),
            width,
        }
    }

    /// Octile distance between two points
    pub fn calc(&self, node: Node) -> f32 {
        let start = index_to_xy(node, self.width);
        let x_distance: f32 = ((start.0 as f32) - (self.goal.0 as f32)).abs();
        let y_distance: f32 = ((start.1 as f32) - (self.goal.1 as f32)).abs();

        if x_distance > y_distance {
            (x_distance - y_distance) + DIAGONAL_COST * y_distance
        } else {
            (y_distance - x_distance) + DIAGONAL_COST * x_distance
        }
    }
}
