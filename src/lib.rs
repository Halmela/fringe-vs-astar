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

/// Neat printing of a map and possibly start, goal and a path.
pub mod printable;

// pub mod node;

/// Octile distance of diagonal movement
pub const DIAGONAL_COST: f32 = std::f32::consts::SQRT_2;

/// Adapter for converting grid coordinate to array index.
/// Formula is `x + width * y`
/// ```
/// # use fringe_vs_astar::xy_to_index;
/// let (x, y, width) = (1,1,3);
/// assert_eq!(xy_to_index(x, y, width), 4);
/// ```
pub fn xy_to_index(x: usize, y: usize, width: usize) -> Node {
    (x + width * y).try_into().unwrap()
}

/// Adapter for converting array index to grid coordinate
/// ```
/// # use fringe_vs_astar::index_to_xy;
/// let (index,width) = (4,3);
/// assert_eq!(index_to_xy(index, width), (1,1));
/// ```
pub fn index_to_xy(i: Node, width: usize) -> (usize, usize) {
    ((i as usize) % width, (i as usize) / width)
}

/// Type alias for Node. This is calculated with [`xy_to_index`].
pub type Node = u32;
