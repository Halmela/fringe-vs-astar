use crate::index_to_xy;
use crate::structures::{Fringe, Graph};

use super::heuristic;

pub struct FringeSearch<'a> {
    fringe: Fringe,
    cache: Vec<(f64, usize, bool, Option<f64>)>,
    start: usize,
    goal: usize,
    graph: &'a Box<dyn Graph>,
}

impl<'a> FringeSearch<'a> {
    pub fn new(start: usize, goal: usize, graph: &'a Box<dyn Graph>) -> Self {
        let size = graph.get_width() * graph.get_height();
        let fringe = Fringe::new(start, size);

        let mut cache: Vec<(f64, usize, bool, Option<f64>)> =
            std::iter::repeat((f64::MAX, 0, false, None))
                .take(size)
                .collect();
        cache[start].0 = 0.0;
        cache[start].2 = true;

        FringeSearch {
            fringe,
            cache,
            start,
            goal,
            graph,
        }
    }

    pub fn solve(mut self) -> Option<(Vec<usize>, f64)> {
        let ixy = |i: usize| index_to_xy(i, self.graph.get_width());
        let gxy = ixy(self.goal);
        let h = |i: usize| heuristic(ixy(i), gxy);

        let mut f_limit = h(self.start);
        let mut found = false;

        while !(found || self.fringe.is_empty()) {
            let mut f_min = f64::MAX;
            while let Some(node) = self.fringe.pop_now() {
                if !self.cache[node].2 {
                    continue;
                }
                let cost = self.cache[node].0;
                let to_goal: f64;
                if let Some(tg) = self.cache[node].3 {
                    to_goal = tg;
                } else {
                    to_goal = h(node);
                    self.cache[node].3 = Some(to_goal);
                }
                let estimate = cost + to_goal;

                if estimate > f_limit {
                    if estimate < f_min {
                        f_min = estimate;
                    }
                    self.fringe.push_later(node);
                    continue;
                }

                if node == self.goal {
                    found = true;
                    break;
                }

                for (child, old_cost) in self.graph.neighbors(node) {
                    let new_cost = cost + old_cost;
                    if new_cost >= self.cache[*child].0 {
                        continue;
                    }
                    self.fringe.push_now(*child);
                    self.cache[*child].0 = new_cost;
                    self.cache[*child].1 = node;
                    self.cache[*child].2 = true;
                }
                self.cache[node].2 = false;
                // println!("");
            }
            f_limit = f_min;
            self.fringe.later_to_now();
            // println!("{}\n\n", f_limit);
        }

        if found {
            let cost = self.cache[self.goal].0;
            return Some((construct_path(self.start, self.goal, self.cache), cost));
        }
        None
    }
}

/// Reconstruct path that was found
fn construct_path(
    start: usize,
    goal: usize,
    cache: Vec<(f64, usize, bool, Option<f64>)>,
) -> Vec<usize> {
    let mut path = vec![(goal)];
    loop {
        let i = path[path.len() - 1];
        let new = cache[i].1;
        path.push(new);

        if new == start {
            break;
        }
    }
    path.reverse();

    path
}
