use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const INPUT_PATH: &str = "input.txt";

fn read_input() -> String {
    let path = Path::new(INPUT_PATH);
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

fn get_parameter_value(computer: &mut Vec<i32>, parameter: i32, mode: i32) -> i32 {
    match mode {
        0 => computer[parameter as usize],
        1 => parameter,
        x => panic!("{} is a wrong parameter mode!", x),
    }
}

fn add(computer: &mut Vec<i32>, pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);
    let parameter3 = computer[pointer + 3] as usize;

    computer[parameter3] = parameter1 + parameter2;

    pointer + 4
}

fn multiply(computer: &mut Vec<i32>, pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);
    let parameter3 = computer[pointer + 3] as usize;

    computer[parameter3] = parameter1 * parameter2;

    pointer + 4
}

fn get_input(computer: &mut Vec<i32>, pointer: usize, input: i32) -> usize {
    let parameter = computer[pointer + 1] as usize;
    computer[parameter] = input;

    pointer + 2
}

fn calculate_output(
    computer: &mut Vec<i32>,
    pointer: usize,
    modes: i32,
    output: &mut i32,
) -> usize {
    *output = get_parameter_value(computer, computer[pointer + 1], modes % 10);

    pointer + 2
}

fn jump_if_true(computer: &mut Vec<i32>, pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);

    if parameter1 != 0 {
        parameter2 as usize
    } else {
        pointer + 3
    }
}

fn jump_if_false(computer: &mut Vec<i32>, pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);

    if parameter1 == 0 {
        parameter2 as usize
    } else {
        pointer + 3
    }
}

fn less_than(computer: &mut Vec<i32>, pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);
    let parameter3 = computer[pointer + 3] as usize;

    computer[parameter3] = if parameter1 < parameter2 { 1 } else { 0 };

    pointer + 4
}

fn equals(computer: &mut Vec<i32>, pointer: usize, modes: i32) -> usize {
    let parameter1 = get_parameter_value(computer, computer[pointer + 1], modes % 10);
    let parameter2 = get_parameter_value(computer, computer[pointer + 2], (modes / 10) % 10);
    let parameter3 = computer[pointer + 3] as usize;

    computer[parameter3] = if parameter1 == parameter2 { 1 } else { 0 };

    pointer + 4
}

fn compute(computer: &mut Vec<i32>, pointer: &mut usize, inputs: Vec<i32>) -> Result<i32, i32> {
    let mut inputs_iter = inputs.iter();
    let mut output = 0;

    let result: Result<i32, i32> = Err(inputs[inputs.len() - 1]);

    loop {
        let instruction = computer[*pointer];

        let opcode = instruction % 100;
        let modes = instruction / 100;

        match opcode {
            1 => *pointer = add(computer, *pointer, modes),
            2 => *pointer = multiply(computer, *pointer, modes),
            3 => *pointer = get_input(computer, *pointer, *inputs_iter.next().unwrap()),
            4 => {
                *pointer = calculate_output(computer, *pointer, modes, &mut output);
                return Ok(output);
            }
            5 => *pointer = jump_if_true(computer, *pointer, modes),
            6 => *pointer = jump_if_false(computer, *pointer, modes),
            7 => *pointer = less_than(computer, *pointer, modes),
            8 => *pointer = equals(computer, *pointer, modes),
            99 => return result,
            _ => panic!("unknown operand!"),
        }
    }
}

fn generate_phases_combinations(
    k: usize,
    permutation: &mut [usize; 5],
    phases_combinations: &mut HashSet<[usize; 5]>,
) {
    if k == 1 {
        phases_combinations.insert(permutation.clone());
    } else {
        generate_phases_combinations(k - 1, permutation, phases_combinations);

        for i in 0..k - 1 {
            if k % 2 == 0 {
                permutation[i] += permutation[k - 1];
                permutation[k - 1] = permutation[i] - permutation[k - 1];
                permutation[i] -= permutation[k - 1];
            } else {
                permutation[0] += permutation[k - 1];
                permutation[k - 1] = permutation[0] - permutation[k - 1];
                permutation[0] -= permutation[k - 1];
            }

            generate_phases_combinations(k - 1, permutation, phases_combinations);
        }
    }
}

fn main() {
    let instructions: Vec<i32> = read_input()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut phases_combinations: HashSet<[usize; 5]> = HashSet::new();
    let mut results: HashSet<i32> = HashSet::new();

    generate_phases_combinations(5, &mut [5, 6, 7, 8, 9], &mut phases_combinations);
    //phases_combinations.insert([9, 8, 7, 6, 5]);

    for phases in phases_combinations {
        let mut computers = [
            instructions.clone(),
            instructions.clone(),
            instructions.clone(),
            instructions.clone(),
            instructions.clone(),
        ];

        let mut result: Result<i32, i32> = Ok(0);
        let mut pointers = [0; 5];

        let mut i = 0;
        let mut k = 0;

        while result.is_ok() {
            let mut inputs: Vec<i32> = Vec::new();

            dbg!(i);

            if i < phases.len() {
                inputs.push(phases[i] as i32);
            }

            if let Ok(res) = result {
                inputs.push(res);
            };

            let k = i % phases.len();

            dbg!(k);
            dbg!(&inputs);
            dbg!(&pointers[k]);
            dbg!(&computers[k]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","));

            result = compute(&mut computers[k], &mut pointers[k], inputs);

            dbg!(&pointers[k]);
            dbg!(&computers[k]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","));
            dbg!(result);

            i += 1;
        }

        if let Err(res) = result {
            results.insert(res);
        }
    }

    println!("{}", results.iter().max().unwrap());
}
