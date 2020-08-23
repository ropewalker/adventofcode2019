mod explorer;
mod maze;
mod read_input;

use explorer::Explorer;
use std::collections::HashSet;

fn main() {
    let mut explorer = Explorer::init();
    explorer.print_maze();
    println!("{:?}", &explorer.portals);
    println!("{:?}", &explorer.shortest_path());
}
