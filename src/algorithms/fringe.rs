use super::heuristic;
use crate::index_to_xy;
use crate::structures::{AdjacencyListGraph, Fringe};

#[derive(Clone, Copy)]
enum Status {
    Now,
    Later,
    None,
}

struct Heuristic {
    goal: (usize, usize),
    width: usize,
}

impl Heuristic {
    pub fn new(goal: usize, width: usize) -> Self {
        Heuristic {
            goal: index_to_xy(goal, width),
            width,
        }
    }

    pub fn get(&mut self, i: usize) -> f64 {
        heuristic(index_to_xy(i, self.width), self.goal)
    }
}

#[derive(Clone, Copy)]
struct CacheValue {
    pub cost: f64,
    pub status: Status,
    pub heuristic: f64,
    pub parent: usize,
}
impl CacheValue {
    pub fn new() -> Self {
        CacheValue {
            cost: f64::MAX,
            status: Status::None,
            heuristic: f64::MAX,
            parent: 0,
        }
    }
}

pub struct FringeSearch<'a> {
    fringe: Fringe,
    cache: Vec<CacheValue>, // (cost,parent,heuristic,in_fringe)
    f_limit: f64,
    f_min: f64,
    heuristic: Heuristic,
    start: usize,
    goal: usize,
    graph: &'a AdjacencyListGraph,
}

impl<'a> FringeSearch<'a> {
    pub fn new(start: usize, goal: usize, graph: &'a AdjacencyListGraph) -> Self {
        let size = graph.get_width() * graph.get_height();
        let fringe = Fringe::new(start, size);

        let mut cache: Vec<CacheValue> = vec![CacheValue::new(); size];
        cache[start].cost = 0.0;
        cache[start].status = Status::Now;

        let mut heuristic = Heuristic::new(goal, graph.get_width());

        FringeSearch {
            fringe,
            cache,
            f_limit: heuristic.get(start),
            f_min: f64::MAX,
            heuristic,
            start,
            goal,
            graph,
        }
    }

    pub fn solve(mut self) -> Option<(Vec<usize>, f64)> {
        loop {
            if let Some(node) = self.fringe.pop_now() {
                if let Some(goal) = self.process_node(node) {
                    let cost = self.cache[goal].cost;
                    return Some((self.construct_path(), cost));
                }
            } else {
                if self.prepare_next_iteration() {
                    continue;
                } else {
                    return None;
                }
            }

            // println!("{}\n\n", f_limit);
        }
    }

    fn process_node(&mut self, node: usize) -> Option<usize> {
        if !self.in_fringe(node) {
            return None;
        }

        if self.check_estimate(node) {
            if node == self.goal {
                return Some(node);
            }

            self.process_neighbors(node);
            self.cache[node].status = Status::None;
        } else {
            self.fringe.push_later(node);
            self.cache[node].status = Status::Later;
        }
        None
    }

    fn prepare_next_iteration(&mut self) -> bool {
        self.f_limit = self.f_min;
        self.f_min = f64::MAX;

        self.fringe.later_to_now()
    }

    fn in_fringe(&self, node: usize) -> bool {
        matches!(self.cache[node].status, Status::Later | Status::Now)
    }

    fn check_estimate(&mut self, node: usize) -> bool {
        let estimate = self.cache[node].cost + self.h(node);
        if estimate <= self.f_limit {
            return true;
        }

        if estimate < self.f_min {
            self.f_min = estimate;
            return false;
        } else {
            return false;
        }
    }

    fn process_neighbors(&mut self, node: usize) {
        let children: Vec<(&usize, f64)> = self
            .graph
            .neighbors(node)
            .iter()
            .map(|(i, c)| (i, self.cache[node].cost + c))
            .filter(|(i, new)| {
                !matches!(self.cache[**i].status, Status::Now) && *new < self.cache[**i].cost
            })
            .collect();

        for (child, new_cost) in children {
            self.fringe.push_now(*child);
            self.cache[*child].cost = new_cost;
            self.cache[*child].parent = node;
            self.cache[*child].status = Status::Now;
        }
    }

    fn h(&mut self, node: usize) -> f64 {
        if self.cache[node].heuristic == f64::MAX {
            self.cache[node].heuristic = self.heuristic.get(node);
        }
        self.cache[node].heuristic
    }

    /// Reconstruct path that was found
    fn construct_path(self) -> Vec<usize> {
        let mut path = vec![(self.goal)];
        loop {
            let i = path[path.len() - 1];
            let new = self.cache[i].parent;
            path.push(new);

            if new == self.start {
                break;
            }
        }
        path.reverse();

        path
    }
}
