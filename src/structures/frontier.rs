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

        let mut smallest_found: Vec<Option<f64>> = std::iter::repeat(None).take(size).collect();

        smallest_found[start] = Some(0.0);
        Frontier {
            heap,
            smallest_found,
        }
    }

    /// Push a value to the heap, if it was not already there or if new priority is higher than the old
    pub fn push(&mut self, i: usize, weight: f64) -> bool {
        match self.smallest_found[i] {
            Some(w) if w <= weight => {
                return false;
            }
            _ => {
                self.heap.push(WeightedCell::new(i, weight));
            }
        }

        self.smallest_found[i] = Some(weight);
        return true;
    }

    // Provide node with the highest priority
    pub fn pop(&mut self) -> Option<usize> {
        loop {
            if let Some(WeightedCell { i, weight }) = self.heap.pop() {
                if self.smallest_found[i].is_some_and(|w| w == weight) {
                    return Some(i);
                } else {
                    continue;
                }
            } else {
                return None;
            }
        }
    }
}
