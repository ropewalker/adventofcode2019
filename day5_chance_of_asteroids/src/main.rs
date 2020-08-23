use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const INPUT: i32 = 5;

fn read_input() -> String {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why.description())
    }

    s
}

fn get_parameter_value(computer: &[i32], parameter: i32, mode: i32) -> i32 {
    match mode {
        0 => computer[parameter as usize],
        1 => parameter,
        x => panic!("{} is a wrong parameter mode!", x),
    }
}

fn add(computer: &mut [i32], pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);
    let parameter3 = computer[pointer + 3] as usize;

    computer[parameter3] = parameter1 + parameter2;

    pointer + 4
}

fn multiply(computer: &mut [i32], pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);
    let parameter3 = computer[pointer + 3] as usize;

    computer[parameter3] = parameter1 * parameter2;

    pointer + 4
}

fn input(computer: &mut [i32], pointer: usize) -> usize {
    let parameter = computer[pointer + 1] as usize;
    computer[parameter] = INPUT;

    pointer + 2
}

fn output(computer: &mut [i32], pointer: usize, modes: i32) -> usize {
    let parameter = get_parameter_value(computer, computer[pointer + 1], modes % 10);

    println!("result: {}", parameter);

    pointer + 2
}

fn jump_if_true(computer: &mut [i32], pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);

    if parameter1 != 0 {
        parameter2 as usize
    } else {
        pointer + 3
    }
}

fn jump_if_false(computer: &mut [i32], pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);

    if parameter1 == 0 {
        parameter2 as usize
    } else {
        pointer + 3
    }
}

fn less_than(computer: &mut [i32], pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);
    let parameter3 = computer[pointer + 3] as usize;

    computer[parameter3] = if parameter1 < parameter2 { 1 } else { 0 };

    pointer + 4
}

fn equals(computer: &mut [i32], pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);
    let parameter3 = computer[pointer + 3] as usize;

    computer[parameter3] = if parameter1 == parameter2 { 1 } else { 0 };

    pointer + 4
}

fn main() {
    let mut computer: Vec<i32> = read_input()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut i: usize = 0;

    loop {
        let instruction = computer[i];

        let opcode = instruction % 100;
        let modes = instruction / 100;

        match opcode {
            1 => i = add(&mut computer, i, modes),
            2 => i = multiply(&mut computer, i, modes),
            3 => i = input(&mut computer, i),
            4 => i = output(&mut computer, i, modes),
            5 => i = jump_if_true(&mut computer, i, modes),
            6 => i = jump_if_false(&mut computer, i, modes),
            7 => i = less_than(&mut computer, i, modes),
            8 => i = equals(&mut computer, i, modes),
            99 => break,
            _ => panic!("unknown operand!"),
        }
    }
}
