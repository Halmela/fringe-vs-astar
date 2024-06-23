use crate::Node;
use std::cmp::Ordering;
use std::fmt;

/// Cell with a weight. IMPORTANT: lower weights are ordered as greater.
/// This allows us to use binary heap
#[derive(Debug, Copy, Clone)]
pub struct WeightedCell {
    pub node: Node,
    pub weight: f32,
}

impl WeightedCell {
    /// Create a new cell
    pub fn new(node: Node, weight: f32) -> WeightedCell {
        WeightedCell { node, weight }
    }
}

impl fmt::Display for WeightedCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "W({}, {})", self.node, self.weight)
    }
}

impl PartialEq for WeightedCell {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.weight == other.weight
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
