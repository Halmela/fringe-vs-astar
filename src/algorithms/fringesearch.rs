use self::action::Action;
use self::bucket::Bucket;
use self::buckets::Buckets;
use self::cache::Value;
use self::fringe::Fringe;
use self::indexes::Indexes;
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
/// Fringe's internal representation of Nodes
pub mod buckets;
/// Cached values used in algorithm
pub mod cache;
/// Datastructure for holding [`Node`]s to be processed
pub mod fringe;
/// Auxilliary structure for holding and modifying bucket-indexes
pub mod indexes;

/// Fringe search implementation.
///
/// This is mostly a wrapper around the [`Fringe`] struct which does the actual solving.
/// Search can be done fast or it can be done with additional information on internal state at each step.§
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
    /// Does not leak internal state and lets [`Fringe`] do some optimizations.
    #[must_use]
    pub fn solve(mut self) -> Option<(Path, Cost)> {
        self.fringe.run()
    }

    /// One step of the solving process. This is used for getting the state of [`Fringe`] at each step of solving process
    pub fn progress(&mut self) -> State {
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
}
