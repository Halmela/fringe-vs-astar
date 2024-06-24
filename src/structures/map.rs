use crate::xy_to_index;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

/// Simplifies lines to a boolean vector.
/// '.' and 'G' are traversable, others are not.
fn simplify_map(map: Vec<String>) -> Vec<bool> {
    map.iter()
        .flat_map(|s| s.chars().map(|c| matches!(c, '.' | 'G')))
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

/// Terrainmap stored as a continuous `array[x + y*width]`
pub struct Map {
    height: usize,
    width: usize,
    map: Vec<bool>,
}

impl Map {
    /// Constructor
    pub fn new(file_path: PathBuf) -> Map {
        let (height, width, map) = read_map(file_path).expect("Malformed map file");
        let map = simplify_map(map);
        Map { height, width, map }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.map[xy_to_index(x, y, self.width) as usize])
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
        self.map.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_simplifies() {
        let lines = vec![".T.".to_string(), "TGT".to_string()];
        let expected = vec![true, false, true, false, true, false];
        let result = simplify_map(lines);
        assert_eq!(expected, result);
    }

    #[test]
    fn map_gets_correct_cell() {
        let map = Map::new(PathBuf::from("maps/3x3.map"));
        assert_eq!(Some(true), map.get_cell(0, 0));
    }

    #[test]
    fn map_gets_correct_wall() {
        let map = Map::new(PathBuf::from("maps/3x3.map"));
        assert_eq!(Some(false), map.get_cell(1, 1));
    }

    #[test]
    fn map_fails_out_of_bounds() {
        let map = Map::new(PathBuf::from("maps/3x3.map"));
        assert_eq!(None, map.get_cell(3, 3));
    }
}
