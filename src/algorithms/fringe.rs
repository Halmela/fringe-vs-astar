use crate::algorithms::fringe::cache::*;
use crate::algorithms::heuristic;
use crate::index_to_xy;
use crate::structures::AdjacencyListGraph;
use crate::structures::Fringe;
use crate::Node;

mod cache;

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

    pub fn get(&self, i: Node) -> f32 {
        heuristic(index_to_xy(i as usize, self.width), self.goal)
    }
}

pub struct FringeSearch<'a> {
    fringe: Fringe,
    cache: Cache, // (cost,parent,heuristic,in_fringe)
    start: Node,
    goal: Node,
    graph: &'a AdjacencyListGraph,
}

impl<'a> FringeSearch<'a> {
    pub fn new(start: usize, goal: usize, graph: &'a AdjacencyListGraph) -> Self {
        let size = graph.get_width() * graph.get_height();
        let fringe = Fringe::new(start.try_into().unwrap(), size);
        let heuristic = Heuristic::new(goal, graph.get_width());
        let cache = Cache::new(start, size, heuristic);

        FringeSearch {
            fringe,
            cache,
            start: start.try_into().unwrap(),
            goal: goal.try_into().unwrap(),
            graph,
        }
    }

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

    fn process_node(&mut self, node: Node) -> Option<Node> {
        match self.cache.check_estimate(node) {
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

    fn prepare_next_iteration(&mut self) -> bool {
        self.cache.refresh_limits();
        self.fringe.later_to_now()
    }

    fn process_neighbors(&mut self, node: Node) {
        self.graph
            .neighbors(node as usize)
            .filter_map(|(child, c)| self.cache.check(child, node, *c))
            .for_each(|child| self.fringe.push_now(child));
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

        path.drain(..).map(|n| n as usize).collect()
    }
}
