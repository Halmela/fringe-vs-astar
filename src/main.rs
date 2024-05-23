use fringe_vs_astar::graph::*;
use fringe_vs_astar::map::*;
use fringe_vs_astar::pathfinder::*;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let map = "benchmarks/scene_sp_sax_04.map";
    let grid = map_builder(map, MapType::GridMap)?;
    let grid_lol = map_builder(map, MapType::GridMap)?;
    //println!("{}", grid);

    /* let graph = AdjacencyGridGraph::new(grid);

    println!("expected: 7712.59278044");
    let astar = AStar::new(3411, 4166, 3381, 533, graph);
    */

    /* let map = "benchmarks/Berlin_1_256.map";
    let grid = map_builder(map, MapType::GridMap)?;
    let grid_lol = map_builder(map, MapType::GridMap)?;
    println!("{}", grid);

    let graph = AdjacencyGridGraph::new(grid);

    println!("expected: 361.98989868");
    let astar = AStar::new(16, 3, 236, 223, graph); */

    let map = "benchmarks/Berlin_0_1024.map";
    let grid = map_builder(map, MapType::GridMap)?;
    let grid_lol = map_builder(map, MapType::GridMap)?;
    println!("{}", grid);

    let graph = AdjacencyGridGraph::new(grid);

    println!("expected: 1539.80230712");
    let astar = AStar::new(19, 3, 1005, 1002, graph);

    let path: HashSet<(usize, usize)> = astar.solve().unwrap().drain(..).collect();

    /* let mut result = String::new();
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
    println!("{}", result); */

    Ok(())
}

/*
.map -> Map -> Graph -> Pathfinder -> Presenter
.scen -> [Scenario] -->^
*/
