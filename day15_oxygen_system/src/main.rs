mod intcode;
mod maze;
mod read_input;
mod robot;

use read_input::*;
use robot::Robot;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut robot = Robot::init(&read_input(INPUT_PATH));
    let dist = robot.oxygen_fill();

    println!("{}", dist);
}
