use crate::eris::Eris;
use read_input::*;

mod eris;
mod read_input;

fn main() {
    let mut eris = Eris::init(&read_input());

    for _ in 1..=200 {
        eris.pass_a_minute();
    }

    println!("{}", eris.count_bugs());
//    println!("{}", eris);
}
