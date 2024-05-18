/// Simplifies lines to a boolean vector

fn simplify_map(lines: Vec<String>) -> Vec<Vec<bool>> {
    lines
        .iter()
        .map(|s| s.chars().map(|c| matches!(c, '.' | 'G')).collect())
        .collect()
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
}

pub trait Map {
    fn new(height: usize, width: usize, map: Vec<Vec<bool>>) -> impl Map;
    fn get_cell(&self, x: usize, y: usize) -> Option<bool>; // Is the node passable?
    fn get_height(&self) -> usize;
    fn get_width(&self) -> usize;
}

struct GridMap {
    height: usize,
    width: usize,
    grid: Vec<Vec<bool>>,
}

impl Map for GridMap {
    fn new(height: usize, width: usize, map: Vec<Vec<bool>>) -> GridMap {
        GridMap {
            height,
            width,
            grid: map,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.grid[x][y])
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

struct ArrayMap {
    height: usize,
    width: usize,
    array: Vec<bool>,
}

impl Map for ArrayMap {
    fn new(height: usize, width: usize, mut map: Vec<Vec<bool>>) -> ArrayMap {
        ArrayMap {
            height,
            width,
            array: map.drain(..).flatten().collect(),
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.array[x + y * self.width])
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
