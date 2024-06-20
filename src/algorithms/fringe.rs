use crate::algorithms::fringe::cache::*;
use crate::algorithms::heuristic;
use crate::index_to_xy;
use crate::printable::Printable;
use crate::structures::AdjacencyListGraph;
use crate::structures::Fringe;
use crate::Node;

mod cache;

use std::collections::HashMap;

pub enum State {
    Finished((Vec<usize>, f32)),
    Processing(Node),
    NotFound,
}

/// Ugly wrapper for the common heuristic function. Handles 1D -> 2D coordinate conversion...
/// This should not be a thing anymore...
struct Heuristic {
    goal: (usize, usize),
    width: usize,
}

impl Heuristic {
    /// Initialize self with goal and width
    pub fn new(goal: usize, width: usize) -> Self {
        Heuristic {
            goal: index_to_xy(goal, width),
            width,
        }
    }

    /// Calculate heuristic value
    pub fn get(&self, i: Node) -> f32 {
        heuristic(index_to_xy(i as usize, self.width), self.goal)
    }
}

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
    graph: &'a AdjacencyListGraph,
}

impl<'a> FringeSearch<'a> {
    /// Initialize the search with a start, goal and a graph to be acted upon.
    pub fn new(start: usize, goal: usize, graph: &'a AdjacencyListGraph) -> Self {
        let size = graph.get_width() * graph.get_height();
        let heuristic = Heuristic::new(goal, graph.get_width());
        let cache = Cache::new(start, size, heuristic);
        let fringe = Fringe::new(start.try_into().unwrap(), size, cache.f_limit);

        FringeSearch {
            fringe,
            cache,
            start: start.try_into().unwrap(),
            goal: goal.try_into().unwrap(),
            graph,
        }
    }

    /// Solve from start to goal. Returns `Some((path,cost))` if it can be found, `None` if path can't be found.
    /// I would like to unroll this main loop at some point to expose the datastructures at different points of search.
    ///
    /// Main idea here is to get a new node from now-queue, process it and maybe return it.
    /// If now is empty, then try to prepare datastructures for next iteration (f_min -> f_limit and later -> now).
    /// If now is empty and later is empty, then no further search can be conducted and thus a path can be found and `None` is returned.
    pub fn solve(mut self) -> Option<(Vec<usize>, f32)> {
        loop {
            if let Some(node) = self.fringe.pop_now() {
                if let Some(goal) = self.process_node(node) {
                    let cost = self.cache.get_cost(goal);
                    return Some((self.construct_path(), cost));
                }
            } else if self.prepare_next_iteration() {
                continue;
            } else {
                return None;
            }
        }
    }

    pub fn progress(&mut self) -> State {
        if let Some(node) = self.fringe.pop_now() {
            // println!("{:?}\t{:?}", self.fringe.current, self.cache.f_limit);
            if let Some(goal) = self.process_node(node) {
                let cost = self.cache.get_cost(goal);
                let path = self.construct_path();
                State::Finished((path, cost))
            } else {
                // println!("- {:?}", self.fringe.buckets);
                // println!("{}", self.cache);
                State::Processing(node)
            }
        } else if self.prepare_next_iteration() {
            /* let v: Vec<f32> = self
            .fringe
            .now
            .iter()
            .map(|i| self.cache[*i].estimate)
            .collect(); */

            /* let mut h: HashMap<u32, i32> = HashMap::new();
            for g in self
                .fringe
                .now
                .iter()
                .map(|i| self.cache[*i].estimate.floor() as u32)
            {
                h.entry(g).and_modify(|counter| *counter += 1).or_insert(1);
            }
            let mut v: Vec<_> = h.iter().collect();
            v.sort(); */

            /* println!(
                "v: {}\t{:<12} {:?}",
                self.fringe.now.len(),
                ((*v[0].1 as f32) / (v.iter().map(|(_, c)| **c).sum::<i32>() as f32)),
                v
            ); */
            // println!("l: {}", v.len());
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
            _ => {}
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
            .neighbors(node as usize)
            .filter_map(|(child, c)| self.cache.check(child, node, *c))
            .for_each(|child| self.fringe.push_now(child));
    }

    /// Reconstruct path that was found
    fn construct_path(&self) -> Vec<usize> {
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

        path.iter().map(|n| *n as usize).collect()
    }

    pub fn add_to_printable(&self, mut print: Printable) -> Printable {
        self.fringe
            .buckets
            .iter()
            .flatten()
            .for_each(|n| print.add_oldlater((*n).try_into().unwrap()));
        self.fringe[self.fringe.current]
            .iter()
            .for_each(|n| print.add_inlater((*n).try_into().unwrap()));
        self.fringe
            .now
            .iter()
            .for_each(|n| print.add_inopen((*n).try_into().unwrap()));
        self.cache
            .cache
            .iter()
            .enumerate()
            .filter(|(_, n): &(usize, &CacheValue)| n.closed)
            .for_each(|(i, _)| print.add_inclosed(i));
        /*
        self.cache
            .cache
            .iter()
            .enumerate()
            .for_each(|(i, n): (usize, &CacheValue)| match n.status {
                Status::Now => print.add_inopen(i),
                Status::Later(b) => {
                    if self.fringe.current == b.into() {
                        print.add_inlater(i)
                    } else {
                        print.add_oldlater(i)
                    }
                }
                /* Status::Later => {
                    if self.cache.iteration == n.later {
                        print.add_inlater(i)
                    } else {
                        print.add_oldlater(i)
                    }
                } */
                Status::Closed => print.add_inclosed(i),
                Status::NotVisited => {}
            });
            */

        print
    }
}
