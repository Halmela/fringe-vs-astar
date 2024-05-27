use crate::algorithms::heuristic;
use crate::structures::Graph;
use std::collections::VecDeque;

pub struct FringeSearch {}

impl FringeSearch {
    pub fn new() -> Self {
        todo!()
    }

    pub fn solve(mut self) {
        todo!()
    }
}

struct Fringe {
    now: VecDeque<(usize, usize)>,
    later: VecDeque<(usize, usize)>,
    in_fringe: Vec<Vec<(bool, bool)>>,
}

impl Fringe {
    pub fn new(start_x: usize, start_y: usize, width: usize, height: usize) -> Self {
        let now = VecDeque::from(vec![(start_x, start_y)]);
        let later = VecDeque::new();

        let mut in_fringe: Vec<Vec<(bool, bool)>> = vec![];

        for x in 0..width {
            in_fringe.push(vec![]);
            for _ in 0..height {
                in_fringe[x].push((false, false));
            }
        }
        in_fringe[start_x][start_y].0 = true;

        Fringe {
            now,
            later,
            in_fringe,
        }
    }

    pub fn push_now(&mut self, x: usize, y: usize) {
        let i_f = self.in_fringe[x][y];
        if i_f.0 {
            self.now = delete_from_fringe(x, y, self.now.clone());
        }
        if i_f.1 {
            self.later = delete_from_fringe(x, y, self.later.clone());
            self.in_fringe[x][y].1 = false;
        }
        self.now.push_front((x, y));
        self.in_fringe[x][y].0 = true;
    }

    pub fn push_later(&mut self, x: usize, y: usize) {
        self.in_fringe[x][y] = (false, true);
        self.later.push_back((x, y));
    }

    pub fn pop_now(&mut self) -> Option<(usize, usize)> {
        if let Some((x, y)) = self.now.pop_front() {
            self.in_fringe[x][y].0 = false;
            Some((x, y))
        } else {
            None
        }
    }

    pub fn later_to_now(&mut self) {
        self.now = self.later.clone();
        self.later = VecDeque::new();
    }

    pub fn now_is_empty(&self) -> bool {
        self.now.is_empty()
    }
}

pub fn fringe_simple<'a>(
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    graph: &'a Box<dyn Graph>,
) -> Option<(Vec<(usize, usize)>, f64)> {
    let diagonal_cost = 2.0_f64.sqrt();
    let h = |x: usize, y: usize| heuristic(x, y, goal_x, goal_y, diagonal_cost);

    let mut fringe = Fringe::new(start_x, start_y, graph.get_width(), graph.get_height());

    let mut cache: Vec<Vec<(Option<f64>, Option<(usize, usize)>)>> = vec![];
    for x in 0..graph.get_width() {
        cache.push(vec![]);
        for _ in 0..graph.get_height() {
            cache[x].push((None, None));
        }
    }
    cache[start_x][start_y] = (Some(0.0), None);
    let mut f_limit = h(start_x, start_y);
    let mut found = false;

    while !(found || fringe.now_is_empty()) {
        let mut f_min = f64::MAX;
        while let Some((x, y)) = fringe.pop_now() {
            let (g, ..) = cache[x][y];
            let g = g.unwrap();
            let f = g + h(x, y);

            if f > f_limit {
                if f < f_min {
                    f_min = f;
                }
                fringe.push_later(x, y);
                continue;
            }

            if (x, y) == (goal_x, goal_y) {
                found = true;
                break;
            }

            for ((x1, y1), cost) in graph.neighbors(x, y).unwrap() {
                let g_new = g + cost;
                if let (Some(g_cache), _) = cache[*x1][*y1] {
                    if g_new >= g_cache {
                        continue;
                    }
                }
                fringe.push_now(*x1, *y1);
                cache[*x1][*y1] = (Some(g_new), Some((x, y)));
            }
        }
        f_limit = f_min;
        fringe.later_to_now();
    }

    if found {
        let cost = cache[goal_x][goal_y].0.unwrap();
        return Some((
            construct_path(start_x, start_y, goal_x, goal_y, cache),
            cost,
        ));
    }
    None
}

fn delete_from_fringe(
    x: usize,
    y: usize,
    mut fringe: VecDeque<(usize, usize)>,
) -> VecDeque<(usize, usize)> {
    fringe.drain(..).filter(|xy| xy != &(x, y)).collect()
}

/// Reconstruct path that was found
fn construct_path(
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    cache: Vec<Vec<(Option<f64>, Option<(usize, usize)>)>>,
) -> Vec<(usize, usize)> {
    let mut path = vec![(goal_x, goal_y)];
    loop {
        let (x, y) = path[path.len() - 1];
        let new = cache[x][y].1.unwrap();
        path.push(new);

        if (new) == (start_x, start_y) {
            break;
        }
    }
    path.reverse();

    path
}
