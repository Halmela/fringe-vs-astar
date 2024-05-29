use super::WeightedCell;
use std::collections::BinaryHeap;

/// BinaryHeap augmented with key update for a node
pub struct Frontier {
    heap: BinaryHeap<WeightedCell>,
    smallest_found: Vec<Option<f64>>,
}

impl Frontier {
    /// Create a heap and "memory" that could contain the whole graph and initialize it with starting node
    pub fn new(start: usize, size: usize) -> Frontier {
        let mut heap: BinaryHeap<WeightedCell> = BinaryHeap::with_capacity(size);
        heap.push(WeightedCell::new(start, 0.0));

        let mut smallest_found: Vec<Option<f64>> = vec![];
        for _ in 0..size {
            smallest_found.push(None);
        }

        smallest_found[start] = Some(0.0);
        Frontier {
            heap,
            smallest_found,
        }
    }

    /// Push a value to the heap, if it was not already there or if new priority is higher than the old
    pub fn push(&mut self, i: usize, weight: f64) -> bool {
        match self.smallest_found[i] {
            Some(w) if w < weight => {
                return false;
            }
            Some(_) => {
                self.replace(i, weight);
            }
            None => {
                self.heap.push(WeightedCell::new(i, weight));
            }
        }

        self.smallest_found[i] = Some(weight);
        return true;
    }

    // Provide node with the highest priority
    pub fn pop(&mut self) -> Option<usize> {
        if let Some(WeightedCell { i, .. }) = self.heap.pop() {
            Some(i)
        } else {
            None
        }
    }

    // Replace old value in heap with the new one
    fn replace(&mut self, i: usize, value: f64) {
        self.heap = self
            .heap
            .drain()
            .map(|mut wc| {
                if wc.get_i() == i {
                    wc.change_weight(value)
                }
                wc
            })
            .collect()
    }
}
