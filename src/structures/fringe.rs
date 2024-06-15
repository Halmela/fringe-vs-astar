use crate::Node;

/// Datastructure used for storing nodes in Fringe search.
/// Does not hold any data on existence of a node in fringe
#[derive(Debug)]
pub struct Fringe {
    now: Vec<Node>,
    later: Vec<Node>,
}

impl Fringe {
    /// Create new Fringe
    pub fn new(start: Node, size: usize) -> Self {
        let mut now = Vec::with_capacity(size);
        now.push(start);
        let later = Vec::with_capacity(size);

        Fringe { now, later }
    }

    /// Push node to be processed in this iteration
    pub fn push_now(&mut self, node: Node) {
        self.now.push(node);
    }

    /// Push node to be processed in later iteration
    pub fn push_later(&mut self, node: Node) {
        self.later.push(node);
    }

    /// Try to give a node from now list
    pub fn pop_now(&mut self) -> Option<Node> {
        // Try to take first element

        self.now.pop()
    }

    /// Are there any nodes to be processed?
    pub fn is_empty(&self) -> bool {
        self.now.is_empty()
    }

    /// Swap later to now
    pub fn later_to_now(&mut self) -> bool {
        if self.later.is_empty() {
            return false;
        }
        self.now
            .reserve(self.later.capacity() - self.now.capacity());
        self.now.extend(self.later.iter().copied());

        self.later.clear();
        return true;
    }
}
