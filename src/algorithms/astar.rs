use crate::algorithms::heuristic;
use crate::structures::AdjacencyListGraph;
use crate::{index_to_xy, Node};

use self::frontier::Frontier;
use crate::algorithms::astar::weighted_cell::WeightedCell;

mod frontier;
mod weighted_cell;

/// A* pathfinder
pub struct AStar<'a> {
    frontier: Frontier,
    cache: Vec<(Node, f32, Option<f32>)>,
    start: Node,
    goal: Node,
    graph: &'a AdjacencyListGraph,
}

impl<'a> AStar<'a> {
    /// Create solver of a problem for a graph
    pub fn new(start: Node, goal: Node, graph: &'a AdjacencyListGraph) -> Self {
        let size = graph.get_width() * graph.get_height();
        let frontier = Frontier::new(start, size);

        // (previous, current cost)
        let mut cache: Vec<(Node, f32, Option<f32>)> = vec![(0, f32::MAX, None); size];
        cache[start as usize] = (start, 0.0, None);

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
        let ixy = |i: Node| index_to_xy(i, self.graph.get_width());
        let gxy = ixy(self.goal);
        let h = |i: Node| heuristic(ixy(i), gxy);

        while let Some(node) = self.frontier.pop() {
            if node == self.goal {
                return Some((self.construct_path(), self.cache[self.goal as usize].1));
            }

            let current_cost = self.cache[node as usize].1;

            for (child, w1) in self.graph.neighbors(node) {
                let to_goal: f32;
                if let Some(tg) = self.cache[*child as usize].2 {
                    to_goal = tg;
                } else {
                    to_goal = h(*child);
                    self.cache[*child as usize].2 = Some(to_goal);
                }

                let new_cost = current_cost + w1;
                let priority = new_cost + to_goal;
                if self.frontier.push(*child, priority) {
                    self.cache[*child as usize].0 = node;
                    self.cache[*child as usize].1 = new_cost;
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
            let new = self.cache[node as usize].0;
            path.push(new);

            if new == self.start {
                break;
            }
        }
        path.reverse();

        path
    }
}
