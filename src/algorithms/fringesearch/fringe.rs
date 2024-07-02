use self::buckets::Buckets;
use self::indexes::Indexes;
use super::action::Action;
use super::cache::Cache;
use super::Bucket;
use crate::algorithms::{Heuristic, State};
use crate::structures::Graph;
use crate::{Cost, Node, Path};
use std::f32::INFINITY;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

/// Datastructure used for storing nodes in Fringe search.
/// Does not hold any data on existence of a node in fringe
/// Buckets can be indexed with `Bucket` (`self[bucket]`).
// #[derive(Debug)]
pub struct Fringe<'a> {
    pub cache: Cache,
    pub current: Bucket,
    pub f_limit: Cost,
    pub f_min: Cost,
    pub graph: &'a Graph,
    pub buckets: Buckets,
}

impl<'a> Fringe<'a> {
    /// Create new Fringe
    #[must_use]
    pub fn new(start: Node, goal: Node, graph: &'a Graph) -> Self {
        let heuristic = Heuristic::new(goal, graph.get_width());
        let f_limit = heuristic.calc(start);
        let cache = Cache::new(start, goal, graph.map_size(), heuristic);

        let buckets = Buckets::new(start, f_limit, graph.size());

        Fringe {
            cache,
            current: Bucket::from(f_limit),
            f_limit,
            f_min: INFINITY,
            graph,
            buckets,
        }
    }

    /// Check if estimation of length is smaller than current limits and decide if anything should be done with the node.
    /// Updates f_min if needed.
    pub fn estimation_check(&mut self, node: Node) -> Action {
        match self.cache.get_estimate(node) {
            e if e >= self.f_min => Action::ToLater(node),
            e if e <= self.f_limit => self.cache.decide_action(node),
            e => {
                self.f_min = e;
                Action::ToLater(node)
            }
        }
    }

    /// Update f_limit and f_min, if a lower estimate was found this iteration.
    /// If this is the first passthrough of a bucket, it probably was not found.
    pub fn refresh_limit(&mut self) {
        if self.f_min < INFINITY {
            self.f_limit = self.f_min;
            self.f_min = INFINITY;
        }
    }

    /// Add node's neighbors to either Now-list or their corresponding bucket according to estimated length to goal.
    fn process_node(&mut self, node: Node) {
        for (child, cost) in self.graph.neighbors(node) {
            if let Some((child, parent, cost)) = self.cache.check(*child, node, *cost) {
                let estimate = self.cache.update(child, parent, cost);

                if estimate <= self.f_limit {
                    self.buckets.push_now(child);
                } else {
                    self.buckets.push_bucket(child, Bucket::from(estimate));
                }
            }
        }
    }

    /// Run through the whole solving process. Internally this works the same as [`act`], but does not reveal internal state.
    pub fn run(&mut self) -> Option<(Path, Cost)> {
        loop {
            match self.buckets.pop() {
                (None, true) => self.refresh_limit(),
                (None, false) => return None,
                (Some(node), from_now) => match self.estimation_check(node) {
                    Action::Finish(path) => return Some(path),
                    Action::Process(node) => {
                        if !from_now {
                            self.buckets.remove_later_head();
                        }
                        self.process_node(node);
                    }
                    Action::ToLater(_) => {
                        self.buckets.keep_current();
                    }
                    Action::Nothing => self.buckets.remove_later_head(),
                    _ => {}
                },
            }
        }
    }

    /// Do one step of the solving process and return what was done.
    ///
    /// One step is taking a node out of [`Buckets`] and adding its neighbors if it was a valid node.
    /// Otherwise it is inserted to the right bucket.
    /// If the node is has cost smaller than f_limit and it is the goal node, a full path is returned instead.
    /// If no node was found and refreshing buckets failed, then no path can be found.
    pub fn act(&mut self) -> State {
        match self.buckets.pop() {
            (None, true) => {
                self.refresh_limit();
                State::Internal
            }
            (None, false) => State::NotFound,
            (Some(node), from_now) => match self.estimation_check(node) {
                Action::Finish(path) => return State::Finished(path),
                Action::Process(node) => {
                    if !from_now {
                        self.buckets.remove_later_head();
                    }
                    self.process_node(node);
                    return State::Processing(node);
                }
                Action::ToLater(_) => {
                    self.buckets.keep_current();
                    return State::Processing(node);
                }
                Action::Nothing => {
                    self.buckets.remove_later_head();
                    return State::Processing(node);
                }
                _ => panic!("what"),
            },
        }
    }
}

/* impl<'a> Index<Bucket> for Fringe<'a> {
    type Output = Vec<Node>;

    fn index(&self, index: Bucket) -> &Self::Output {
        &self.buckets[Into::<usize>::into(index)]
    }
}
impl<'a> IndexMut<Bucket> for Fringe<'a> {
    fn index_mut(&mut self, index: Bucket) -> &mut Self::Output {
        &mut self.buckets[Into::<usize>::into(index)]
    }
} */

mod buckets;

pub mod indexes;
