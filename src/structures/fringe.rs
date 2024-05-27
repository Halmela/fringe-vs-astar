use std::collections::VecDeque;

pub struct Fringe {
    now: VecDeque<(usize, usize)>,
    later: VecDeque<(usize, usize)>,
    in_fringe: Vec<Vec<(bool, bool)>>,
}

impl Fringe {
    pub fn new(start_x: usize, start_y: usize, width: usize, height: usize) -> Self {
        let mut now = VecDeque::with_capacity(width); // Surely it won't be bigger
        now.push_front((start_x, start_y));
        let later = VecDeque::with_capacity(width);

        let mut in_fringe: Vec<Vec<(bool, bool)>> = vec![];

        for x in 0..width {
            in_fringe.push(vec![]);
            for _ in 0..height {
                in_fringe[x].push((false, false));
            }
        }
        in_fringe[start_x][start_y].0 = true;

        Fringe {
            now,
            later,
            in_fringe,
        }
    }

    pub fn push_now(&mut self, x: usize, y: usize) {
        let i_f = self.in_fringe[x][y];
        if i_f.0 {
            self.now = delete_from_fringe(x, y, self.now.clone());
        }
        if i_f.1 {
            self.later = delete_from_fringe(x, y, self.later.clone());
            self.in_fringe[x][y].1 = false;
        }
        self.now.push_front((x, y));
        self.in_fringe[x][y].0 = true;
    }

    pub fn push_later(&mut self, x: usize, y: usize) {
        self.in_fringe[x][y] = (false, true);
        self.later.push_back((x, y));
    }

    pub fn pop_now(&mut self) -> Option<(usize, usize)> {
        if let Some((x, y)) = self.now.pop_front() {
            self.in_fringe[x][y].0 = false;
            Some((x, y))
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

fn delete_from_fringe(
    x: usize,
    y: usize,
    mut fringe: VecDeque<(usize, usize)>,
) -> VecDeque<(usize, usize)> {
    fringe.drain(..).filter(|xy| xy != &(x, y)).collect()
}
