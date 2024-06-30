use super::super::Bucket;
use super::indexes::Indexes;
use crate::Cost;
use crate::Node;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub(crate) struct Buckets {
    pub(crate) buckets: Vec<Option<Node>>,
    pub(crate) current_bucket: Bucket,
    pub(crate) now_last: usize,
    pub(crate) later_head: usize,
    pub(crate) later_last: usize,
    pub(crate) indexes: Indexes,
    pub(crate) bucket_length: usize,
}

impl Buckets {
    pub fn new(start: Node, f_limit: Cost, min_size: usize) -> Self {
        let mut length = min_size.next_power_of_two();
        length *= 2;
        // dbg!(min_size, length);
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

    pub(crate) fn push_now(&mut self, node: Node) {
        self.now_last += 1;
        self.buckets[self.now_last] = Some(node);
    }

    pub(crate) fn pop(&mut self) -> (Option<Node>, bool) {
        if let Some(node) = self.buckets[self.now_last] {
            self.buckets[self.now_last] = None;
            self.now_last = self.now_last.saturating_sub(1);
            (Some(node), true)
        } else {
            self.pop_later()
        }
    }

    pub(crate) fn pop_later(&mut self) -> (Option<Node>, bool) {
        /*
        current: b[later_first..=later_last]
        true-later: b[later_last+1..=I[b]]
        */
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

    pub(crate) fn remove_current(&mut self) {
        // println!("remove {:?}", self.buckets[self.later_first]);
        self.buckets.swap(self.later_head, self.later_last);
        // println!("{self}");
        self.buckets
            .swap(self.later_last, self.indexes[self.current_bucket]);
        // println!("{self}\n");
        self.buckets[self.indexes[self.current_bucket]] = None;

        self.later_last = std::cmp::max(
            self.later_last.saturating_sub(1),
            self.bucket_start(self.current_bucket),
        );
        self.indexes.sub(self.current_bucket);
        // if self.later_last > self.later_head {
        //     self.later_last -= 1;
        // }
        // if self.later_last < self.indexes[self.current_bucket] {
        //     self.indexes.sub(self.current_bucket);
        // }
        // println!("{self}\n");
    }

    pub(crate) fn push_bucket(&mut self, node: Node, bucket: Bucket) {
        // println!("{node} {:?}", bucket);
        if let None = self.buckets[self.indexes[bucket]] {
            self.buckets[self.indexes[bucket]] = Some(node);
        } else {
            self.indexes.add(bucket);
            self.buckets[self.indexes[bucket]] = Some(node);
        }
    }

    pub(crate) fn refresh(&mut self) -> bool {
        if self.later_last == self.indexes[self.current_bucket]
            && self.later_last == self.bucket_start(self.current_bucket)
        {
            self.rotate()
        } else {
            self.refresh_index();
            true
        }
    }

    pub(crate) fn rotate(&mut self) -> bool {
        // println!("start rotate");
        for _ in 0..8 {
            if self.buckets[self.indexes[self.current_bucket]].is_some() {
                self.refresh_index();
                return true;
            }
            self.current_bucket = self.current_bucket.add();
        }
        return false;
    }

    fn bucket_start(&self, bucket: Bucket) -> usize {
        self.bucket_length * bucket as usize
    }

    pub(crate) fn refresh_index(&mut self) {
        self.later_head = self.bucket_start(self.current_bucket);
        if self[self.current_bucket].is_some() {
            self.later_last = self.indexes[self.current_bucket];
        }

        self.indexes[self.current_bucket] = self.later_last + 1;
        /* else if self.later_last > self.bucket_start(self.current_bucket) {
            self.later_last = self.indexes[self.current_bucket] - 1;
        } */

        self.now_last = self.bucket_start(self.current_bucket.sub());
    }

    fn bucket(&self, bucket: Bucket) -> &[Option<Node>] {
        let start = self.bucket_length * Into::<usize>::into(bucket);
        &self.buckets[start..(start + self.bucket_length)]
    }

    // fn now(&self) -> impl Iterator<Item = &Node> {

    //     todo!()
    // }
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
