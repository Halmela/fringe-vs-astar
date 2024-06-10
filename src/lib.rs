//! A* and Fringe Search comparison program

/// Pathfinder algorithms
pub mod algorithms;

/// Data structures used by algorithms
pub mod structures;

/// Command Line Interface
pub mod cli;

/// Holder of all relevant information regarding problem solving
pub mod context;

/// Instance of a problem
pub mod problem;

pub mod printable;

pub const DIAGONAL_COST: f64 = std::f64::consts::SQRT_2;

/// Adapter for converting grid coordinate to array index
/// ```
/// # use fringe_vs_astar::xy_to_index;
/// let (x, y, width) = (1,1,3);
/// assert_eq!(xy_to_index(x, y, width), 4);
/// ```
pub fn xy_to_index(x: usize, y: usize, width: usize) -> usize {
    x + width * y
}

/// Adapter for converting array index to grid coordinate
/// ```
/// # use fringe_vs_astar::index_to_xy;
/// let (index,width) = (4,3);
/// assert_eq!(index_to_xy(index, width), (1,1));
/// ```
pub fn index_to_xy(i: usize, width: usize) -> (usize, usize) {
    (i % width, i / width)
}
