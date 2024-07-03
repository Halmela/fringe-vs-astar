use super::Action;
use super::Heuristic;
use crate::Cost;
use crate::Node;
use crate::Path;

use std::ops::{Index, IndexMut};

use std::f32::INFINITY;

/// Stored values for a single node in graph as wanted by Fringe Search.
///
/// Cost, estimate and parent are updated as needed, heuristic is calculated only once
/// and `closed` makes sure that already closed [`Node`]s are not expanded again.
/// Bucket indicates where Node is in later-queue.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    pub cost: Cost,
    pub heuristic: Cost,
    pub estimate: Cost,
    pub parent: Node,
    pub closed: bool,
}

impl Default for Value {
    fn default() -> Self {
        Value {
            cost: INFINITY,
            heuristic: INFINITY,
            estimate: INFINITY,
            parent: 0,
            closed: false,
        }
    }
}

/// Datastructure for caching information while performing Fringe search.
/// I separated this to its own structure to clarify the main algorithm.
/// This does the book keeping for every node and updates the values as needed.
///
/// Cache can be indexed with a [`Node`]: `cache[node]` or `self[node]`.
pub struct Cache {
    pub cache: Vec<Value>,
    heuristic: Heuristic,
    pub iteration: u32,
    pub start: Node,
    pub goal: Node,
}

impl Cache {
    /// Initialize cache
    #[must_use]
    pub fn new(start: Node, goal: Node, size: usize, heuristic: Heuristic) -> Self {
        let mut cache = vec![Value::default(); size];
        cache[start as usize].cost = 0.0;
        cache[start as usize].heuristic = heuristic.calc(start);
        cache[start as usize].estimate = cache[start as usize].heuristic;
        Cache {
            cache,
            heuristic,
            iteration: 1,
            goal,
            start,
        }
    }

    /// Decide if [`Node`] should go to now or to later or if nothing should be done to it.
    ///
    /// Checks if a node is already closed, otherwise will check the estimate.
    /// Updates `self.f_limit` if needed.
    pub fn decide_action(&mut self, node: Node) -> Action {
        if self[node].closed {
            return Action::Nothing;
        } else if node == self.goal {
            return Action::Finish(self.construct_path());
        } else {
            self[node].closed = true;
            Action::Process(node)
        }
    }

    /// Update value of a node with given `cost` and `parent`
    ///
    /// Also calculates `heuristic`, `estimate` and `bucket` in advance
    pub fn update(&mut self, node: Node, parent: Node, cost: Cost) -> Cost {
        self[node].cost = cost;
        self[node].parent = parent;
        self[node].closed = false;
        self[node].estimate = self.get_heuristic(node) + cost;

        self[node].estimate
    }

    /// Get heuristic value from cache or calculate it
    pub fn get_heuristic(&mut self, node: Node) -> Cost {
        if self[node].heuristic == INFINITY {
            self[node].heuristic = self.heuristic.calc(node);
        }
        self[node].heuristic
    }

    /// Get cost of a node
    #[must_use]
    pub fn get_cost(&self, node: Node) -> Cost {
        self[node].cost
    }

    /// Get cost of a node
    pub fn get_estimate(&mut self, node: Node) -> Cost {
        self[node].estimate
    }

    /// Decide if a child-node should be added to the now-queue.
    /// It's value is updated, if it is added.
    /// This returns `Option<Node` because it allows neat `filter_map` on the call site.
    pub fn check(&self, child: Node, parent: Node, move_cost: Cost) -> Option<(Node, Node, Cost)> {
        let new_cost = self[parent].cost + move_cost;

        if new_cost < self[child].cost {
            Some((child, parent, new_cost))
        } else {
            None
        }
    }

    /// Reconstruct path that was found
    fn construct_path(&self) -> (Path, Cost) {
        let mut path = vec![(self.goal)];
        loop {
            let node = path[path.len() - 1];
            let new = self[node].parent;
            path.push(new);

            if self.start == new {
                break;
            }
        }
        path.reverse();

        (path, self.get_cost(self.goal))
    }
}

impl Index<Node> for Cache {
    type Output = Value;

    fn index(&self, index: Node) -> &Self::Output {
        &self.cache[index as usize]
    }
}

impl IndexMut<Node> for Cache {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.cache[index as usize]
    }
}
