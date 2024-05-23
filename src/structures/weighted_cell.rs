use std::cmp::Ordering;
use std::fmt;

/// Cell with a weight. IMPORTANT: lower weights are ordered as greater.
/// This allows us to use binary heap
#[derive(Debug)]
pub struct WeightedCell {
    pub x: usize,
    pub y: usize,
    weight: f64,
}

impl WeightedCell {
    pub fn new(x: usize, y: usize, weight: f64) -> WeightedCell {
        WeightedCell { x, y, weight }
    }

    pub fn get_xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn change_weight(&mut self, weight: f64) {
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
