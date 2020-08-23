use termion::{clear, cursor, color, style};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::path::Path;
use std::{thread, time};

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

    String::from(s.trim())
}

struct Computer {
    program: HashMap<isize, isize>,
    pointer: isize,
    relative_base: isize,
    input: Vec<isize>,
    output: Vec<isize>,
    is_halted: bool,
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
            is_halted: false,
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
                0 => self.pointer = 0,
                1 => self.add(&mut modes),
                2 => self.multiply(&mut modes),
                3 => {
                    if self.input.is_empty() {
                        self.pointer -= 1;
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
                99 => {
                    self.is_halted = true;
                    break;
                }
                x => panic!("unknown operand! {}", x),
            }
        }
    }
}

fn main() {
    let mut computer = Computer::init(&read_input(INPUT_PATH), Vec::new());
    computer.compute();

    loop {
        let mut tiles_with_coords: HashMap<(isize, isize), isize> = HashMap::new();

        let x_coords: Vec<isize> = computer
            .output
            .iter()
            .enumerate()
            .filter(|(i, &_o)| i % 3 == 0)
            .map(|(_i, &o)| o)
            .collect();
        let y_coords: Vec<isize> = computer
            .output
            .iter()
            .enumerate()
            .filter(|(i, &_o)| i % 3 == 1)
            .map(|(_i, &o)| o)
            .collect();
        let tiles: Vec<isize> = computer
            .output
            .iter()
            .enumerate()
            .filter(|(i, &_o)| i % 3 == 2)
            .map(|(_i, &o)| o)
            .collect();

        for i in 0..x_coords.len() {
            tiles_with_coords.insert((x_coords[i], y_coords[i]), tiles[i]);
        }

        let num_of_blocks = tiles_with_coords.values().filter(|&v| *v == 2).count();

        write!(io::stdout(), "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();

        println!(
            "SCORE: {}{}",
            color::Fg(color::White),
            tiles_with_coords.entry((-1, 0)).or_insert(0)
        );

        if num_of_blocks == 0 {
            break;
        }

        let max_x = tiles_with_coords
            .keys()
            .map(|v| v.0)
            .max()
            .unwrap_or_else(|| 0);
        let max_y = tiles_with_coords
            .keys()
            .map(|v| v.1)
            .max()
            .unwrap_or_else(|| 0);

        let mut ball_x = 0;
        let mut plank_x = 0;

        for y in 0..=max_y {
            for x in 0..=max_x {
                let tile = tiles_with_coords.entry((x, y)).or_insert(0);

                match tile {
                    1 => print!("{}{}", color::Fg(color::Blue), "█"),
                    2 => print!("{}{}", color::Fg(color::Red), "▒"),
                    3 => {
                        plank_x = x;
                        print!("{}{}", color::Fg(color::White), "—");
                    }
                    4 => {
                        ball_x = x;
                        print!("{}{}", color::Fg(color::White), "o");
                    }
                    _ => print!("{}{}", color::Fg(color::Black), " "),
                }
            }

            println!();
        }

        let delay = time::Duration::from_millis(100);

        thread::sleep(delay);

        //        let mut input_command = String::new();
        //
        //        io::stdin()
        //            .read_line(&mut input_command).expect("shit");
        //
        //        match input_command.trim().parse().unwrap() {
        //            'a' => computer.input = vec![-1],
        //            's' => computer.input = vec![0],
        //            'd' => computer.input = vec![1],
        //            x => panic!("wrong input: {}", x),
        //        }

        computer.input = vec![match ball_x - plank_x {
            x if x < 0 => -1,
            x if x > 0 => 1,
            _ => 0,
        }];

        computer.compute();
    }
}
