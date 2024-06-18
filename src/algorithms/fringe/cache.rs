use crate::Node;

use super::Heuristic;

use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub struct CacheValue {
    pub cost: f32,
    pub heuristic: f32,
    pub parent: Node,
    pub estimate: f32,
    pub later: u32,
}

impl CacheValue {
    pub fn new() -> Self {
        CacheValue {
            cost: f32::MAX,
            heuristic: f32::MAX,
            parent: 0,
            estimate: f32::MAX,
            later: 0,
        }
    }
}

pub enum Action {
    Process(Node),
    ToLater(Node),
    Nothing,
}

pub struct Cache {
    pub cache: Vec<CacheValue>,
    heuristic: Heuristic,
    f_limit: f32,
    f_min: f32,
    pub iteration: u32,
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
            f_min: f32::MAX,
            iteration: 1,
        }
    }

    #[inline(always)]
    pub fn check_estimate(&mut self, node: Node) -> Action {
        let estimate = self[node].estimate;

        if estimate <= self.f_limit {
            return Action::Process(node);
        }

        if estimate < self.f_min {
            self.f_min = estimate;
        }

        if self.not_in_later(node) {
            Action::ToLater(node)
        } else {
            Action::Nothing
        }
    }

    pub fn update(&mut self, node: Node, cost: f32, parent: Node) {
        self[node].cost = cost;
        self[node].parent = parent;
        self[node].estimate = cost + self.get_heuristic(node);
    }

    pub fn get_heuristic(&mut self, node: Node) -> f32 {
        if self[node].heuristic == f32::MAX {
            self[node].heuristic = self.heuristic.get(node);
        }
        self[node].heuristic
    }

    pub fn get_cost(&self, node: Node) -> f32 {
        self[node].cost
    }

    pub fn refresh_limits(&mut self) {
        self.f_limit = self.f_min;
        self.f_min = f32::MAX;
        self.iteration += 1;
    }

    pub fn check(&mut self, child: &usize, parent: Node, move_cost: f32) -> Option<Node> {
        let new_cost = self[parent].cost + move_cost;
        let child = (*child).try_into().unwrap();

        if new_cost < self[child].cost {
            self.update(child, new_cost, parent);
            // if new_cost <= self.f_min {
            Some(child)
            /* } else {
                self[child].later = self.iteration;
                None
            } */
        } else {
            None
        }
    }
    fn not_in_later(&mut self, node: Node) -> bool {
        if self[node].later != self.iteration {
            self[node].later = self.iteration;
            true
        } else {
            false
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
