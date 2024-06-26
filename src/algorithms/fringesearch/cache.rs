use super::Action;
use super::Bucket;
use super::Heuristic;
use crate::Node;

use std::fmt;
use std::ops::{Index, IndexMut};

/// Stored values for a single node in graph as wanted by Fringe Search.
///
/// Cost, estimate and parent are updated as needed, heuristic is calculated only once
/// and `closed` makes sure that already closed [`Node`]s are not expanded again.
/// Bucket indicates where Node is in later-queue.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    pub cost: f32,
    pub heuristic: f32,
    pub parent: Node,
    pub estimate: f32,
    pub later: u32,
    pub closed: bool,
    pub bucket: Bucket,
}

impl Default for Value {
    fn default() -> Self {
        Value {
            cost: f32::MAX,
            heuristic: f32::MAX,
            parent: 0,
            estimate: f32::MAX,
            later: 0,
            closed: false,
            bucket: Bucket::None,
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
    pub f_limit: f32,
    f_min: f32,
    pub iteration: u32,
}

impl Cache {
    /// Initialize cache
    #[must_use]
    pub fn new(start: Node, size: usize, heuristic: Heuristic) -> Self {
        let mut cache = vec![Value::default(); size];
        let f_limit = heuristic.calc(start);
        cache[start as usize].cost = 0.0;
        cache[start as usize].heuristic = f_limit;
        cache[start as usize].estimate = f_limit;
        Cache {
            cache,
            heuristic,
            f_limit,
            f_min: f32::MAX,
            iteration: 1,
        }
    }

    /// Decide if [`Node`] should go to now or to later or if nothing should be done to it.
    ///
    /// Checks if a node is already closed, otherwise will check the estimate.
    /// Updates `self.f_limit` if needed.
    pub fn decide_action(&mut self, node: Node) -> Action {
        if self[node].closed {
            return Action::Nothing;
        }
        if self[node].estimate <= self.f_limit {
            self[node].closed = true;
            Action::Process(node)
        } else {
            self.later_or_nothing(node)
        }
    }

    /// Update value of a node with given `cost` and `parent`
    ///
    /// Also calculates `heuristic`, `estimate` and `bucket` in advance
    pub fn update(&mut self, node: Node, cost: f32, parent: Node) -> Bucket {
        self[node].cost = cost;
        self[node].parent = parent;
        self[node].estimate = cost + self.get_heuristic(node);
        self[node].closed = false;
        let bucket = Bucket::from(self[node].estimate);
        self[node].bucket = bucket;

        bucket
    }

    /// Get heuristic value from cache or calculate it
    pub fn get_heuristic(&mut self, node: Node) -> f32 {
        if self[node].heuristic == f32::MAX {
            self[node].heuristic = self.heuristic.calc(node);
        }
        self[node].heuristic
    }

    /// Get cost of a node
    #[must_use]
    pub fn get_cost(&self, node: Node) -> f32 {
        self[node].cost
    }

    /// Update `f_limit` value and +1 to the iteration count used for counting if a node is in later
    pub fn refresh_limits(&mut self, lower_limit: u8) {
        // Really uglly haccck for the times when floor(f_limit+lower_limit) was found on earlier iterations
        if lower_limit > 0 {
            self.f_limit = (self.f_limit + f32::from(lower_limit)).floor();
        }
        // Really funky behavior without this check
        else if self.f_min != f32::MAX {
            self.f_limit = self.f_min;
        }
        self.f_min = f32::MAX;
        self.iteration += 1;
    }

    /// Decide if a child-node should be added to the now-queue.
    /// It's value is updated, if it is added.
    /// This returns `Option<Node` because it allows neat `filter_map` on the call site.
    pub fn check(&mut self, child: Node, parent: Node, move_cost: f32) -> Option<(Node, Bucket)> {
        let new_cost = self[parent].cost + move_cost;

        if new_cost < self[child].cost {
            Some((child, self.update(child, new_cost, parent)))
        } else {
            None
        }
    }

    /// Add node to later if it is not already there
    fn later_or_nothing(&mut self, node: Node) -> Action {
        if self[node].estimate < self.f_min {
            self.f_min = self[node].estimate;
        }
        self[node].closed = false;
        if self[node].later == self.iteration {
            Action::Nothing
        } else {
            self[node].later = self.iteration;
            Action::ToLater((node, self[node].bucket))
        }
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

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "i: {}\tlimit: {}\tmin: {}",
            self.iteration, self.f_limit, self.f_min
        )
    }
}
