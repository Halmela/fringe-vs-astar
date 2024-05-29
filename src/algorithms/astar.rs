use crate::algorithms::heuristic;
use crate::structures::{Frontier, Graph};

/// A* pathfinder
pub struct AStar<'a> {
    frontier: Frontier,
    history: Vec<Vec<(Option<(usize, usize)>, Option<f64>)>>,
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    graph: &'a Box<dyn Graph>,
}

impl<'a> AStar<'a> {
    /// Create solver of a problem for a graph
    pub fn new(
        start_x: usize,
        start_y: usize,
        goal_x: usize,
        goal_y: usize,
        graph: &'a Box<dyn Graph>,
    ) -> Self {
        let frontier = Frontier::new(start_x, start_y, graph.get_width(), graph.get_height());

        // (previous xy, current cost, current prority)
        let mut history: Vec<Vec<(Option<(usize, usize)>, Option<f64>)>> = vec![];
        for x in 0..graph.get_width() {
            history.push(vec![]);
            for _ in 0..graph.get_height() {
                history[x].push((None, None));
            }
        }

        history[start_x][start_y] = (None, Some(0.0));

        AStar {
            frontier,
            history,
            start_x,
            start_y,
            goal_x,
            goal_y,
            graph,
        }
    }

    /// Try to solve the problem
    pub fn solve(mut self) -> Option<(Vec<(usize, usize)>, f64)> {
        let h = |x: usize, y: usize| heuristic(x, y, self.goal_x, self.goal_y);

        while let Some((x, y)) = self.frontier.pop() {
            if x == self.goal_x && y == self.goal_y {
                return Some((
                    self.construct_path(),
                    self.history[self.goal_x][self.goal_y].1.unwrap(),
                ));
            }

            let current_cost = self.history[x][y].1.unwrap();

            for ((x1, y1), w1) in self.graph.neighbors(x, y) {
                let new_cost = current_cost + w1;
                let priority = new_cost + h(x1, y1);
                if self.frontier.push(x1, y1, priority) {
                    self.history[x1][y1] = (Some((x, y)), Some(new_cost));
                }
            }
        }
        // If frontier is empty, no path can be found
        None
    }

    /// Reconstruct path that was found
    fn construct_path(&self) -> Vec<(usize, usize)> {
        let mut path = vec![(self.goal_x, self.goal_y)];
        loop {
            let (x, y) = path[path.len() - 1];
            let new = self.history[x][y].0.unwrap();
            path.push(new);

            if (new) == (self.start_x, self.start_y) {
                break;
            }
        }
        path.reverse();

        path
    }
}

#[cfg(test)]
mod tests {
    use crate::DIAGONAL_COST;

    use super::*;

    #[test]
    fn heuristic_works_diagonally() {
        let result = heuristic(0, 0, 1, 1);
        assert_eq!(DIAGONAL_COST, result);
    }
    #[test]
    fn heuristic_works_downwards() {
        let result = heuristic(0, 0, 0, 1);
        let expected = 1.0;
        assert_eq!(expected, result);
    }
}
