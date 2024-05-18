use fringe_vs_astar::map::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let map = read_map("benchmarks/dia10x10.map").unwrap();
    println!("{:?}", map);
    let grid = ArrayMap::new(map.0, map.1, simplify_map(map.2));
    println!("{grid}");
}

fn read_map(file_path: &str) -> anyhow::Result<(usize, usize, Vec<String>)> {
    let f = File::open(file_path)?;
    let mut contents = BufReader::new(f).lines();

    contents.next();

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

/*
.map -> Map -> Graph -> Pathfinder -> Presenter
.scen -> [Scenario] -->^
*/
