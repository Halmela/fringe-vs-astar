use super::WeightedCell;
use crate::Node;
use std::collections::BinaryHeap;

/// BinaryHeap augmented with key update for a node
pub struct Frontier {
    heap: BinaryHeap<WeightedCell>,
    smallest_found: Vec<Option<f32>>,
}

impl Frontier {
    /// Create a heap and "memory" that could contain the whole graph and initialize it with starting node
    pub fn new(start: Node, size: usize) -> Frontier {
        let mut heap: BinaryHeap<WeightedCell> = BinaryHeap::with_capacity(size);
        heap.push(WeightedCell::new(start, 0.0));

        let mut smallest_found: Vec<Option<f32>> = vec![None; size];

        smallest_found[start as usize] = Some(0.0);
        Frontier {
            heap,
            smallest_found,
        }
    }

    /// Push a value to the heap, if it was not already there or if new priority is higher than the old
    pub fn push(&mut self, node: Node, weight: f32) -> bool {
        match self.smallest_found[node as usize] {
            Some(w) if w <= weight => {
                return false;
            }
            _ => {
                self.heap.push(WeightedCell::new(node, weight));
            }
        }

        self.smallest_found[node as usize] = Some(weight);
        true
    }

    // Provide node with the highest priority
    pub fn pop(&mut self) -> Option<Node> {
        loop {
            if let Some(WeightedCell { node, weight }) = self.heap.pop() {
                if self.smallest_found[node as usize].is_some_and(|w| w == weight) {
                    return Some(node);
                } else {
                    continue;
                }
            } else {
                return None;
            }
        }
    }
}
