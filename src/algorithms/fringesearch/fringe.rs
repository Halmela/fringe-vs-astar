use super::action::Action;
use super::cache::Cache;
use super::Bucket;
use crate::algorithms::Heuristic;
use crate::structures::Graph;
use crate::{Cost, Node, Path};
use std::f32::INFINITY;
use std::ops::{Index, IndexMut};

use std::collections::VecDeque;

const STOPPER: Node = Node::MAX;

/// Datastructure used for storing nodes in Fringe search.
/// Does not hold any data on existence of a node in fringe
/// Buckets can be indexed with `Bucket` (`self[bucket]`).
// #[derive(Debug)]
pub struct Fringe<'a> {
    pub now: VecDeque<Node>,
    pub buckets: [Vec<Node>; 8],
    pub cache: Cache,
    pub current: Bucket,
    pub f_limit: Cost,
    pub f_min: Cost,
    pub graph: &'a Graph,
}

impl<'a> Fringe<'a> {
    /// Create new Fringe
    #[must_use]
    pub fn new(start: Node, goal: Node, graph: &'a Graph) -> Self {
        let heuristic = Heuristic::new(goal, graph.get_width());
        let f_limit = heuristic.calc(start);
        let cache = Cache::new(start, goal, graph.get_size(), heuristic);

        let mut now = VecDeque::with_capacity(1024);
        now.push_front(start);
        now.push_back(STOPPER);

        Fringe {
            now,
            cache,
            buckets: Default::default(),
            current: Bucket::from(f_limit),
            f_limit,
            f_min: INFINITY,
            graph,
        }
    }

    pub fn now(&self) -> impl Iterator<Item = &Node> {
        self.now.iter().take_while(|n| n != &&STOPPER)
    }
    pub fn later(&self) -> impl Iterator<Item = &Node> {
        self.now.iter().skip_while(|n| n != &&STOPPER).skip(1)
    }

    pub fn push(&mut self, (node, estimate): (Node, Cost)) {
        let bucket = Bucket::from(estimate);
        if bucket == self.current {
            self.now.push_front(node);
        } else {
            self[bucket].push(node);
        }
    }

    /// Push node to be processed in later iteration
    #[inline(always)]
    pub fn push_later(&mut self, node: Node) {
        self.now.push_back(node);
    }

    pub fn run(&mut self) -> Option<(Path, Cost)> {
        loop {
            match self.fast_pop() {
                Action::Finish(path) => return Some(path),
                Action::Rotate => {
                    if !self.change_bucket() {
                        return None;
                    }
                }
                _ => {}
            }
        }
    }
    pub fn fast_pop(&mut self) -> Action {
        match self.now.pop_front() {
            Some(STOPPER) if self.now.len() > 0 => {
                self.refresh_limit();
            }
            Some(STOPPER) => return Action::Rotate,
            Some(node) => {
                let estimate = self.cache.get_estimate(node);
                if estimate <= self.f_limit {
                    match self.cache.decide_action(node) {
                        Action::Process(node) => self.process_neighbors(node),
                        Action::Finish(g) => return Action::Finish(g),
                        _ => {}
                    }
                } else if estimate < self.f_min {
                    self.f_min = estimate;
                    self.push_later(node);
                } else {
                    self.push_later(node);
                }
            }
            None => panic!("what an odd panic"),
        }
        self.fast_pop()
    }

    /// Try to give a node from now list
    #[inline(always)]
    pub fn pop_now(&mut self) -> Action {
        match self.now.pop_front() {
            Some(STOPPER) if self.now.len() > 0 => Action::Refresh,
            Some(STOPPER) => Action::Rotate,
            Some(node) => {
                let estimate = self.cache.get_estimate(node);
                if estimate <= self.f_limit {
                    return self.cache.decide_action(node);
                } else if estimate < self.f_min {
                    self.f_min = estimate
                }
                Action::ToLater(node)
            }
            None => Action::Nothing,
        }
    }

    pub fn change_bucket(&mut self) -> bool {
        let mut i = 0;
        loop {
            if i == 8 {
                return false;
            }
            if self[self.current].is_empty() {
                self.current = self.current.add();
                i += 1;
            } else {
                break;
            }
        }

        let current = self.current;
        self.now = self[current]
            .iter()
            .filter(|n| !self.cache[**n].closed)
            .copied()
            .collect();
        self[current].clear();
        self.refresh_limit();

        true
    }

    pub fn refresh_limit(&mut self) {
        if self.f_min < INFINITY {
            self.f_limit = self.f_min;
            self.f_min = INFINITY;
        }
        self.now.push_back(STOPPER);
    }

    /// Rotate later-buckets until a suitable is found, empty it to now and return the amount of rotations for `f_limit` fixing.
    /// Returns `None` if all buckets are empty.
    pub fn later_to_now(&mut self) -> bool {
        if self.now.is_empty() {
            // Rotate buckets until a suitable is found
            let mut i = 0;
            loop {
                if i == 8 {
                    return false;
                }
                if self[self.current].is_empty() {
                    self.current = self.current.add();
                    i += 1;
                } else {
                    break;
                }
            }
            let current = self.current;
            self.now = VecDeque::from(self[current].clone());
            self[current].clear();
        }
        self.now.push_back(STOPPER);
        true
    }

    /// Put node's viable neighbors to the now-queue. Viable as in lower cost then ever seen before.
    pub fn process_neighbors(&mut self, node: Node) {
        for (child, cost) in self.graph.neighbors(node) {
            if let Some((child, parent, cost)) = self.cache.check(*child, node, *cost) {
                let estimate = self.cache.update(child, parent, cost);
                self.push((child, estimate));
            }
        }
    }
}

impl<'a> Index<Bucket> for Fringe<'a> {
    type Output = Vec<Node>;

    fn index(&self, index: Bucket) -> &Self::Output {
        &self.buckets[Into::<usize>::into(index)]
    }
}
impl<'a> IndexMut<Bucket> for Fringe<'a> {
    fn index_mut(&mut self, index: Bucket) -> &mut Self::Output {
        &mut self.buckets[Into::<usize>::into(index)]
    }
}
