use super::{Heuristic, State};
use crate::printable::Printable;
use crate::structures::AdjacencyListGraph;
use crate::Node;

use self::cache::Cache;
use self::frontier::Frontier;
use self::weighted_cell::WeightedCell;

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
                return Some(self.construct_path());
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

    pub fn progress(&mut self) -> State {
        if let Some(node) = self.frontier.pop() {
            if node == self.goal {
                return State::Finished(self.construct_path());
            }

            let current_cost = self.cache.get_cost(node);

            for (child, w1) in self.graph.neighbors(node) {
                if let Some((node, weight)) = self.cache.check(*child, node, current_cost + w1) {
                    self.frontier.push(node, weight);
                }
            }
            return State::Processing(node);
        } else {
            return State::NotFound;
        }
    }

    /// Reconstruct path that was found
    fn construct_path(&self) -> (Vec<Node>, f32) {
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

        (path, self.cache.get_cost(self.goal))
    }

    /// Add current state to Printable
    pub fn add_to_printable(&self, mut print: Printable) -> Printable {
        /* self.fringe
            .buckets
            .iter()
            .flatten()
            .for_each(|n| print.add_oldlater(*n));

        self.fringe[self.fringe.current]
            .iter()
            .for_each(|n| print.add_inlater(*n));

        self.fringe.now.iter().for_each(|n| print.add_inopen(*n));

        self.cache
            .cache
            .iter()
            .enumerate()
            .filter(|(_, n): &(usize, &CacheValue)| n.closed)
            .for_each(|(i, _)| print.add_inclosed(i.try_into().unwrap())); */

        self.cache
            .iter()
            .enumerate()
            .filter(|(_, n)| n.cost != 0.0)
            .for_each(|(i, _)| print.add_inclosed(i.try_into().unwrap()));

        self.frontier.iter().for_each(|n| print.add_infrontier(*n));
        let top3 = self.frontier.top3();
        if let Some(first) = top3.0 {
            print.add_first(first);
        }
        if let Some(second) = top3.1 {
            print.add_second(second);
        }
        if let Some(third) = top3.2 {
            print.add_third(third);
        }

        print
    }
}
