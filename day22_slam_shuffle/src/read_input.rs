#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const INPUT_PATH: &str = "input.txt";

pub fn read_input() -> String {
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
