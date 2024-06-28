use super::Bucket;
use crate::Node;
use std::ops::{Index, IndexMut};

use std::collections::VecDeque;

/// Datastructure used for storing nodes in Fringe search.
/// Does not hold any data on existence of a node in fringe
/// Buckets can be indexed with `Bucket` (`self[bucket]`).
#[derive(Debug)]
pub struct Fringe {
    pub now: VecDeque<Node>,
    pub buckets: [Vec<Node>; 8],
    pub current: Bucket,
}

impl Fringe {
    /// Create new Fringe
    #[must_use]
    pub fn new(start: Node, size: usize, f_limit: f32) -> Self {
        let mut now = VecDeque::with_capacity(1024);
        now.push_front(start);
        now.push_back(Node::MAX);

        Fringe {
            now,
            buckets: Default::default(),
            current: Bucket::from(f_limit),
        }
    }

    /// Fast access to current bucket
    #[must_use]
    pub fn current(&self) -> &Vec<Node> {
        &self[self.current]
    }

    pub fn now(&self) -> impl Iterator<Item = &Node> {
        self.now.iter().take_while(|n| n != &&Node::MAX)
    }
    pub fn later(&self) -> impl Iterator<Item = &Node> {
        self.now.iter().skip_while(|n| n != &&Node::MAX).skip(1)
    }

    pub fn push(&mut self, (node, bucket): (Node, Bucket)) {
        if bucket == self.current {
            self.push_now(node);
        } else {
            self[bucket].push(node);
            // self.push_later((node, bucket));
        }
    }

    /// Push node to be processed in this iteration
    #[inline(always)]
    pub fn push_now(&mut self, node: Node) {
        self.now.push_front(node);
    }

    /// Push node to be processed in later iteration
    #[inline(always)]
    pub fn push_later(&mut self, (node, bucket): (Node, Bucket)) {
        self.now.push_back(node);
    }

    /// Try to give a node from now list
    #[inline(always)]
    pub fn pop_now(&mut self) -> Option<Node> {
        self.now.pop_front().filter(|n| n != &Node::MAX)
    }

    /// Rotate later-buckets until a suitable is found, empty it to now and return the amount of rotations for `f_limit` fixing.
    /// Returns `None` if all buckets are empty.
    pub fn later_to_now(&mut self) -> Option<u8> {
        if self.now.is_empty() {
            // Rotate buckets until a suitable is found
            let mut i = 0;
            loop {
                if i == 8 {
                    return None;
                }
                if self.current().is_empty() {
                    self.current = self.current.add();
                    i += 1;
                } else {
                    break;
                }
            }
            let current = self.current;
            self.now = VecDeque::from(self[current].clone());
            self[current].clear();
            self.now.push_back(Node::MAX);
            Some(i)
        } else {
            self.now.push_back(Node::MAX);
            Some(0)
        }

        // self.now.sort_unstable();
        // self.now.dedup();
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
