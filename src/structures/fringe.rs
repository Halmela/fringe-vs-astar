use std::collections::VecDeque;

pub struct Fringe {
    now: VecDeque<usize>,
    later: VecDeque<usize>,
    in_fringe: Vec<(bool, bool)>,
}

impl Fringe {
    pub fn new(start: usize, size: usize) -> Self {
        let mut now = VecDeque::with_capacity(size / 2); // Surely it won't be bigger
        now.push_front(start);
        let later = VecDeque::with_capacity(size / 2);

        let mut in_fringe: Vec<(bool, bool)> = vec![];

        for i in 0..size {
            in_fringe[i] = (false, false);
        }
        in_fringe[start].0 = true;

        Fringe {
            now,
            later,
            in_fringe,
        }
    }

    pub fn push_now(&mut self, i: usize) {
        let i_f = self.in_fringe[i];
        if i_f.0 {
            self.now = delete_from_fringe(&i, self.now.clone());
        }
        if i_f.1 {
            self.later = delete_from_fringe(&i, self.later.clone());
            self.in_fringe[i].1 = false;
        }
        self.now.push_front(i);
        self.in_fringe[i].0 = true;
    }

    pub fn push_later(&mut self, i: usize) {
        self.in_fringe[i] = (false, true);
        self.later.push_back(i);
    }

    pub fn pop_now(&mut self) -> Option<usize> {
        if let Some(i) = self.now.pop_front() {
            self.in_fringe[i].0 = false;
            Some(i)
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

fn delete_from_fringe(i: &usize, mut fringe: VecDeque<usize>) -> VecDeque<usize> {
    fringe.drain(..).filter(|j| j != i).collect()
}
