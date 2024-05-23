use crate::graph::{self, *};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

/// Diagonal octile distance from current node to goal.
/// This is a grid specific method.
fn heuristic(
    current_x: usize,
    current_y: usize,
    goal_x: usize,
    goal_y: usize,
    diagonal_cost: f64,
) -> f64 {
    let x_distance: f64 = ((current_x as f64) - (goal_x as f64)).abs();
    let y_distance: f64 = ((current_y as f64) - (goal_y as f64)).abs();

    if x_distance > y_distance {
        return (x_distance - y_distance) + diagonal_cost * y_distance;
    } else {
        return (y_distance - x_distance) + diagonal_cost * x_distance;
    }
}

/// Cell with a weight. IMPORTANT: lower weights are ordered as greater.
/// This allows us to use binary heap
#[derive(Debug)]
struct WeightedCell {
    x: usize,
    y: usize,
    weight: f64,
}

impl WeightedCell {
    fn new(x: usize, y: usize, weight: f64) -> WeightedCell {
        WeightedCell { x, y, weight }
    }

    fn get_xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }

    fn change_weight(&mut self, weight: f64) {
        self.weight = weight;
    }
}

impl fmt::Display for WeightedCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "W(({}, {}), {})", self.x, self.y, self.weight)
    }
}

impl PartialEq for WeightedCell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.weight == other.weight
    }
}

impl Eq for WeightedCell {}

/// We want binary heap to be minimum heap,
/// so we order these in "reverse" order
impl Ord for WeightedCell {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.weight > other.weight {
            Ordering::Less
        } else if self.weight < other.weight {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for WeightedCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStar<G: Graph> {
    frontier: Frontier,
    history: Vec<Vec<(Option<(usize, usize)>, Option<f64>)>>,
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    graph: G,
}

impl<G: Graph> AStar<G> {
    pub fn new(start_x: usize, start_y: usize, goal_x: usize, goal_y: usize, graph: G) -> Self {
        let frontier = Frontier::new(start_x, start_y, graph.get_width(), graph.get_height());
        /*
        let mut frontier: BinaryHeap<WeightedCell> =
            BinaryHeap::with_capacity(graph.get_height() * graph.get_width());
        frontier.push(WeightedCell::new(start_x, start_y, 0.0));
        */

        // (previous xy, current cost, current prority)

        let mut history: Vec<Vec<(Option<(usize, usize)>, Option<f64>)>> = vec![];
        for x in 0..graph.get_width() {
            history.push(vec![]);
            for _ in 0..graph.get_height() {
                history[x].push((None, None));
            }
        }

        history[start_x][start_y] = (None, Some(0.0));

        AStar {
            frontier,
            history,
            start_x,
            start_y,
            goal_x,
            goal_y,
            graph,
        }
    }

    pub fn solve(mut self) -> Option<Vec<(usize, usize)>> {
        let d_c = 2.0_f64.sqrt();
        let h = |x: usize, y: usize| heuristic(x, y, self.goal_x, self.goal_y, d_c);

        while let Some((x, y)) = self.frontier.pop() {
            if x == self.goal_x && y == self.goal_y {
                return Some(self.construct_path());
            }

            let current_cost = self.history[x][y].1.unwrap();

            for ((x1, y1), w1) in self.graph.neighbors(x, y).unwrap() {
                let new_cost = current_cost + w1;
                let priority = new_cost + h(*x1, *y1);
                if self.frontier.push(*x1, *y1, priority) {
                    self.history[*x1][*y1] = (Some((x, y)), Some(new_cost));
                }
            }
        }
        // If frontier is empty, no path can be found
        None
    }

    fn construct_path(&self) -> Vec<(usize, usize)> {
        let mut path = vec![(self.goal_x, self.goal_y)];
        loop {
            let (x, y) = path[path.len() - 1];
            let new = self.history[x][y].0.unwrap();
            path.push(new);

            if (new) == (self.start_x, self.start_y) {
                break;
            }
        }
        path.reverse();

        println!("{}", self.history[self.goal_x][self.goal_y].1.unwrap());
        path
    }
}

struct Frontier {
    heap: BinaryHeap<WeightedCell>,
    smallest_found: Vec<Vec<Option<f64>>>,
}

impl Frontier {
    pub fn new(start_x: usize, start_y: usize, width: usize, height: usize) -> Frontier {
        let mut heap: BinaryHeap<WeightedCell> = BinaryHeap::with_capacity(height * width);
        heap.push(WeightedCell::new(start_x, start_y, 0.0));

        let mut smallest_found: Vec<Vec<Option<f64>>> = vec![];
        for x in 0..width {
            smallest_found.push(vec![]);
            for _ in 0..height {
                smallest_found[x].push(None);
            }
        }

        smallest_found[start_x][start_y] = Some(0.0);
        Frontier {
            heap,
            smallest_found,
        }
    }

    pub fn push(&mut self, x: usize, y: usize, weight: f64) -> bool {
        match self.smallest_found[x][y] {
            Some(w) if w < weight => false,
            Some(_) => {
                self.smallest_found[x][y] = Some(weight);
                self.replace((x, y), weight);
                true
            }
            None => {
                self.heap.push(WeightedCell::new(x, y, weight));
                self.smallest_found[x][y] = Some(weight);
                true
            }
        }
    }

    pub fn pop(&mut self) -> Option<(usize, usize)> {
        if let Some(WeightedCell { x, y, .. }) = self.heap.pop() {
            Some((x, y))
        } else {
            None
        }
    }

    fn replace(&mut self, xy: (usize, usize), value: f64) {
        self.heap = self
            .heap
            .drain()
            .map(|mut wc| {
                if wc.get_xy() == xy {
                    wc.change_weight(value)
                }
                wc
            })
            .collect()
    }
}

fn modify_heap(
    xy: (usize, usize),
    value: f64,
    mut heap: BinaryHeap<WeightedCell>,
) -> BinaryHeap<WeightedCell> {
    heap.drain()
        .map(|mut wc| {
            if wc.get_xy() == xy {
                wc.change_weight(value)
            }
            wc
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heuristic_works_diagonally() {
        let diagonal_cost = 2.0_f64.sqrt();
        let result = heuristic(0, 0, 1, 1, diagonal_cost);
        assert_eq!(diagonal_cost, result);
    }
    #[test]
    fn heuristic_works_downwards() {
        let diagonal_cost = 2.0_f64.sqrt();
        let result = heuristic(0, 0, 0, 1, diagonal_cost);
        assert_eq!(diagonal_cost, result);
    }
}
