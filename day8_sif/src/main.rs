use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const INPUT_PATH: &str = "input.txt";
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

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

    String::from(s.trim())
}

fn main() {
    let s = read_input();

    let size = WIDTH * HEIGHT;
    let mut result: String = String::new();

    for i in 0..size {
        for layer in s.chars().collect::<Vec<char>>().chunks(size) {
            if layer[i] != '2' {
                result.push(layer[i]);
                break;
            }
        }
    }

    for layer in result.chars().collect::<Vec<char>>().chunks(WIDTH) {
        let mut layer_string: String = String::new();

        for &c in layer {
            if c == '1' {
                layer_string.push(' ');
            } else {
                layer_string.push('█');
            }
        }

        println!("{}", layer_string);
    }
}
