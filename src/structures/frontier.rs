use super::WeightedCell;
use std::collections::BinaryHeap;

pub struct Frontier {
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
