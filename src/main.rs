use fringe_vs_astar::graph::*;
use fringe_vs_astar::map::*;
use fringe_vs_astar::pathfinder::*;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let map = "benchmarks/adaptive-depth-1.map";
    let grid = map_builder(map, MapType::GridMap)?;
    let grid_lol = map_builder(map, MapType::GridMap)?;
    println!("{}", grid);

    //let graph = AdjacencyMapGraph::new(grid);
    let graph = AdjacencyGridGraph::new(grid);

    println!("expected: 121.811182141304");
    let path: HashSet<(usize, usize)> = a_star_simple(79, 18, 38, 78, graph)
        .unwrap()
        .drain(..)
        .collect();

    let mut result = String::new();
    for y in 0..grid_lol.get_height() {
        for x in 0..grid_lol.get_width() {
            if path.contains(&(x, y)) {
                result.push('🟩');
            } else if let Some(true) = grid_lol.get_cell(x, y) {
                result.push('⬛');
            } else {
                result.push('⬜');
            }
        }
        result.push('\n');
    }
    println!("{}", result);

    Ok(())
}

/*
.map -> Map -> Graph -> Pathfinder -> Presenter
.scen -> [Scenario] -->^
*/
