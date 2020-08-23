mod intcode;
mod maze;
mod read_input;
mod robot;

use read_input::*;
use robot::Robot;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut robot = Robot::init(&read_input(INPUT_PATH));
    robot.print_maze();

//"R,12,L,8,L,4,L,4,L,8,R,6,L,6,R,12,L,8,L,4,L,4,L,8,R,6,L,6,L,8,L,4,R,12,L,6,L,4,R,12,L,8,L,4,L,4,L,8,L,4,R,12,L,6,L,4,R,12,L,8,L,4,L,4,L,8,L,4,R,12,L,6,L,4,L,8,R,6,L,6"
    //        dbg!(robot.find_path());
    //dbg!(robot.computer.get_output());
}
