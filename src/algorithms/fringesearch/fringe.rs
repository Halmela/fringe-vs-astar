use super::Bucket;
use crate::Node;
use std::ops::{Index, IndexMut};

/// Datastructure used for storing nodes in Fringe search.
/// Does not hold any data on existence of a node in fringe
/// Buckets can be indexed with `Bucket` (`self[bucket]`).
#[derive(Debug)]
pub struct Fringe {
    pub now: Vec<Node>,
    pub buckets: [Vec<Node>; 8],
    pub current: Bucket,
}

impl Fringe {
    /// Create new Fringe
    pub fn new(start: Node, size: usize, f_limit: f32) -> Self {
        let mut now = Vec::with_capacity(size);
        now.push(start);

        Fringe {
            now,
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
        self.now.pop()
    }

    /// Rotate later-buckets until a suitable is found, empty it to now and return the rotation-amount for f_limit fixing
    pub fn later_to_now(&mut self) -> Option<u8> {
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

        let current = self.current;
        self.now.append(&mut self[self.current].to_owned());
        self[current].clear();

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
