use crate::index_to_xy;
use crate::DIAGONAL_COST;

#[derive(Clone)]
pub struct Heuristic {
    goal_x: f32,
    goal_y: f32,
    cache: Vec<Option<f32>>,
    width: usize,
    p: f32,
}

impl Heuristic {
    pub fn new(goal: Node, width: usize, height: usize) -> Self {
        let cache = std::iter::repeat(None).take(width * height).collect();
        let (goal_x, goal_y) = index_to_xy(goal, width);

        Heuristic {
            goal_x: goal_x as f32,
            goal_y: goal_y as f32,
            cache,
            width,
        }
    }

    /// Diagonal octile distance from current node to goal.
    /// This is a grid specific method.
    pub fn distance_to_goal(&self, i: Node) -> f32 {
        if let Some(distance) = self.cache[i as usize] {
            return distance;
        }

        let (x, y) = index_to_xy(i, self.width);

        let x_distance: f32 = ((x as f32) - self.goal_x).abs();
        let y_distance: f32 = ((y as f32) - self.goal_y).abs();

        if x_distance > y_distance {
            ((x_distance - y_distance) + DIAGONAL_COST * y_distance)
        } else {
            ((y_distance - x_distance) + DIAGONAL_COST * x_distance)
        }
    }
}
