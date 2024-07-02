use super::Bucket;
use super::Indexes;
use crate::Cost;
use crate::Node;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

/// Store now, later, and inactive buckets in 8 Vecs and access them with indexes.
///
/// Now-list is `Node`s found in current iteration.
/// Later-list is `Node`s found in previous iterations, but which have same integer part as current `f_limit`.
/// A bucket is a list that holds all found nodes that have the same integer part, such as 3.13 would go to `Bucket::Three`.
///
/// With octile distance heuristic, every found node has an upper bound estimate of +4,
/// This means that we can take an modulo 8 of an estimate and push it to an unique bucket.
/// Because at least the bucket before current one is empty, we can use it to store the Now-list.
///
/// I am not sure if this really provides any real speedups, but I don't want to redo old work, so this stays.
/// Maintaining an array of indexes for static vectors is a real pain,
/// but at least this does not allocate even once after it has been initialized.
#[derive(Debug, Clone)]
pub struct Buckets {
    pub(crate) buckets: Vec<Option<Node>>,
    pub(crate) current_bucket: Bucket,
    pub(crate) now_last: usize,
    pub(crate) later_head: usize,
    pub(crate) later_last: usize,
    pub(crate) indexes: Indexes,
    pub(crate) bucket_length: usize,
}

impl Buckets {
    /// Create Buckets and initialize them.
    /// Every bucket contains only `None`s, but Now has start-node.
    /// `min_size` should be the size of graph (maximum amount of nodes to be discovered).
    pub fn new(start: Node, f_limit: Cost, min_size: usize) -> Self {
        let mut length = min_size.next_power_of_two();
        length *= 2;
        let mut buckets: Vec<Option<Node>> = vec![None; length];

        let bucket_length = length / Bucket::SIZE;
        let indexes = Indexes::new(bucket_length);

        let current_bucket = Bucket::from(f_limit);
        let later_last = indexes[current_bucket];
        let later_head = later_last + 1;

        let now_last = indexes[current_bucket.sub()];
        buckets[now_last] = Some(start);

        Buckets {
            buckets,
            current_bucket,
            now_last,
            later_head,
            later_last,
            indexes,
            bucket_length,
        }
    }

    /// Push a node to Now-list
    pub(crate) fn push_now(&mut self, node: Node) {
        self.now_last += 1;
        self.buckets[self.now_last] = Some(node);
    }

    /// Pop a node from Now or try to get it from Later.
    ///
    /// `bool` part indicates where the `Node` came from.
    /// `true` if it came from Now-list, `bool` if it came from Later-list
    pub(crate) fn pop(&mut self) -> (Option<Node>, bool) {
        if let Some(node) = self.buckets[self.now_last] {
            self.buckets[self.now_last] = None;
            self.now_last = self.now_last.saturating_sub(1);
            (Some(node), true)
        } else {
            self.pop_later()
        }
    }

    /// Try pop a node from later. If later is "empty", try to refresh it.
    /// If the refresh didn't succeed, then no path can be found.
    pub(crate) fn pop_later(&mut self) -> (Option<Node>, bool) {
        if self.later_head > self.later_last {
            (None, self.refresh())
        } else if let Some(node) = self.buckets[self.later_head] {
            (Some(node), false)
        } else {
            (None, self.refresh())
        }
    }

    pub(crate) fn keep_current(&mut self) {
        self.later_head += 1;
    }

    fn current_index(&self) -> usize {
        self.indexes[self.current_bucket]
    }

    /// Remove a node from head of the Later list.
    /// This is done by swapping head and tail of Later, then swapping tail of Later with last node of current bucket.
    /// Then the Node is made a None and indexes are updated accordingly.
    pub(crate) fn remove_later_head(&mut self) {
        let current = self.current_index();
        self.buckets.swap(self.later_head, self.later_last);
        self.buckets.swap(self.later_last, current);
        self.buckets[current] = None;

        self.later_last = std::cmp::max(
            self.later_last.saturating_sub(1),
            self.bucket_start(self.current_bucket),
        );
        self.indexes.sub(self.current_bucket);
    }

    /// Push a Node to indicated bucket.
    pub(crate) fn push_bucket(&mut self, node: Node, bucket: Bucket) {
        if self.buckets[self.indexes[bucket]].is_some() {
            self.indexes.add(bucket);
        }

        self.buckets[self.indexes[bucket]] = Some(node);
    }

    /// Try to make Later a viable vector again. Returns if the operation succeeded.
    pub(crate) fn refresh(&mut self) -> bool {
        let later_ends_to_bucket_start = self.later_last == self.bucket_start(self.current_bucket);
        let later_head_is_end = self.later_last == self.later_head;
        let same_as_current_start = |index| index == self.indexes[self.current_bucket];

        if later_ends_to_bucket_start
            && (same_as_current_start(self.later_last)
                || same_as_current_start(self.later_last + 1) && later_head_is_end)
        {
            self.rotate()
        } else {
            self.refresh_index();
            true
        }
    }

    /// Rotate buckets until a viable one is found and refresh it.
    /// If all buckets are empty, then no path can be found and a `false` is returned.
    pub(crate) fn rotate(&mut self) -> bool {
        for _ in 0..8 {
            if self.buckets[self.indexes[self.current_bucket]].is_some() {
                self.refresh_index();
                return true;
            }
            self.current_bucket = self.current_bucket.add();
        }
        return false;
    }

    /// Bucket's default starting index
    fn bucket_start(&self, bucket: Bucket) -> usize {
        self.bucket_length * bucket as usize
    }

    /// Reset Now, Later and the current bucket.
    ///
    /// Now and current bucket will be empty, Later will contain same nodes as in the end of last iteration and all the other `Node`s in the current bucket.
    pub(crate) fn refresh_index(&mut self) {
        self.later_head = self.bucket_start(self.current_bucket);
        if self[self.current_bucket].is_some() {
            self.later_last = self.indexes[self.current_bucket];
        }

        self.indexes[self.current_bucket] = self.later_last + 1;
        if self.buckets[self.later_head].is_none() && self.buckets[self.later_last].is_some() {
            self.remove_later_head();
        }

        self.now_last = self.bucket_start(self.current_bucket.sub());
    }

    fn bucket(&self, bucket: Bucket) -> &[Option<Node>] {
        let start = self.bucket_length * Into::<usize>::into(bucket);
        &self.buckets[start..(start + self.bucket_length)]
    }

    pub fn all(&self) -> &[Option<Node>] {
        &self.buckets
    }
    pub fn now(&self) -> &[Option<Node>] {
        let i = self.current_bucket.sub() as usize * self.bucket_length;
        &self.buckets[i..=self.now_last]
    }
    pub fn later(&self) -> &[Option<Node>] {
        &self.buckets[self.later_head..=self.later_last]
    }
    pub fn next_later(&self) -> &[Option<Node>] {
        let i = self.later_last + 1;
        let j = self.current_bucket as usize * self.bucket_length + self.bucket_length;
        &self.buckets[i..j]
    }
}

impl Index<Bucket> for Buckets {
    type Output = Option<Node>;

    fn index(&self, index: Bucket) -> &Self::Output {
        &self.buckets[self.indexes[index]]
    }
}

impl IndexMut<Bucket> for Buckets {
    fn index_mut(&mut self, index: Bucket) -> &mut Self::Output {
        &mut self.buckets[self.indexes[index]]
    }
}

impl Display for Buckets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("{:?}\n", self.current_bucket));
        result.push_str(&format!(
            "now     {} {:?}\n  {:?}\n",
            self.now_last,
            self.buckets[self.now_last],
            self.bucket(self.current_bucket.sub())
                .iter()
                .map_while(|n| *n)
                .collect::<Vec<_>>()
        ));
        result.push_str(&format!(
            "l_first {} {:?}\n",
            self.later_head, self.buckets[self.later_head]
        ));
        result.push_str(&format!(
            "l_last  {} {:?}\n  {:?}\n",
            self.later_last,
            self.buckets[self.later_last],
            &self.buckets[self.later_head..=self.later_last]
        ));
        for (i, bucket) in self.buckets.chunks(self.bucket_length).enumerate() {
            result.push_str(&format!(
                "{:<5} {}\t{:?}\n",
                i * self.bucket_length,
                self.indexes[i.into()],
                bucket.iter().map_while(|n| *n).collect::<Vec<_>>()
            ));
        }
        result.push_str(&format!(
            "{:?}\n",
            self.now().iter().map_while(|n| *n).collect::<Vec<_>>()
        ));
        result.push_str(&format!(
            "{:?}\n",
            self.later().iter().map_while(|n| *n).collect::<Vec<_>>()
        ));
        result.push_str(&format!(
            "{:?}\n",
            self.next_later() // .iter()
                              // .map_while(|n| *n)
                              // .collect::<Vec<_>>()
        ));

        // println!("now:\n{:?}", self.now().collect::<Vec<_>>());
        // println!("later:\n{:?}", self.later().collect::<Vec<_>>());
        // println!("next:\n{:?}", self.next_later().collect::<Vec<_>>());

        write!(f, "{result}")
    }
}
