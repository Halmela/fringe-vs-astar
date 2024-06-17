use crate::Node;

use super::Heuristic;

use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct CacheValue {
    pub cost: f64,
    pub heuristic: f64,
    pub parent: Node,
    pub estimate: f64,
}

impl CacheValue {
    pub fn new() -> Self {
        CacheValue {
            cost: f64::MAX,
            heuristic: f64::MAX,
            parent: 0,
            estimate: f64::MAX,
        }
    }
}

pub struct Cache {
    cache: Vec<CacheValue>,
    heuristic: Heuristic,
    f_limit: f64,
    f_min: f64,
}

impl Cache {
    pub fn new(start: usize, size: usize, heuristic: Heuristic) -> Self {
        let mut cache: Vec<CacheValue> = vec![CacheValue::new(); size];
        let f_limit = heuristic.get(start.try_into().unwrap());
        cache[start].cost = 0.0;
        cache[start].heuristic = f_limit;
        cache[start].estimate = f_limit;
        Cache {
            cache,
            heuristic,
            f_limit,
            f_min: f64::MAX,
        }
    }

    #[inline]
    pub fn check_estimate(&mut self, node: Node) -> bool {
        let estimate = self[node].estimate;

        if estimate >= self.f_min {
            false
        } else if estimate <= self.f_limit {
            true
        } else {
            self.f_min = estimate;
            false
        }
        /* if estimate <= self.f_limit {
            self.counter.1 += 1;
            true
        } else if estimate < self.f_min {
            self.counter.2 += 1;
            self.f_min = estimate;
            return false;
        } else {
            self.counter.3 += 1;
            return false;
        } */
    }

    pub fn update(&mut self, node: Node, cost: f64, parent: Node) {
        self[node].cost = cost;
        self[node].parent = parent;
        self[node].estimate = cost + self.get_heuristic(node);
    }

    pub fn get_heuristic(&mut self, node: Node) -> f64 {
        if self[node].heuristic == f64::MAX {
            self[node].heuristic = self.heuristic.get(node);
        }
        self[node].heuristic
    }

    pub fn get_cost(&self, node: Node) -> f64 {
        self[node].cost
    }

    pub fn refresh_limits(&mut self) {
        self.f_limit = self.f_min;
        self.f_min = f64::MAX;
    }

    pub fn check(&mut self, child: &usize, parent: Node, move_cost: f64) -> Option<Node> {
        let new_cost = self[parent].cost + move_cost;
        let child = (*child).try_into().unwrap();

        if new_cost < self[child].cost {
            self.update(child, new_cost, parent);
            if new_cost <= self.f_limit {
                Some(child)
            } else {
                None
            }
        } else {
            None
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
