use crate::index_to_xy;
use crate::DIAGONAL_COST;

#[derive(Clone)]
pub struct Heuristic {
    goal_x: f64,
    goal_y: f64,
    cache: Vec<Option<f64>>,
    width: usize,
    p: f64,
}

impl Heuristic {
    pub fn new(goal_x: usize, goal_y: usize, width: usize, height: usize) -> Self {
        let cache = std::iter::repeat(None).take(width * height).collect();

        Heuristic {
            goal_x: goal_x as f64,
            goal_y: goal_y as f64,
            cache,
            width,
            p: 1.0 / ((width * height) as f64),
        }
    }

    /// Diagonal octile distance from current node to goal.
    /// This is a grid specific method.
    pub fn distance_to_goal(&self, i: usize) -> f64 {
        if let Some(distance) = self.cache[i] {
            return distance;
        }

        let (x, y) = index_to_xy(i, self.width);

        let x_distance: f64 = ((x as f64) - self.goal_x).abs();
        let y_distance: f64 = ((y as f64) - self.goal_y).abs();

        if x_distance > y_distance {
            ((x_distance - y_distance) + DIAGONAL_COST * y_distance) * self.p
        } else {
            ((y_distance - x_distance) + DIAGONAL_COST * x_distance) * self.p
        }
    }
}
