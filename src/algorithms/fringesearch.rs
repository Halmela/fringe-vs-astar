use self::action::Action;
use self::bucket::Bucket;
use self::cache::{Cache, Value};
use self::fringe::Fringe;
use super::Heuristic;
use super::State;

use crate::printable::Printable;
use crate::structures::Graph;
use crate::Node;

/// Enum for representing an action for some [`Node`]
pub mod action;
/// Enum for accessing a bucket in [`Fringe`]
pub mod bucket;
/// Cached values used in algorithm
pub mod cache;
/// Datastructure for holding [`Node`]s to be processed
pub mod fringe;

/// Fringe search implementation.
/// Fringe holds now- and later-queues and Cache holds information on nodes.
/// This structure kind of glues those two together and passes nodes around as needed.
/// I wanted to separate them away from this main search algorithm for clarity, since caching can be messy
/// and at least in some points of development the queue handling was too.
pub struct FringeSearch<'a> {
    fringe: Fringe,
    cache: Cache,
    start: Node,
    goal: Node,
    graph: &'a Graph,
}

impl<'a> FringeSearch<'a> {
    /// Initialize the search with a start, goal and a graph to be acted upon.
    #[must_use]
    pub fn new(start: Node, goal: Node, graph: &'a Graph) -> Self {
        let size = graph.get_width() * graph.get_height();
        let heuristic = Heuristic::new(goal, graph.get_width());
        let cache = Cache::new(start, size, heuristic);
        let fringe = Fringe::new(start, size, cache.f_limit);

        FringeSearch {
            fringe,
            cache,
            start,
            goal,
            graph,
        }
    }

    /// Solve from start to goal. Returns `Some((path,cost))` if it can be found, `None` if path can't be found.
    /// I would like to unroll this main loop at some point to expose the datastructures at different points of search.
    ///
    /// Main idea here is to get a new node from now-queue, process it and maybe return it.
    /// If now is empty, then try to prepare datastructures for next iteration (`f_min` -> `f_limit` and later -> now).
    /// If now is empty and later is empty, then no further search can be conducted and thus a path can be found and `None` is returned.
    #[must_use]
    pub fn solve(mut self) -> Option<(Vec<Node>, f32)> {
        loop {
            if let Some(node) = self.fringe.pop_now() {
                if let Some(_goal) = self.process_node(node) {
                    return Some(self.construct_path());
                }
            } else if self.prepare_next_iteration() {
                continue;
            } else {
                return None;
            }
        }
    }

    /// One step of the solving process. This is used for the experimental printing of solution.
    pub fn progress(&mut self) -> State {
        if let Some(node) = self.fringe.pop_now() {
            if let Some(_goal) = self.process_node(node) {
                State::Finished(self.construct_path())
            } else {
                State::Processing(node)
            }
        } else if self.prepare_next_iteration() {
            return self.progress();
        } else {
            return State::NotFound;
        }
    }

    /// Check from cache if a `Node` has a low enough cost to have it (as a goal) returned,
    /// or have its neighbors processed.
    /// If it has too high of a cost, it is then put to later if it is not already there.
    fn process_node(&mut self, node: Node) -> Option<Node> {
        match self.cache.decide_action(node) {
            Action::Process(goal) if goal == self.goal => return Some(goal),
            Action::Process(node) => {
                self.process_neighbors(node);
            }
            Action::ToLater(node) => {
                self.fringe.push_later(node);
            }
            Action::Nothing => {}
        }
        None
    }

    /// Prepare the datastructures for next iteration
    fn prepare_next_iteration(&mut self) -> bool {
        if let Some(lower_limit) = self.fringe.later_to_now() {
            self.cache.refresh_limits(lower_limit);
            true
        } else {
            false
        }
    }

    /// Put node's viable neighbors to the now-queue. Viable as in lower cost then ever seen before.
    fn process_neighbors(&mut self, node: Node) {
        self.graph
            .neighbors(node)
            .filter_map(|(child, c)| self.cache.check(*child, node, *c))
            .for_each(|child| self.fringe.push(child));
    }

    /// Reconstruct path that was found
    fn construct_path(&self) -> (Vec<Node>, f32) {
        let mut path = vec![(self.goal)];
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
    #[must_use]
    pub fn add_to_printable(&self, mut print: Printable) -> Printable {
        self.fringe
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
            .filter(|(_, n): &(usize, &Value)| n.closed)
            .for_each(|(i, _)| print.add_inclosed(i.try_into().unwrap()));

        print.add_start(self.start);
        print.add_goal(self.goal);

        print.add_header("|Now|", self.fringe.now.len());
        let current_l = self.fringe.buckets[self.fringe.current as usize].len();
        print.add_header(format!("|{:?}|", self.fringe.current), current_l);
        let later_total: usize = self.fringe.buckets.iter().map(std::vec::Vec::len).sum();
        print.add_header("|Later|", later_total);
        if later_total > 0 {
            print.add_header(
                "% of later",
                (current_l as f32 / later_total as f32) * 100.0,
            );
            print.add_header(format!("in {:?}", self.fringe.current), "");
        }

        print
    }

    #[must_use]
    pub fn get_cost(&self, node: Node) -> f32 {
        self.cache.get_cost(node)
    }
    #[must_use]
    pub fn get_estimate(&self, node: Node) -> f32 {
        self.cache[node].estimate
    }
    #[must_use]
    pub fn now_size(&self) -> usize {
        self.fringe.now.len()
    }
    #[must_use]
    pub fn bucket_size(&self) -> usize {
        self.fringe.current().len()
    }
    #[must_use]
    pub fn later_size(&self) -> usize {
        self.fringe.buckets.iter().map(std::vec::Vec::len).sum()
    }
}
