use crate::xy_to_index;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

/// Terrainmap stored as a continuous `array[x + y*width]`
pub struct Map {
    height: usize,
    width: usize,
    map: Vec<bool>,
}

impl Map {
    /// Initialize from a file.
    /// # Panics
    /// Panics if the supplied map does not follow the formatting standards
    #[must_use]
    pub fn new(file_path: PathBuf) -> Map {
        let (height, width, map) = read(file_path).expect("Malformed map file");
        let map = simplify(&map);
        Map { height, width, map }
    }

    /// Get the value of a cell.
    ///```
    /// # use fringe_vs_astar::structures::Map;    
    /// # use std::path::PathBuf;
    /// // maps/3x3.map looks like:
    /// // ...
    /// // .@.
    /// // ...
    /// let map = Map::new(PathBuf::from("maps/3x3.map"));
    /// assert_eq!(Some(true), map.get_cell(0, 0));
    /// assert_eq!(Some(false), map.get_cell(1, 1));
    /// assert_eq!(None, map.get_cell(3, 3));
    ///```
    #[must_use]
    pub fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.map[xy_to_index(x, y, self.width) as usize])
        } else {
            None
        }
    }

    /// Get height
    #[must_use]
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Get width
    #[must_use]
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Access to underlying array
    #[must_use]
    pub fn array(&self) -> Vec<bool> {
        self.map.clone()
    }
}

/// Simplifies lines to a boolean vector.
/// `.` and `G` are traversable, others are not.
///```
/// # use fringe_vs_astar::structures::map::simplify;
/// let lines = vec![".T.".to_string(), "TGT".to_string()];
/// let expected = vec![true, false, true, false, true, false];
/// let result = simplify(&lines);
/// assert_eq!(expected, result);
///```
#[must_use]
pub fn simplify(map: &[String]) -> Vec<bool> {
    map.iter()
        .flat_map(|s| s.chars().map(|c| matches!(c, '.' | 'G')))
        .collect()
}

/// Read a map from file
fn read(file_path: PathBuf) -> anyhow::Result<(usize, usize, Vec<String>)> {
    let f = File::open(file_path)?;
    let mut contents = BufReader::new(f).lines();

    contents.next(); // octiles

    let height = contents
        .next()
        .unwrap()?
        .strip_prefix("height ")
        .unwrap()
        .parse()?;
    let width = contents
        .next()
        .unwrap()?
        .strip_prefix("width ")
        .unwrap()
        .parse()?;
    contents.next();

    let map = contents.map(|s| s.unwrap()).collect();

    Ok((height, width, map))
}
