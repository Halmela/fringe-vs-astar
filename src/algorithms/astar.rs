use crate::algorithms::heuristic;
use crate::structures::{Frontier, Graph};
use crate::{index_to_xy, xy_to_index};

/// A* pathfinder
pub struct AStar<'a> {
    frontier: Frontier,
    history: Vec<(usize, f64)>,
    start: usize,
    goal: usize,
    graph: &'a Box<dyn Graph>,
}

impl<'a> AStar<'a> {
    /// Create solver of a problem for a graph
    pub fn new(start: usize, goal: usize, graph: &'a Box<dyn Graph>) -> Self {
        let size = graph.get_width() * graph.get_height();
        let frontier = Frontier::new(start, size);

        // (previous xy, current cost, current prority)
        let mut history: Vec<(usize, f64)> = vec![];
        for i in 0..size {
            history.push((i, f64::MAX));
        }

        history[start] = (start, 0.0);

        AStar {
            frontier,
            history,
            start,
            goal,
            graph,
        }
    }

    /// Try to solve the problem
    pub fn solve(mut self) -> Option<(Vec<usize>, f64)> {
        let ixy = |i: usize| index_to_xy(i, self.graph.get_width());
        let (goal_x, goal_y) = ixy(self.goal);
        let h = |i: usize| {
            let (x, y) = ixy(i);
            heuristic(x, y, goal_x, goal_y)
        };

        while let Some(i) = self.frontier.pop() {
            if i == self.goal {
                return Some((self.construct_path(), self.history[self.goal].1));
            }

            let current_cost = self.history[i].1;

            for (n, w1) in self.graph.neighbors(i) {
                let new_cost = current_cost + w1;
                let priority = new_cost + h(n);
                if self.frontier.push(n, priority) {
                    self.history[n] = (i, new_cost);
                }
            }
        }
        // If frontier is empty, no path can be found
        None
    }

    /// Reconstruct path that was found
    fn construct_path(&self) -> Vec<usize> {
        let mut path = vec![self.goal];
        loop {
            let i = path[path.len() - 1];
            let new = self.history[i].0;
            path.push(new);

            if new == self.start {
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
