use crate::algorithms::heuristic;
use crate::structures::{Frontier, Graph};

pub struct AStar<'a> {
    frontier: Frontier,
    history: Vec<Vec<(Option<(usize, usize)>, Option<f64>)>>,
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    graph: &'a Box<dyn Graph>,
}

impl<'a> AStar<'a> {
    pub fn new(
        start_x: usize,
        start_y: usize,
        goal_x: usize,
        goal_y: usize,
        graph: &'a Box<dyn Graph>,
    ) -> Self {
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

        println!(
            "Result:\n\t{}",
            self.history[self.goal_x][self.goal_y].1.unwrap()
        );
        path
    }
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
