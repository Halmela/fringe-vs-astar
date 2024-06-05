use std::cmp::max;

#[derive(Debug)]
pub struct Fringe {
    now: Vec<usize>,
    later: Vec<usize>,
}

impl Fringe {
    pub fn new(start: usize, size: usize) -> Self {
        let mut now = Vec::with_capacity(size);
        now.push(start);
        let later = Vec::with_capacity(size);

        Fringe { now, later }
    }

    pub fn push_now(&mut self, node: usize) {
        self.now.push(node);
    }

    pub fn push_later(&mut self, node: usize) {
        self.later.push(node);
    }

    pub fn pop_now(&mut self) -> Option<usize> {
        // Try to take first element

        self.now.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.now.is_empty()
    }

    pub fn later_to_now(&mut self) {
        self.now
            .reserve(self.later.capacity() - self.now.capacity());
        self.now.extend(self.later.iter().copied());

        self.later.clear();
    }
}
