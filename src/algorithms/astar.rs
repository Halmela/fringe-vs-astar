use crate::algorithms::heuristic;
use crate::index_to_xy;
use crate::structures::{Frontier, Graph};

/// A* pathfinder
pub struct AStar<'a> {
    frontier: Frontier,
    cache: Vec<(usize, f64)>,
    start: usize,
    goal: usize,
    graph: &'a Box<dyn Graph>,
}

impl<'a> AStar<'a> {
    /// Create solver of a problem for a graph
    pub fn new(start: usize, goal: usize, graph: &'a Box<dyn Graph>) -> Self {
        let size = graph.get_width() * graph.get_height();
        let frontier = Frontier::new(start, size);

        // (previous, current cost)
        let mut cache: Vec<(usize, f64)> = std::iter::repeat((0, f64::MAX)).take(size).collect();
        cache[start] = (start, 0.0);

        AStar {
            frontier,
            cache,
            start,
            goal,
            graph,
        }
    }

    /// Try to solve the problem
    pub fn solve(mut self) -> Option<(Vec<usize>, f64)> {
        let ixy = |i: usize| index_to_xy(i, self.graph.get_width());
        let gxy = ixy(self.goal);
        let h = |i: usize| heuristic(ixy(i), gxy);

        while let Some(i) = self.frontier.pop() {
            if i == self.goal {
                return Some((self.construct_path(), self.cache[self.goal].1));
            }

            let current_cost = self.cache[i].1;

            for (n, w1) in self.graph.neighbors(i) {
                let new_cost = current_cost + w1;
                let priority = new_cost + h(n);
                if self.frontier.push(n, priority) {
                    self.cache[n] = (i, new_cost);
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
            let new = self.cache[i].0;
            path.push(new);

            if new == self.start {
                break;
            }
        }
        path.reverse();

        path
    }
}
