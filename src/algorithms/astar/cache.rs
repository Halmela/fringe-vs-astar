use super::Heuristic;
use crate::Node;
use std::ops::{Index, IndexMut};

/// A* cache value. Holds parent, cost, heuristic (calculated once) and estimate
#[derive(Clone, Copy)]
pub struct Value {
    pub parent: Node,
    pub cost: f32,
    heuristic: f32,
    estimate: f32,
}

impl Default for Value {
    fn default() -> Self {
        Value {
            parent: 0,
            cost: 0.0,
            heuristic: f32::MAX,
            estimate: f32::MAX,
        }
    }
}

/// A* cache, can be indexed as `cache[node]`.
pub struct Cache {
    cache: Vec<Value>,
    heuristic: Heuristic,
}

impl Cache {
    /// Initialize with start
    #[must_use] pub fn new(start: Node, heuristic: Heuristic, size: usize) -> Self {
        let mut cache = vec![Value::default(); size];
        cache[start as usize].parent = start;
        cache[start as usize].heuristic = heuristic.calc(start);

        Self { cache, heuristic }
    }

    /// Get heuristic value from cache or calculate it
    pub fn get_heuristic(&mut self, node: Node) -> f32 {
        if self[node].heuristic == f32::MAX {
            self[node].heuristic = self.heuristic.calc(node);
        }
        self[node].heuristic
    }

    /// Get cost of a node
    #[must_use] pub fn get_cost(&self, node: Node) -> f32 {
        self[node].cost
    }

    /// Check if a node should be processed. Updates values as needed
    pub fn check(&mut self, node: Node, parent: Node, new_cost: f32) -> Option<(Node, f32)> {
        let to_goal = self.get_heuristic(node);
        let estimate = new_cost + to_goal;

        if estimate < self[node].estimate {
            self[node].parent = parent;
            self[node].cost = new_cost;
            self[node].estimate = estimate;
            Some((node, estimate))
        } else {
            None
        }
    }

    /// Ergonomy iterator for own cache
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.cache.iter()
    }

    /// Get estimate of a [`Node`]
    #[must_use] pub fn get_estimate(&self, node: u32) -> f32 {
        self[node].estimate
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
