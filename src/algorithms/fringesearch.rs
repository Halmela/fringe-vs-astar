use self::action::Action;
use self::bucket::Bucket;
use self::cache::Value;
use self::fringe::Fringe;
use super::Heuristic;
use super::State;

use crate::printable::Printable;
use crate::structures::Graph;
use crate::Cost;
use crate::Node;
use crate::Path;

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
    fringe: Fringe<'a>,
    start: Node,
    goal: Node,
}

impl<'a> FringeSearch<'a> {
    /// Initialize the search with a start, goal and a graph to be acted upon.
    #[must_use]
    pub fn new(start: Node, goal: Node, graph: &'a Graph) -> Self {
        let fringe = Fringe::new(start, goal, graph);

        FringeSearch {
            fringe,
            start,
            goal,
        }
    }

    /// Solve from start to goal. Returns `Some((path,cost))` if it can be found, `None` if path can't be found.
    /// I would like to unroll this main loop at some point to expose the datastructures at different points of search.
    ///
    /// Main idea here is to get a new node from now-queue, process it and maybe return it.
    /// If now is empty, then try to prepare datastructures for next iteration (`f_min` -> `f_limit` and later -> now).
    /// If now is empty and later is empty, then no further search can be conducted and thus a path can be found and `None` is returned.
    #[must_use]
    pub fn solve(mut self) -> Option<(Path, Cost)> {
        // loop {
        //     match self.progress() {
        //         State::Finished(path) => return Some(path),
        //         _ => {}
        //     }
        // }
        //self.fringe.run()
        self.fringe.new_run()
    }

    /// One step of the solving process. This is used for the experimental printing of solution.
    pub fn progress(&mut self) -> State {
        // match self.fringe.pop_now() {
        //     Action::Finish(path) => State::Finished(path),
        //     Action::Process(node) => {
        //         self.fringe.process_neighbors(node);
        //         State::Processing(node)
        //     }
        //     Action::ToLater(node) => {
        //         self.fringe.push_later(node);
        //         State::Processing(node)
        //     }
        //     Action::Refresh => {
        //         self.fringe.change_bucket();
        //         // self.fringe.refresh_limit();
        //         State::Internal
        //     }
        //     Action::Rotate => {
        //         if self.fringe.change_bucket() {
        //             State::Internal
        //         } else {
        //             State::NotFound
        //         }
        //     }
        //     Action::Nothing => State::Internal,
        // }
        self.fringe.act()
    }

    /// Add current state to Printable
    #[must_use]
    pub fn add_to_printable(&self, mut print: Printable) -> Printable {
        self.fringe
            .buckets
            .all()
            .iter()
            .flatten()
            .for_each(|n| print.add_oldlater(*n));

        self.fringe
            .buckets
            .next_later()
            .iter()
            .flatten()
            .for_each(|n| print.add_inlater(*n));
        self.fringe
            .buckets
            .later()
            .iter()
            .flatten()
            .for_each(|n| print.add_inopen(*n));
        self.fringe
            .buckets
            .now()
            .iter()
            .flatten()
            .for_each(|n| print.add_first(*n));

        self.fringe
            .cache
            .cache
            .iter()
            .enumerate()
            .filter(|(_, n): &(usize, &Value)| n.closed)
            .for_each(|(i, _)| print.add_inclosed(i.try_into().unwrap()));

        print.add_start(self.start);
        print.add_goal(self.goal);

        print.add_header("f_limit", self.fringe.f_limit);
        print.add_header("f_min", self.fringe.f_min);
        print.add_header(
            "|Now|",
            self.fringe.buckets.now().len() + self.fringe.buckets.later().len(),
        );
        // print.add_header("", format!("{:?}", self.fringe.buckets.now()));
        // print.add_header("", format!("{:?}", self.fringe.buckets.later()));
        let current_l = self.fringe.buckets.next_later().len();
        print.add_header(format!("|{:?}|", self.fringe.current), current_l);
        // print.add_header("", format!("{:?}", self.fringe.buckets.next_later()));
        let later_total: usize = self.fringe.buckets.all().iter().len();
        print.add_header("|Later|", later_total);
        if later_total > 0 {
            print.add_header(
                "% of later",
                (current_l as Cost / (later_total as Cost + current_l as Cost)) * 100.0,
            );
            print.add_header(format!("in {:?}", self.fringe.current), "");
        }
        print.add_header(
            "|Closed|",
            self.fringe.cache.cache.iter().filter(|v| v.closed).count(),
        );
        print.add_spacing();

        print
    }

    #[must_use]
    pub fn get_cost(&self, node: Node) -> Cost {
        self.fringe.cache.get_cost(node)
    }
    #[must_use]
    pub fn get_estimate(&mut self, node: Node) -> Cost {
        self.fringe.cache.get_estimate(node)
    }
    // #[must_use]
    // pub fn now_size(&self) -> usize {
    //     self.fringe.buckets.now.len()
    // }
    // #[must_use]
    // pub fn bucket_size(&self) -> usize {
    //     self.fringe.later().count()
    // }
    // #[must_use]
    // pub fn later_size(&self) -> usize {
    //     self.fringe.buckets.iter().map(std::vec::Vec::len).sum()
    // }
    // pub fn next_is_closed(&self) -> bool {
    //     self.fringe
    //         .now
    //         .last()
    //         .is_some_and(|n| self.fringe.cache[*n].closed)
    // }
}
