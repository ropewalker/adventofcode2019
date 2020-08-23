#![allow(dead_code)]

use crate::read_input::read_input;

#[derive(Debug)]
pub enum Technique {
    DealIntoNewStack,
    Cut(isize),
    DealWithIncrement(isize),
}

#[derive(Debug)]
pub struct Process {
    pub techniques: Vec<Technique>,
}

impl Process {
    pub fn init() -> Process {
        let input = read_input();
        let mut techniques = Vec::new();

        for line in input.lines() {
            let technique = if line.contains("deal with increment") {
                Technique::DealWithIncrement(
                    line.split(' ').nth(3).unwrap().parse::<isize>().unwrap(),
                )
            } else if line.contains("cut") {
                Technique::Cut(line.split(' ').nth(1).unwrap().parse::<isize>().unwrap())
            } else {
                Technique::DealIntoNewStack
            };

            techniques.push(technique);
        }

        Process { techniques }
    }
}
