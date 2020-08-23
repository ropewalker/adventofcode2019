mod explorer;
mod maze;
mod read_input;

use explorer::Explorer;
use std::collections::HashSet;
use std::time::SystemTime;

fn main() {
    let earlier = SystemTime::now();

    let mut explorer = Explorer::init();
    //explorer.print_maze();
    println!(
        "{:?}",
        explorer.find_keys(explorer.starting_positions.clone(), HashSet::new())
    );

    println!("{:?}", SystemTime::now().duration_since(earlier));
}
