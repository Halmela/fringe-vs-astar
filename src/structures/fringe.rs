use std::collections::VecDeque;

#[derive(Debug)]
pub struct Fringe {
    fringe: VecDeque<(usize, usize)>,            // (node_id, fringe_id)
    marker: Vec<(Option<usize>, Option<usize>)>, // marker[node_id] = (f_id in now, f_id in later)
    counter: usize,
    last_rotation: usize,
}

impl Fringe {
    pub fn new(start: usize, size: usize) -> Self {
        let mut fringe = VecDeque::with_capacity(size);
        fringe.push_front((start, 0));

        let mut marker: Vec<(Option<usize>, Option<usize>)> =
            std::iter::repeat((None, None)).take(size).collect();
        marker[start].0 = Some(0);

        Fringe {
            fringe,
            marker,
            counter: 1,
            last_rotation: 0,
        }
    }

    pub fn push_now(&mut self, i: usize) {
        self.fringe.push_front((i, self.counter));
        self.marker[i].0 = Some(self.counter);
        self.counter += 1;
    }

    pub fn push_later(&mut self, i: usize) {
        // println!("later {}: {:?}", i, self.in_fringe[i]);
        self.fringe.push_back((i, self.counter));
        self.marker[i].1 = Some(self.counter);
        self.counter += 1;
    }

    pub fn pop_now(&mut self) -> Option<usize> {
        loop {
            // Try to take first element
            if let Some((node, fringe_id)) = self.fringe.pop_front() {
                match self.marker[node] {
                    // Check if this is the correct Now-node
                    (Some(id), _) if id == fringe_id => {
                        self.marker[node].0 = None;
                        return Some(node);
                    }
                    // Check if this is the correct node from last rotation
                    (_, Some(id)) if id == fringe_id && id < self.last_rotation => {
                        self.marker[node].1 = None;
                        return Some(node);
                    }
                    // Push to front and refresh if it has been marked later this gen
                    (_, Some(id)) if id == fringe_id => {
                        self.last_rotation = self.counter;
                        self.fringe.push_front((node, fringe_id));
                        return None;
                    }
                    // Get a new node if the this is "old"
                    _ => continue,
                }
            }

            return None;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.fringe.is_empty()
    }
}
