// use heuristic::Heuristic;

/// A* path finder
pub mod astar;
pub use crate::algorithms::astar::AStar;

/// Fringe search path finder
pub mod fringesearch;
pub use crate::algorithms::fringesearch::FringeSearch;

/// Runner for search algorithms
pub mod solver;
pub use crate::algorithms::solver::*;

use crate::{index_to_xy, Cost, Node, DIAGONAL_COST};

/// Enumerator for representing different stages of pathfinding.
pub enum State {
    Finished((Vec<Node>, Cost)),
    Processing(Node),
    Internal,
    NotFound,
}

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
    #[must_use]
    pub fn new(goal: Node, width: usize) -> Self {
        Heuristic {
            goal: index_to_xy(goal, width),
            width,
        }
    }

    /// Octile distance between two points
    #[must_use]
    pub fn calc(&self, node: Node) -> Cost {
        let start = index_to_xy(node, self.width);
        let x_distance: Cost = ((start.0 as Cost) - (self.goal.0 as Cost)).abs();
        let y_distance: Cost = ((start.1 as Cost) - (self.goal.1 as Cost)).abs();
        let distance: Cost = (x_distance - y_distance).abs();

        if x_distance > y_distance {
            distance + DIAGONAL_COST * y_distance
        } else {
            distance + DIAGONAL_COST * x_distance
        }
    }
}
