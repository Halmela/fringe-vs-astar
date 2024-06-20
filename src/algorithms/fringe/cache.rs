use crate::structures::fringe::Bucket;
use crate::Node;

use super::Heuristic;

use std::fmt;
use std::ops::{Index, IndexMut};

/// Stored values for a single node in graph as wanted by Fringe Search.
/// Cost, estimate and parent are updated as needed, heuristic is calculated only once
/// and later makes sure that each node is put to later-queue at most once per iteration.
#[derive(Clone, Copy, Debug)]
pub struct CacheValue {
    pub cost: f32,
    pub heuristic: f32,
    pub parent: Node,
    pub estimate: f32,
    pub later: u32,
    // pub status: Status,
    pub closed: bool,
}

impl CacheValue {
    /// Initialize cache value
    pub fn new() -> Self {
        CacheValue {
            cost: f32::MAX,
            heuristic: f32::MAX,
            parent: 0,
            estimate: f32::MAX,
            later: 0,
            // status: Status::NotVisited,
            closed: false,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Status {
    Now,
    Later(Bucket),
    Closed,
    NotVisited,
}

/// What should the main algorithm do with a node?
pub enum Action {
    Process(Node),
    ToLater((Node, Bucket)),
    Nothing,
}

/// Datastructure for caching information while performing Fringe search.
/// I separated this to its own structure to clarify the main algorithm.
/// This does the book keeping for every node and updates the values as needed.
///
/// Cache can be indexed with a `Node`: `cache[node]` or `self[node]`.
pub struct Cache {
    pub cache: Vec<CacheValue>,
    heuristic: Heuristic,
    pub f_limit: f32,
    f_min: f32,
    pub iteration: u32,
}

impl Cache {
    /// Initialize cache
    pub fn new(start: usize, size: usize, heuristic: Heuristic) -> Self {
        let mut cache: Vec<CacheValue> = vec![CacheValue::new(); size];
        let f_limit = heuristic.get(start.try_into().unwrap());
        cache[start].cost = 0.0;
        cache[start].heuristic = f_limit;
        cache[start].estimate = f_limit;
        // cache[start].status = Status::Now;
        Cache {
            cache,
            heuristic,
            f_limit,
            f_min: f32::MAX,
            iteration: 1,
        }
    }

    pub fn decide_action(&mut self, node: Node) -> Action {
        if self[node].closed {
            return Action::Nothing;
        }
        match self[node].estimate {
            e if e <= self.f_limit => {
                // self[node].status = Status::Closed;
                self[node].closed = true;
                Action::Process(node)
            }
            e if e < self.f_min => {
                self.f_min = e;
                self.later_or_nothing(node)
            }
            _ => self.later_or_nothing(node),
        }
    }

    /// Update value of a node
    pub fn update(&mut self, node: Node, cost: f32, parent: Node) {
        self[node].cost = cost;
        self[node].parent = parent;
        self[node].estimate = cost + self.get_heuristic(node);
        // self[node].status = Status::Now;
        self[node].closed = false;
    }

    /// Get heuristic value from cache or calculate it
    pub fn get_heuristic(&mut self, node: Node) -> f32 {
        if self[node].heuristic == f32::MAX {
            self[node].heuristic = self.heuristic.get(node);
        }
        self[node].heuristic
    }

    /// Get cost of a node
    pub fn get_cost(&self, node: Node) -> f32 {
        self[node].cost
    }

    /// Update f_limit value and +1 to the iteration count used for counting if a node is in later
    pub fn refresh_limits(&mut self, lower_limit: u8) {
        // Really uglly haccck for the times when floor(f_limit+lower_limit) was found on earlier iterations
        if lower_limit > 0 {
            self.f_limit = (self.f_limit + lower_limit as f32).floor();
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
    /// This returns `Option<Node` because it allows neat filter_map on the call site.
    pub fn check(&mut self, child: &usize, parent: Node, move_cost: f32) -> Option<Node> {
        let new_cost = self[parent].cost + move_cost;
        let child = (*child).try_into().unwrap();

        if new_cost < self[child].cost {
            self.update(child, new_cost, parent);
            Some(child)
        } else {
            None
        }
    }

    fn later_or_nothing(&mut self, node: Node) -> Action {
        let bucket: Bucket = self[node].estimate.into();
        // self[node].status = Status::Later(bucket);
        self[node].closed = false;
        if self[node].later != self.iteration {
            self[node].later = self.iteration;
            Action::ToLater((node, bucket))
        } else {
            Action::Nothing
        }
    }
}

impl Index<Node> for Cache {
    type Output = CacheValue;

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
