use std::ops::IndexMut;

use std::ops::Index;

use super::super::Bucket;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Indexes([usize; Bucket::SIZE]);

impl Indexes {
    pub fn new(bucket_length: usize) -> Self {
        Indexes([
            0 * bucket_length,
            1 * bucket_length,
            2 * bucket_length,
            3 * bucket_length,
            4 * bucket_length,
            5 * bucket_length,
            6 * bucket_length,
            7 * bucket_length,
        ])
    }

    pub fn add(&mut self, bucket: Bucket) {
        self[bucket] += 1;
    }
    pub fn sub(&mut self, bucket: Bucket) {
        self[bucket] = self[bucket].saturating_sub(1);
    }
}

impl Index<Bucket> for Indexes {
    type Output = usize;

    fn index(&self, index: Bucket) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Bucket> for Indexes {
    fn index_mut(&mut self, index: Bucket) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
