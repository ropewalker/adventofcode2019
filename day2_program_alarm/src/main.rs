use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
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

    for parameter1 in 0..100 {
        for parameter2 in 0..100 {
            let mut computer: Vec<usize> =
                s.split(',').map(|x| x.parse::<usize>().unwrap()).collect();

            computer[1] = parameter1;
            computer[2] = parameter2;

            let mut i: usize = 0;

            while i < computer.len() {
                match computer[i] {
                    1 => {
                        let operand1 = computer[i + 1];
                        let operand2 = computer[i + 2];
                        let operand3 = computer[i + 3];

                        computer[operand3] = computer[operand1] + computer[operand2];

                        i += 4
                    }
                    2 => {
                        let operand1 = computer[i + 1];
                        let operand2 = computer[i + 2];
                        let operand3 = computer[i + 3];

                        computer[operand3] = computer[operand1] * computer[operand2];

                        i += 4
                    }
                    99 => {
                        break;
                    }
                    _ => panic!("unknown operand!"),
                }
            }

            if computer[0] == 19_690_720 {
                println!("{}", 100 * parameter1 + parameter2);
                break;
            };
        }
    }
}