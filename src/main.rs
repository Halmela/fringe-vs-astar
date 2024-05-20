use fringe_vs_astar::map::{map_builder, MapType};

fn main() -> anyhow::Result<()> {
    let map = "benchmarks/dia10x10.map";
    let grid = map_builder(map, MapType::GridMap)?;
    println!("{}", grid);
    let array = map_builder(map, MapType::ArrayMap)?;
    println!("{}", array);

    Ok(())
}

/*
.map -> Map -> Graph -> Pathfinder -> Presenter
.scen -> [Scenario] -->^
*/
