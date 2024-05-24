use fringe_vs_astar::structures::graph::GraphType;
use fringe_vs_astar::structures::map::MapType;

use fringe_vs_astar::context::Context;

fn main() -> anyhow::Result<()> {
    let mut context = Context::new(
        "benchmarks/adaptive-depth-1.map",
        MapType::GridMap,
        GraphType::AdjacencyGridGraph,
    );
    /* context.read_problem_from_file("benchmarks/adaptive-depth-1.map.scenario", 70)?;
    context.print_problem();
    context.solve(true); */
    context.run_full_file("benchmarks/adaptive-depth-1.map.scenario", false)?;

    Ok(())
}
