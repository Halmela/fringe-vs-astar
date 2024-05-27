/// A* path finder
pub mod astar;
pub use crate::algorithms::astar::AStar;

pub mod fringe;
pub use crate::algorithms::fringe::FringeSearch;

/// Diagonal octile distance from current node to goal.
/// This is a grid specific method.
fn heuristic(
    current_x: usize,
    current_y: usize,
    goal_x: usize,
    goal_y: usize,
    diagonal_cost: f64,
) -> f64 {
    let x_distance: f64 = ((current_x as f64) - (goal_x as f64)).abs();
    let y_distance: f64 = ((current_y as f64) - (goal_y as f64)).abs();

    if x_distance > y_distance {
        (x_distance - y_distance) + diagonal_cost * y_distance
    } else {
        (y_distance - x_distance) + diagonal_cost * x_distance
    }
}
