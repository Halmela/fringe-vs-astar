use fringe_vs_astar::structures::graph::GraphType;
use fringe_vs_astar::structures::map::MapType;

use fringe_vs_astar::context::Context;

fn main() -> anyhow::Result<()> {
    let mut context = Context::new(
        "benchmarks/Berlin_1_256.map",
        MapType::GridMap,
        GraphType::AdjacencyGridGraph,
    );
    context.read_problem_from_file("benchmarks/Berlin_1_256.map.scen", 910)?;
    context.solve();

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
