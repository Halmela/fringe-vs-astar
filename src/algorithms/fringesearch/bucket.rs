/// Enum for indexing to a correct bucket.
/// Main idea is that a float is converted to an integer and then taken a modulo 8 is taken from that.
/// This is an enum to clarify the finite values it can hold.
///
///```
/// # use fringe_vs_astar::algorithms::fringesearch::bucket::Bucket;
/// let mut bucket = Bucket::from(14.4 as f32);
/// assert_eq!(Bucket::Six, bucket);
///
/// bucket = bucket.add(); // Bucket::Seven
/// bucket = bucket.add(); // Bucket::Zero
/// assert_eq!(0 as usize, bucket.into());
///
///```

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
    None,
}

impl Bucket {
    /// Add 1 to the value, wrap to Zero if it was Seven (just like modulo works)
    #[must_use]
    pub fn add(&self) -> Bucket {
        match self {
            Bucket::Zero => Bucket::One,
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::Three,
            Bucket::Three => Bucket::Four,
            Bucket::Four => Bucket::Five,
            Bucket::Five => Bucket::Six,
            Bucket::Six => Bucket::Seven,
            Bucket::Seven => Bucket::Zero,
            Bucket::None => Bucket::None,
        }
    }

    pub fn sub(&self) -> Bucket {
        match self {
            Bucket::Zero => Bucket::Seven,
            Bucket::One => Bucket::Zero,
            Bucket::Two => Bucket::One,
            Bucket::Three => Bucket::Two,
            Bucket::Four => Bucket::Three,
            Bucket::Five => Bucket::Four,
            Bucket::Six => Bucket::Five,
            Bucket::Seven => Bucket::Six,
            Bucket::None => Bucket::None,
        }
    }
    pub const SIZE: usize = 8;
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
            7 => Bucket::Seven,
            _ => Bucket::None,
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
            Bucket::None => usize::MAX, // We want this to fail
        }
    }
}
impl From<usize> for Bucket {
    fn from(value: usize) -> Self {
        match value % 8 {
            0 => Bucket::Zero,
            1 => Bucket::One,
            2 => Bucket::Two,
            3 => Bucket::Three,
            4 => Bucket::Four,
            5 => Bucket::Five,
            6 => Bucket::Six,
            7 => Bucket::Seven,
            _ => Bucket::None,
        }
    }
}
