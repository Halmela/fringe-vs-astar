use crate::graph::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

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
#[derive(Debug)]
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

pub fn a_star_simple(
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    graph: impl Graph,
) -> Option<Vec<(usize, usize)>> {
    let h = |x: usize, y: usize| heuristic(x, y, goal_x, goal_y);

    let mut frontier: BinaryHeap<WeightedCell> =
        BinaryHeap::with_capacity(graph.get_height() * graph.get_width());
    frontier.push(WeightedCell::new(start_x, start_y, 0.0));

    // (previous_xy, current_cost, lowest_prority)
    let mut history: Vec<Vec<(Option<(usize, usize)>, Option<f32>, Option<f32>)>> = vec![];
    for x in 0..graph.get_width() {
        history.push(vec![]);
        for _ in 0..graph.get_height() {
            history[x].push((None, None, None));
        }
    }
    println!("{} {}", history.len(), history[0].len());
    history[start_x][start_y] = (None, Some(0.0), Some(0.0));

    while let Some(WeightedCell { x, y, .. }) = frontier.pop() {
        //println!("{x} {y}");
        //println!("{:?}", graph.neighbors(x, y).unwrap());
        if x == goal_x && y == goal_y {
            break;
        }
        let current_cost = history[x][y].1.unwrap();

        for ((x1, y1), w1) in graph.neighbors(x, y).unwrap() {
            let new_cost = current_cost + w1;
            //println!("\t{x1} {y1}");
            let old_cost = history[*x1][*y1].1;
            match old_cost {
                Some(cost) if new_cost > cost => {}
                _ => {
                    let priority = new_cost + h(*x1, *y1);
                    match history[*x1][*y1].2 {
                        Some(p) if p < priority => {}
                        Some(_) => {
                            // println!("\nchange {x1} {y1}");
                            history[*x1][*y1] = (Some((x, y)), Some(new_cost), Some(priority));
                            frontier = delete_from_heap((x1, y1), frontier);
                            frontier.push(WeightedCell::new(*x1, *y1, priority));
                        }
                        None => {
                            frontier.push(WeightedCell::new(*x1, *y1, priority));
                            history[*x1][*y1] = (Some((x, y)), Some(new_cost), Some(priority));
                        }
                    }
                }
            };
            /*
            println!("[");
            for w in &frontier {
                print!("{w}, ");
            }
            println!("\n]");
            */
        }
    }

    let mut path = vec![(goal_x, goal_y)];
    loop {
        let (x, y) = path[path.len() - 1];
        let new = history[x][y].0.unwrap();
        path.push(new);

        if (new) == (start_x, start_y) {
            break;
        }
    }
    path.reverse();

    if path.is_empty() {
        return None;
    } else {
        println!("{}", history[goal_x][goal_y].1.unwrap());
        return Some(path);
    }
}

fn delete_from_heap(
    xy: (&usize, &usize),
    mut heap: BinaryHeap<WeightedCell>,
) -> BinaryHeap<WeightedCell> {
    heap.drain()
        .filter(|WeightedCell { x, y, .. }| (x, y) != xy)
        .collect()
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

fn _reconstruct_path(
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
