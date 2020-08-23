use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const INPUT_PATH: &str = "input.txt";

fn read_input(path: &str) -> String {
    let path = Path::new(path);
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

struct Computer {
    program: HashMap<isize, isize>,
    pointer: isize,
    relative_base: isize,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl Computer {
    fn init(program: &str, input: Vec<isize>) -> Computer {
        let instructions: HashMap<isize, isize> = program
            .trim()
            .split(',')
            .enumerate()
            .map(|(x, y)| (x as isize, y.parse::<isize>().unwrap()))
            .collect();

        Computer {
            program: instructions,
            pointer: 0,
            relative_base: 0,
            input,
            output: Vec::new(),
        }
    }

    fn get_parameter_value(&mut self, parameter: isize, mode: isize) -> isize {
        match mode {
            0 => *self.program.entry(parameter).or_insert(0),
            1 => parameter,
            2 => *self
                .program
                .entry(parameter + self.relative_base)
                .or_insert(0),
            x => panic!("{} is a wrong parameter mode!", x),
        }
    }

    fn get_input_params(&mut self, modes: &mut Vec<isize>, number_of_params: usize) -> Vec<isize> {
        let mut params = Vec::new();

        for _i in 0..number_of_params {
            let param = *self.program.entry(self.pointer).or_insert(0);

            params.push(self.get_parameter_value(
                param,
                if !modes.is_empty() {
                    modes.remove(0)
                } else {
                    0
                },
            ));

            self.pointer += 1;
        }

        params
    }

    fn get_output_param(&mut self, mode: isize) -> isize {
        let param = *self.program.entry(self.pointer).or_insert(0);
        self.pointer += 1;

        match mode {
            0 => param,
            2 => param + self.relative_base,
            x => panic!("{} is a wrong parameter mode for output!", x),
        }
    }

    fn add(&mut self, modes: &mut Vec<isize>) {
        let input_params = self.get_input_params(modes, 2);

        let output_mode = if modes.is_empty() { 0 } else { modes.remove(0) };
        let output_param = self.get_output_param(output_mode);

        self.program
            .insert(output_param, input_params[0] + input_params[1]);
    }

    fn multiply(&mut self, modes: &mut Vec<isize>) {
        let input_params = self.get_input_params(modes, 2);
        let output_mode = if modes.is_empty() { 0 } else { modes.remove(0) };
        let output_param = self.get_output_param(output_mode);

        self.program
            .insert(output_param, input_params[0] * input_params[1]);
    }

    fn get_input(&mut self, modes: &mut Vec<isize>, input: isize) {
        let output_mode = if modes.is_empty() { 0 } else { modes.remove(0) };
        let output_param = self.get_output_param(output_mode);

        self.program.insert(output_param, input);
    }

    fn calculate_output(&mut self, modes: &mut Vec<isize>) {
        let input_param = self.get_input_params(modes, 1)[0];

        self.output.push(input_param);
    }

    fn adjust_relative_base(&mut self, modes: &mut Vec<isize>) {
        let input_param = self.get_input_params(modes, 1)[0];

        self.relative_base += input_param;
    }

    fn jump_if_true(&mut self, modes: &mut Vec<isize>) {
        let input_params = self.get_input_params(modes, 2);

        if input_params[0] != 0 {
            self.pointer = input_params[1];
        }
    }

    fn jump_if_false(&mut self, modes: &mut Vec<isize>) {
        let input_params = self.get_input_params(modes, 2);

        if input_params[0] == 0 {
            self.pointer = input_params[1];
        }
    }

    fn less_than(&mut self, modes: &mut Vec<isize>) {
        let input_params = self.get_input_params(modes, 2);
        let output_mode = if modes.is_empty() { 0 } else { modes.remove(0) };
        let output_param = self.get_output_param(output_mode);

        self.program.insert(
            output_param,
            if input_params[0] < input_params[1] {
                1
            } else {
                0
            },
        );
    }

    fn equals(&mut self, modes: &mut Vec<isize>) {
        let input_params = self.get_input_params(modes, 2);
        let output_mode = if modes.is_empty() { 0 } else { modes.remove(0) };
        let output_param = self.get_output_param(output_mode);

        self.program.insert(
            output_param,
            if input_params[0] == input_params[1] {
                1
            } else {
                0
            },
        );
    }

    fn parse_instruction(instruction: isize) -> (isize, Vec<isize>) {
        let opcode = instruction % 100;
        let mut modes = Vec::new();

        let mut modes_num = instruction / 100;

        while modes_num > 0 {
            modes.push(modes_num % 10);
            modes_num /= 10;
        }

        (opcode, modes)
    }

    fn compute(&mut self) {
        loop {
            let instruction = *self.program.entry(self.pointer).or_insert(0);
            let (opcode, mut modes) = Self::parse_instruction(instruction);
            self.pointer += 1;

            match opcode {
                1 => self.add(&mut modes),
                2 => self.multiply(&mut modes),
                3 => {
                    if self.input.is_empty() {
                        break;
                    } else {
                        let input = self.input.remove(0);
                        self.get_input(&mut modes, input)
                    }
                }
                4 => self.calculate_output(&mut modes),
                5 => self.jump_if_true(&mut modes),
                6 => self.jump_if_false(&mut modes),
                7 => self.less_than(&mut modes),
                8 => self.equals(&mut modes),
                9 => self.adjust_relative_base(&mut modes),
                99 => break,
                _ => panic!("unknown operand!"),
            }
        }
    }
}

fn main() {
    let mut computer = Computer::init(&read_input(INPUT_PATH), vec![2]);
    computer.compute();

    for r in computer.output {
        println!("{}", r);
    }
}
