use crate::Node;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl Bucket {
    fn add(self) -> Bucket {
        match self {
            Bucket::Zero => Bucket::One,
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::Three,
            Bucket::Three => Bucket::Four,
            Bucket::Four => Bucket::Five,
            Bucket::Five => Bucket::Six,
            Bucket::Six => Bucket::Seven,
            Bucket::Seven => Bucket::Zero,
        }
    }
}

impl From<f32> for Bucket {
    fn from(value: f32) -> Self {
        match (value as u32) % 8 {
            0 => Bucket::Zero,
            1 => Bucket::One,
            2 => Bucket::Two,
            3 => Bucket::Three,
            4 => Bucket::Four,
            5 => Bucket::Five,
            6 => Bucket::Six,
            _ => Bucket::Seven,
        }
    }
}

impl From<Bucket> for usize {
    fn from(bucket: Bucket) -> Self {
        match bucket {
            Bucket::Zero => 0,
            Bucket::One => 1,
            Bucket::Two => 2,
            Bucket::Three => 3,
            Bucket::Four => 4,
            Bucket::Five => 5,
            Bucket::Six => 6,
            Bucket::Seven => 7,
        }
    }
}

/// Datastructure used for storing nodes in Fringe search.
/// Does not hold any data on existence of a node in fringe
#[derive(Debug)]
pub struct Fringe {
    pub now: Vec<Node>,
    pub later: Vec<Node>,
    pub buckets: [Vec<Node>; 8],
    pub current: Bucket,
}

impl Fringe {
    /// Create new Fringe
    pub fn new(start: Node, size: usize, f_limit: f32) -> Self {
        let mut now = Vec::with_capacity(size);
        now.push(start);
        let later = Vec::with_capacity(size);

        Fringe {
            now,
            later,
            buckets: Default::default(),
            current: Bucket::from(f_limit),
        }
    }

    /// Push node to be processed in this iteration
    pub fn push_now(&mut self, node: Node) {
        self.now.push(node);
    }

    /// Push node to be processed in later iteration
    pub fn push_later(&mut self, (node, bucket): (Node, Bucket)) {
        self[bucket].push(node);
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

    /// Rotate later-buckets until a suitable is found, empty it to now and return the rotation-amount for f_limit fixing
    pub fn later_to_now(&mut self) -> Option<u8> {
        // println!("{}", self.buckets.iter().filter(|b| !b.is_empty()).count());

        // Rotate buckets until a suitable is found
        let mut i = 0;
        loop {
            if i == 8 {
                return None;
            }
            if self[self.current].is_empty() {
                self.current = self.current.add();
                i += 1;
            } else {
                break;
            }
        }

        // println!(
        //     "{:?}\t{:?}",
        //     self.current,
        //     self.buckets.iter().map(|b| b.len()).collect::<Vec<usize>>()
        // );
        let current = self.current;
        self.now.extend(self[self.current].to_owned().drain(..));
        self[current].clear();
        /* println!(
            "{:?}\t{:?}\t{:?}\n-",
            self.current,
            self.now,
            self.buckets.iter().map(|b| b.len()).collect::<Vec<usize>>()
        ); */

        Some(i)
    }
}

impl Index<Bucket> for Fringe {
    type Output = Vec<Node>;

    fn index(&self, index: Bucket) -> &Self::Output {
        &self.buckets[Into::<usize>::into(index)]
    }
}
impl IndexMut<Bucket> for Fringe {
    fn index_mut(&mut self, index: Bucket) -> &mut Self::Output {
        &mut self.buckets[Into::<usize>::into(index)]
    }
}
