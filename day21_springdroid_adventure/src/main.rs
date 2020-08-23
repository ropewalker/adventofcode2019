mod intcode;
mod read_input;

use intcode::*;
use read_input::*;

fn parse_input(input: &str) -> Vec<isize> {
    let mut parsed = Vec::new();

    for c in input.chars() {
        parsed.push(c as isize);
    }

    parsed
}

fn get_program() -> String {
    let mut input = String::new();
    input.push_str("NOT A J\n");
    input.push_str("NOT B T\n");
    input.push_str("OR T J\n");
    input.push_str("NOT C T\n");
    input.push_str("OR T J\n");
    input.push_str("AND D J\n");
    input.push_str("NOT E T\n");
    input.push_str("NOT T T\n");
    input.push_str("OR H T\n");
    input.push_str("AND T J\n");
    input.push_str("RUN\n");

    input
}

fn main() {
    let mut computer = Computer::init(&read_input(), parse_input(&get_program()));
    computer.compute();

    for o in computer.output {
        if let Some(c) = std::char::from_u32(o as u32) {
            print!("{}", c);
        } else {
            println!("{}", o);
        }
    }
}
