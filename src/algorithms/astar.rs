use super::Heuristic;
use crate::structures::AdjacencyListGraph;
use crate::Node;

use self::cache::Cache;
use self::frontier::Frontier;
use crate::algorithms::astar::weighted_cell::WeightedCell;

mod cache;
mod frontier;
mod weighted_cell;

/// A* pathfinder
pub struct AStar<'a> {
    frontier: Frontier,
    cache: Cache,
    start: Node,
    goal: Node,
    graph: &'a AdjacencyListGraph,
}

impl<'a> AStar<'a> {
    /// Create solver of a problem for a graph
    pub fn new(start: Node, goal: Node, graph: &'a AdjacencyListGraph) -> Self {
        let size = graph.get_width() * graph.get_height();
        let frontier = Frontier::new(start, size);

        let heuristic = Heuristic::new(goal, graph.get_width());
        let cache = Cache::new(start, heuristic, size);

        AStar {
            frontier,
            cache,
            start,
            goal,
            graph,
        }
    }

    /// Try to solve the problem
    pub fn solve(mut self) -> Option<(Vec<Node>, f32)> {
        while let Some(node) = self.frontier.pop() {
            if node == self.goal {
                return Some((self.construct_path(), self.cache.get_cost(self.goal)));
            }

            let current_cost = self.cache.get_cost(node);

            for (child, w1) in self.graph.neighbors(node) {
                if let Some((node, weight)) = self.cache.check(*child, node, current_cost + w1) {
                    self.frontier.push(node, weight);
                }
            }
        }
        // If frontier is empty, no path can be found
        None
    }

    /// Reconstruct path that was found
    fn construct_path(&self) -> Vec<Node> {
        let mut path = vec![self.goal];
        loop {
            let node = path[path.len() - 1];
            let new = self.cache[node].parent;
            path.push(new);

            if new == self.start {
                break;
            }
        }
        path.reverse();

        path
    }
}
