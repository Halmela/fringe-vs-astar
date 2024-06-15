use crate::xy_to_index;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

/// Simplifies lines to a boolean vector.
/// '.' and 'G' are traversable, others are not.
/// Some maps have Swamps, I have to think about that
fn simplify_map(map: Vec<String>) -> Vec<Vec<bool>> {
    map.iter()
        .map(|s| s.chars().map(|c| matches!(c, '.' | 'G')).collect())
        .collect()
}

/// Read a map from file
fn read_map(file_path: PathBuf) -> anyhow::Result<(usize, usize, Vec<String>)> {
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

/// Enum for different map types.
/// These can be used to distinguish what map is wanted in MapBuilder
pub enum MapType {
    GridMap,
    ArrayMap,
}

/// Read and build a map from a file and specify the type with [MapType]
pub fn map_builder(file_path: PathBuf) -> anyhow::Result<ArrayMap> {
    let (height, width, map) = read_map(file_path)?;
    let map = simplify_map(map);

    Ok(ArrayMap::new(height, width, map))
}

/* /// Representation of the underlying terrain map
pub trait Map: fmt::Display {
    /// Constructor
    /// Provide a cell of the grid if it exists
    fn get_cell(&self, x: usize, y: usize) -> Option<bool>;
    /// Get width
    fn get_height(&self) -> usize;
    /// Get height
    fn get_width(&self) -> usize;
}

/// Terrainmap stored as `grid[y][x]`
pub struct GridMap {
    height: usize,
    width: usize,
    grid: Vec<Vec<bool>>,
}

impl GridMap {
    /// Constructor
    pub fn new(height: usize, width: usize, map: Vec<Vec<bool>>) -> GridMap {
        GridMap {
            height,
            width,
            grid: map,
        }
    }
}

impl Map for GridMap {
    fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_width(&self) -> usize {
        self.width
    }
}

impl fmt::Display for GridMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if let Some(true) = self.get_cell(x, y) {
                    result.push('⬛');
                } else {
                    result.push('⬜');
                }
            }
            result.push('\n');
        }
        writeln!(f, "{}", result)
    }
} */

/// Terrainmap stored as a continuous `array[x + y*width]`
pub struct ArrayMap {
    height: usize,
    width: usize,
    array: Vec<bool>,
}

impl ArrayMap {
    /// Constructor
    pub fn new(height: usize, width: usize, mut map: Vec<Vec<bool>>) -> ArrayMap {
        ArrayMap {
            height,
            width,
            array: map.drain(..).flatten().collect(),
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.array[xy_to_index(x, y, self.width)])
        } else {
            None
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn array(&self) -> Vec<bool> {
        self.array.iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_simplifies() {
        let lines = vec![".T.".to_string(), "TGT".to_string()];
        let expected = vec![vec![true, false, true], vec![false, true, false]];
        let result = simplify_map(lines);
        assert_eq!(expected, result);
    }

    /* #[test]
    fn gridmap_gets_correct_cell() {
        let vec = simplify_map(vec![".T.".to_string(), "TGT".to_string()]);
        let map = GridMap::new(2, 3, vec);
        assert_eq!(Some(true), map.get_cell(0, 0));
    }

    #[test]
    fn gridmap_fails_out_of_bounds() {
        let vec = simplify_map(vec![".T.".to_string(), "TGT".to_string()]);
        let map = GridMap::new(2, 3, vec);
        assert_eq!(None, map.get_cell(3, 3));
    } */

    #[test]
    fn arraymap_gets_correct_cell() {
        let vec = simplify_map(vec![".T.".to_string(), "TGT".to_string()]);
        let map = ArrayMap::new(2, 3, vec);
        assert_eq!(Some(true), map.get_cell(0, 0));
    }

    #[test]
    fn arraymap_fails_out_of_bounds() {
        let vec = simplify_map(vec![".T.".to_string(), "TGT".to_string()]);
        let map = ArrayMap::new(2, 3, vec);
        assert_eq!(None, map.get_cell(3, 3));
    }
}
