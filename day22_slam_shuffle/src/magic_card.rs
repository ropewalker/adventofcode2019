#![allow(dead_code)]

use crate::shuffle_process::{Process, Technique};

pub struct MagicCard {
    pub index: isize,
    pub deck_size: isize,
}

impl MagicCard {
    pub fn init(index: isize, deck_size: isize) -> MagicCard {
        MagicCard { index, deck_size }
    }

    pub fn complete_process(&mut self, process: &Process) -> isize {
        for technique in process.techniques.iter() {
            self.shuffle(technique);
        }

        self.index
    }

    pub fn shuffle(&mut self, technique: &Technique) {
        match *technique {
            Technique::DealIntoNewStack => {
                self.index = self.deck_size - self.index - 1;
            }
            Technique::DealWithIncrement(increment) => {
                self.index = self.index * increment % self.deck_size;
            }
            Technique::Cut(size) => {
                self.index =
                    (self.deck_size + (self.index - size) % self.deck_size) % self.deck_size;
            }
        }
    }

    pub fn quck_process(&self, process: &Process) -> (isize, isize) {
        let mut result = (1, 0);

        for technique in process.techniques.iter() {
            match *technique {
                Technique::DealIntoNewStack => {
                    result = (-result.0, -result.1 - 1);
                }
                Technique::DealWithIncrement(increment) => {
                    result = (result.0 * increment, result.1 * increment);
                }
                Technique::Cut(size) => {
                    result = (result.0, result.1 - size);
                }
            }

            result = (result.0 % self.deck_size, result.1 % self.deck_size)
        }

        result
    }

    pub fn complete_quick_process(&mut self, process: &Process) -> isize {
        let quick_process = self.quck_process(process);

        self.index = ((self.index * quick_process.0 + quick_process.1) % self.deck_size
            + self.deck_size)
            % self.deck_size;

        self.index
    }
}
