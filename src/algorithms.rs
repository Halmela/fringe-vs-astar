/// A* path finder
pub mod astar;
pub use crate::algorithms::astar::AStar;

/// Fringe path finder
pub mod fringe;
pub use crate::algorithms::fringe::FringeSearch;

use crate::DIAGONAL_COST;

/// Octile distance between two points
pub fn heuristic(start: (usize, usize), goal: (usize, usize)) -> f32 {
    let x_distance: f32 = ((start.0 as f32) - (goal.0 as f32)).abs();
    let y_distance: f32 = ((start.1 as f32) - (goal.1 as f32)).abs();

    if x_distance > y_distance {
        (x_distance - y_distance) + DIAGONAL_COST * y_distance
    } else {
        (y_distance - x_distance) + DIAGONAL_COST * x_distance
    }
}

#[cfg(test)]
mod tests {
    use crate::DIAGONAL_COST;

    use super::*;

    #[test]
    fn heuristic_works_diagonally() {
        let result = heuristic((0, 0), (1, 1));
        assert_eq!(DIAGONAL_COST, result);
    }
    #[test]
    fn heuristic_works_downwards() {
        let result = heuristic((0, 0), (0, 1));
        let expected = 1.0;
        assert_eq!(expected, result);
    }
}
