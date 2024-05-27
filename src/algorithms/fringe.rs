use crate::algorithms::heuristic;
use crate::structures::{Fringe, Graph};
use std::collections::VecDeque;

pub struct FringeSearch<'a> {
    fringe: Fringe,
    cache: Vec<Vec<(f64, (usize, usize))>>,
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    graph: &'a Box<dyn Graph>,
}

impl<'a> FringeSearch<'a> {
    pub fn new(
        start_x: usize,
        start_y: usize,
        goal_x: usize,
        goal_y: usize,
        graph: &'a Box<dyn Graph>,
    ) -> Self {
        let fringe = Fringe::new(start_x, start_y, graph.get_width(), graph.get_height());

        let mut cache: Vec<Vec<(f64, (usize, usize))>> = vec![];
        for x in 0..graph.get_width() {
            cache.push(vec![]);
            for y in 0..graph.get_height() {
                cache[x].push((f64::MAX, (x, y)));
            }
        }
        cache[start_x][start_y].0 = 0.0;

        FringeSearch {
            fringe,
            cache,
            start_x,
            start_y,
            goal_x,
            goal_y,
            graph,
        }
    }

    pub fn solve(mut self) -> Option<(Vec<(usize, usize)>, f64)> {
        let diagonal_cost = 2.0_f64.sqrt();
        let h = |x: usize, y: usize| heuristic(x, y, self.goal_x, self.goal_y, diagonal_cost);

        let mut f_limit = h(self.start_x, self.start_y);
        let mut found = false;

        while !(found || self.fringe.now_is_empty()) {
            let mut f_min = f64::MAX;
            while let Some((x, y)) = self.fringe.pop_now() {
                let g = self.cache[x][y].0;
                let f = g + h(x, y);

                if f > f_limit {
                    if f < f_min {
                        f_min = f;
                    }
                    self.fringe.push_later(x, y);
                    continue;
                }

                if (x, y) == (self.goal_x, self.goal_y) {
                    found = true;
                    break;
                }

                for ((x1, y1), cost) in self.graph.neighbors(x, y).unwrap() {
                    let g_new = g + cost;
                    if g_new >= self.cache[*x1][*y1].0 {
                        continue;
                    }
                    self.fringe.push_now(*x1, *y1);
                    self.cache[*x1][*y1] = (g_new, (x, y));
                }
            }
            f_limit = f_min;
            self.fringe.later_to_now();
        }

        if found {
            let cost = self.cache[self.goal_x][self.goal_y].0;
            return Some((
                construct_path(
                    self.start_x,
                    self.start_y,
                    self.goal_x,
                    self.goal_y,
                    self.cache,
                ),
                cost,
            ));
        }
        None
    }
}

/// Reconstruct path that was found
fn construct_path(
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    cache: Vec<Vec<(f64, (usize, usize))>>,
) -> Vec<(usize, usize)> {
    let mut path = vec![(goal_x, goal_y)];
    loop {
        let (x, y) = path[path.len() - 1];
        let new = cache[x][y].1;
        path.push(new);

        if (new) == (start_x, start_y) {
            break;
        }
    }
    path.reverse();

    path
}
