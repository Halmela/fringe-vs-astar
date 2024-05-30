use std::collections::VecDeque;

pub struct Fringe {
    now: VecDeque<usize>,
    later: VecDeque<usize>,
    in_fringe: Vec<bool>,
}

impl Fringe {
    pub fn new(start: usize, size: usize) -> Self {
        let mut now = VecDeque::with_capacity(size / 2); // Surely it won't be bigger
        now.push_front(start);
        let later = VecDeque::with_capacity(size / 2);

        let mut in_fringe: Vec<bool> = vec![];

        for _ in 0..size {
            in_fringe.push(false);
        }
        in_fringe[start] = true;

        Fringe {
            now,
            later,
            in_fringe,
        }
    }

    pub fn push_now(&mut self, i: usize) {
        self.now.push_front(i);
        self.in_fringe[i] = true;
    }

    pub fn push_later(&mut self, i: usize) {
        self.in_fringe[i] = true;
        self.later.push_back(i);
    }

    pub fn pop_now(&mut self) -> Option<usize> {
        if let Some(i) = self.now.pop_front() {
            if self.in_fringe[i] {
                self.in_fringe[i] = false;
                Some(i)
            } else {
                self.pop_now()
            }
        } else {
            None
        }
    }

    pub fn later_to_now(&mut self) {
        self.now = self.later.clone();
        self.later = VecDeque::with_capacity(self.now.capacity());
    }

    pub fn now_is_empty(&self) -> bool {
        self.now.is_empty()
    }
}
