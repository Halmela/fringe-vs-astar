use super::WeightedCell;
use crate::Node;
use std::collections::BinaryHeap;

/// BinaryHeap augmented with key update for a node
pub struct Frontier {
    heap: BinaryHeap<WeightedCell>,
}

impl Frontier {
    /// Create a heap and "memory" that could contain the whole graph and initialize it with starting node
    pub fn new(start: Node, size: usize) -> Frontier {
        let mut heap: BinaryHeap<WeightedCell> = BinaryHeap::with_capacity(size);
        heap.push(WeightedCell::new(start, 0.0));

        Frontier { heap }
    }

    /// Push a value to the heap, if it was not already there or if new priority is higher than the old
    pub fn push(&mut self, node: Node, weight: f32) {
        self.heap.push(WeightedCell::new(node, weight));
    }

    // Provide node with the highest priority
    pub fn pop(&mut self) -> Option<Node> {
        if let Some(WeightedCell { node, .. }) = self.heap.pop() {
            Some(node)
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.heap.iter().map(|w| &w.node)
    }

    pub fn top3(&self) -> (Option<Node>, Option<Node>, Option<Node>) {
        let v: Vec<Node> = self
            .heap
            .to_owned()
            .into_sorted_vec()
            .iter()
            .map(|w| w.node)
            .collect();

        (
            v.last().copied(),
            v.get(v.len() - 2).copied(),
            v.get(v.len() - 3).copied(),
        )
    }

    pub fn size(&self) -> usize {
        self.heap.len()
    }
}
