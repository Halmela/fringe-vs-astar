use std::ops::IndexMut;

use std::ops::Index;

use super::super::Bucket;

#[derive(Debug, Clone, Copy)]
pub struct Indexes([usize; Bucket::SIZE]);

impl Indexes {
    /// Array of indexes for accessing different buckets in Fringe
    /// ```
    /// # use fringe_vs_astar::algorithms::fringesearch::fringe::indexes::Indexes;
    /// # use fringe_vs_astar::algorithms::fringesearch::bucket::Bucket;
    /// let indexes = Indexes::new(4);
    /// assert_eq!(0, indexes[Bucket::Zero]);
    /// assert_eq!(4, indexes[Bucket::One]);
    /// assert_eq!(8, indexes[Bucket::Two]);
    /// assert_eq!(12, indexes[Bucket::Three]);
    /// assert_eq!(16, indexes[Bucket::Four]);
    /// assert_eq!(20, indexes[Bucket::Five]);
    /// assert_eq!(24, indexes[Bucket::Six]);
    /// assert_eq!(28, indexes[Bucket::Seven]);
    /// ```
    pub fn new(bucket_length: usize) -> Self {
        Indexes([
            0,
            bucket_length,
            2 * bucket_length,
            3 * bucket_length,
            4 * bucket_length,
            5 * bucket_length,
            6 * bucket_length,
            7 * bucket_length,
        ])
    }

    /// +1 to a bucket's index
    /// ```
    /// # use fringe_vs_astar::algorithms::fringesearch::fringe::indexes::Indexes;
    /// # use fringe_vs_astar::algorithms::fringesearch::bucket::Bucket;
    /// let mut indexes = Indexes::new(4);
    /// indexes.add(Bucket::Zero);
    /// assert_eq!(1, indexes[Bucket::Zero]);
    pub fn add(&mut self, bucket: Bucket) {
        self[bucket] += 1;
    }

    /// -1 to a bucket's index. Will stop at 0
    /// ```
    /// # use fringe_vs_astar::algorithms::fringesearch::fringe::indexes::Indexes;
    /// # use fringe_vs_astar::algorithms::fringesearch::bucket::Bucket;
    /// let mut indexes = Indexes::new(4);
    /// indexes.add(Bucket::Zero);
    /// indexes.sub(Bucket::Zero);
    /// assert_eq!(0, indexes[Bucket::Zero]);
    /// indexes.sub(Bucket::Zero);
    /// assert_eq!(0, indexes[Bucket::Zero]);
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
