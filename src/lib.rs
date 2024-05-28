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

pub const DIAGONAL_COST: f64 = 1.414213562373095;
