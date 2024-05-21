use crate::graph::*;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Diagonal octile distance from current node to goal.
/// This is a grid specific method.
fn heuristic(current_x: usize, current_y: usize, goal_x: usize, goal_y: usize) -> f32 {
    let x_distance: f32 = ((current_x as f32) - (goal_x as f32)).abs();
    let y_distance: f32 = ((current_y as f32) - (goal_y as f32)).abs();

    if x_distance > y_distance {
        return (x_distance - y_distance) + 2.0_f32.sqrt() * y_distance;
    } else {
        return (y_distance - x_distance) + 2.0_f32.sqrt() * x_distance;
    }
}

/// Cell with a weight. IMPORTANT: lower weights are ordered as greater.
/// This allows us to use binary heap
struct WeightedCell {
    x: usize,
    y: usize,
    weight: f32,
}

impl WeightedCell {
    fn new(x: usize, y: usize, weight: f32) -> WeightedCell {
        WeightedCell { x, y, weight }
    }
}

impl PartialEq for WeightedCell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.weight == other.weight
    }
}

impl Eq for WeightedCell {}

impl Ord for WeightedCell {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.weight > other.weight {
            Ordering::Equal
        } else if self.weight < other.weight {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for WeightedCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// struct Min_heap {
//     heap: BinaryHeap<WeightedCell>,
// }

// impl Min_heap {
//     fn new() -> Min_heap {
//         Min_heap {
//             heap: BinaryHeap::new(),
//         }
//     }

//     fn push(&mut self, x: usize, y: usize, weight: f32) {
//         self.heap.push(WeightedCell { x,y , weight  });
//     }

//     fn pop(&mut self) -> Option<((usize, usize), f32)> {
//         if let Some((weight, (x, y))) = self.heap.pop() {
//             return Some(((x, y), Reverse(weight)));
//         } else {
//             return None;
//         }
//     }
// }

pub fn a_star_simple(
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    graph: impl Graph,
) -> Option<Vec<(usize, usize)>> {
    let h = |x: usize, y: usize| heuristic(x, y, goal_x, goal_y);

    let mut open_set: BinaryHeap<WeightedCell> =
        BinaryHeap::from([WeightedCell::new(start_x, start_y, 0.0)]);
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(
        (start_x, start_y),
        heuristic(start_x, start_y, goal_x, goal_y),
    );

    while let Some(WeightedCell { x, y, .. }) = open_set.pop() {
        if x == goal_x && y == goal_y {
            break;
        }
        println!("{} {} {:?}", x, y, graph.neighbors(x, y));

        for ((x1, y1), w1) in graph.neighbors(x, y).unwrap() {
            // We can unwrap this since current node always has a cost value
            let new_cost = cost_so_far.get(&(x, y)).unwrap() + w1;
            match cost_so_far.get(&(*x1, *y1)) {
                Some(cost) if new_cost > *cost => {}
                _ => {
                    cost_so_far.insert((*x1, *y1), new_cost);
                    let priority = new_cost + h(*x1, *y1);
                    open_set.push(WeightedCell::new(*x1, *y1, priority));
                    came_from.insert((*x1, *y1), (x, y));
                }
            };
        }
    }

    println!("{:?}", came_from);
    println!("{:?}", cost_so_far);
    let path = reconstruct_path((start_x, start_y), (goal_x, goal_y), came_from);
    if path.is_empty() {
        return None;
    } else {
        return Some(path);
    }
}

/*
def reconstruct_path(came_from: dict[Location, Location],
                     start: Location, goal: Location) -> list[Location]:

    current: Location = goal
    path: list[Location] = []
    if goal not in came_from: # no path was found
        return []
    while current != start:
        path.append(current)
        current = came_from[current]
    path.append(start) # optional
    path.reverse() # optional
    return path
*/

fn reconstruct_path(
    start: (usize, usize),
    goal: (usize, usize),
    came_from: HashMap<(usize, usize), (usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut current = goal;
    let mut path: Vec<(usize, usize)> = vec![];

    if !came_from.contains_key(&goal) {
        return path;
    }

    while current != start {
        path.push(current);
        if let Some(next) = came_from.get(&current) {
            current = *next;
        } else {
            return vec![];
        }
    }
    path.push(start);
    path.reverse();

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heuristic_works_diagonally() {
        let result = heuristic(0, 0, 1, 1);
        let expected: f32 = 2.0_f32.sqrt();
        assert_eq!(expected, result);
    }
    #[test]
    fn heuristic_works_downwards() {
        let result = heuristic(0, 0, 0, 1);
        let expected: f32 = 1.0_f32;
        assert_eq!(expected, result);
    }
}
