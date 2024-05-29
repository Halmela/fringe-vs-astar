use crate::algorithms::heuristic;
use crate::structures::{graph, Fringe, Graph};
use crate::{index_to_xy, xy_to_index};

pub struct FringeSearch<'a> {
    fringe: Fringe,
    cache: Vec<(f64, usize)>,
    start: usize,
    goal: usize,
    graph: &'a Box<dyn Graph>,
}

impl<'a> FringeSearch<'a> {
    pub fn new(start: usize, goal: usize, graph: &'a Box<dyn Graph>) -> Self {
        let fringe = Fringe::new(start, graph.get_width() * graph.get_height());

        let mut cache: Vec<(f64, usize)> = vec![];
        for x in 0..graph.get_width() {
            for y in 0..graph.get_height() {
                let i = xy_to_index(x, y, graph.get_width());
                cache.push((f64::MAX, i));
            }
        }
        cache[start].0 = 0.0;

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
        let (goal_x, goal_y) = ixy(self.goal);
        let h = |i: usize| {
            let (x, y) = ixy(i);
            heuristic(x, y, goal_x, goal_y)
        };

        let mut f_limit = h(self.start);
        let mut found = false;

        while !(found || self.fringe.now_is_empty()) {
            let mut f_min = f64::MAX;
            while let Some(i) = self.fringe.pop_now() {
                let g = self.cache[i].0;
                let f = g + h(i);

                if f > f_limit {
                    if f < f_min {
                        f_min = f;
                    }
                    self.fringe.push_later(i);
                    continue;
                }

                if i == self.goal {
                    found = true;
                    break;
                }

                for (n, w1) in self.graph.neighbors(i) {
                    let g_new = g + w1;
                    if g_new >= self.cache[n].0 {
                        continue;
                    }
                    self.fringe.push_now(n);
                    self.cache[n] = (g_new, i);
                }
            }
            f_limit = f_min;
            self.fringe.later_to_now();
        }

        if found {
            let cost = self.cache[self.goal].0;
            return Some((construct_path(self.start, self.goal, self.cache), cost));
        }
        None
    }
}

/// Reconstruct path that was found
fn construct_path(start: usize, goal: usize, cache: Vec<(f64, usize)>) -> Vec<usize> {
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
