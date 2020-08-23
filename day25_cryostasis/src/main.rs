use crate::intcode::{Computer, State};
use crate::read_input::*;
use std::collections::HashSet;
use std::io;

mod intcode;
mod read_input;

fn main() {
    let mut computer = Computer::init(&read_input());
    let mut result = computer.compute();

    println!("{}", computer.read_ascii_output());

    let commands = [
        "east",
        "take klein bottle",
        "east",
        "take semiconductor",
        "west",
        "north",
        "north",
        "north",
        "take dehydrated water",
        "south",
        "south",
        "south",
        "west",
        "north",
        "take sand",
        "north",
        "north",
        "take astrolabe",
        "south",
        "south",
        "west",
        "west",
        "take mutex",
        "east",
        "east",
        "south",
        "west",
        "north",
        "take shell",
        "south",
        "south",
        "west",
        "take ornament",
        "west",
        "south",
    ];

    for &command in commands.iter() {
        let mut command = String::from(command);
        command.push('\n');

        computer.provide_ascii_input(command);
        result = computer.compute();

        println!("{}", computer.read_ascii_output());
    }

    let items = vec![
        "mutex",
        "ornament",
        "astrolabe",
        "sand",
        "semiconductor",
        "dehydrated water",
        "shell",
        "klein bottle",
    ];

    let mut inv: HashSet<&str> = items.iter().cloned().collect();

    let alerts = [
        "Droids on this ship are lighter than the detected value!",
        "Droids on this ship are heavier than the detected value!",
    ];

    for i in (0..2_i32.pow(8)).rev() {
        for (k, item) in items.iter().enumerate().take(8_usize) {
            let j = i / 2_i32.pow(k as u32) % 2;

            if j == 1 && !inv.contains(item) {
                let mut instruction = String::from("take ");
                instruction.push_str(item);
                instruction.push('\n');

                computer.provide_ascii_input(instruction);
                result = computer.compute();
                println!("{}", computer.read_ascii_output());

                inv.insert(item);
            }

            if j == 0 && inv.contains(item) {
                let mut instruction = String::from("drop ");
                instruction.push_str(item);
                instruction.push('\n');

                computer.provide_ascii_input(instruction);
                result = computer.compute();
                println!("{}", computer.read_ascii_output());

                inv.remove(item);
            }
        }

        let instruction = String::from("south\n");

        computer.provide_ascii_input(instruction);
        result = computer.compute();

        let output = computer.read_ascii_output();
        println!("{}", output);

        if !output.contains(alerts[0]) && !output.contains(alerts[1]) {
            break;
        }
    }

    while result == State::AwaitingInput {
        let mut instruction = String::new();

        io::stdin()
            .read_line(&mut instruction)
            .expect("Failed to read line");

        computer.provide_ascii_input(instruction);
        result = computer.compute();
        println!("{}", computer.read_ascii_output());
    }
}
