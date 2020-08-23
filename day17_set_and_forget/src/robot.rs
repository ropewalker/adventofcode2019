use super::intcode::Computer;
use super::maze::Direction;
use super::maze::*;
use std::collections::{HashMap, HashSet};

pub struct Robot {
    pub computer: Computer,
    pub maze: HashMap<(isize, isize), char>,
    pub position: (isize, isize),
    pub direction: Direction,
}

impl Robot {
    pub fn init(program: &str) -> Robot {
        let mut m = vec![
            //B,C,B,C,A,B,A,B,A,C
            'B' as isize,
            44,
            'C' as isize,
            44,
            'B' as isize,
            44,
            'C' as isize,
            44,
            'A' as isize,
            44,
            'B' as isize,
            44,
            'A' as isize,
            44,
            'B' as isize,
            44,
            'A' as isize,
            44,
            'C' as isize,
            10,
        ];
        let mut a = vec![
            'L' as isize,
            44,
            '8' as isize,
            44,
            'L' as isize,
            44,
            '4' as isize,
            44,
            'R' as isize,
            44,
            '1' as isize,
            '2' as isize,
            44,
            'L' as isize,
            44,
            '6' as isize,
            44,
            'L' as isize,
            44,
            '4' as isize,
            10,
        ];
        let mut b = vec![
            'R' as isize,
            44,
            '1' as isize,
            '2' as isize,
            44,
            'L' as isize,
            44,
            '8' as isize,
            44,
            'L' as isize,
            44,
            '4' as isize,
            44,
            'L' as isize,
            44,
            '4' as isize,
            10,
        ];
        let mut c = vec![
            'L' as isize,
            44,
            '8' as isize,
            44,
            'R' as isize,
            44,
            '6' as isize,
            44,
            'L' as isize,
            44,
            '6' as isize,
            10,
        ];
        let mut n = vec!['n' as isize, 10];

        let mut input: Vec<isize> = Vec::new();
        input.append(&mut m);
        input.append(&mut a);
        input.append(&mut b);
        input.append(&mut c);
        input.append(&mut n);

        let mut computer = Computer::init(program, input);
        computer.compute();

        dbg!(computer.get_output());

        let mut x = 0;
        let mut y = 0;
        let mut position = (0, 0);
        let mut direction = Direction::North;

        let mut maze: HashMap<(isize, isize), char> = HashMap::new();

        for output in computer.get_output() {
            if output == 10 {
                //carriage return
                y += 1;
                x = 0;
            } else {
                let c = std::char::from_u32(output as u32).unwrap();

                match c {
                    '^' => {
                        position = (x, y);
                        direction = Direction::North;
                        maze.insert((x, y), '#');
                    }
                    'v' => {
                        position = (x, y);
                        direction = Direction::South;
                        maze.insert((x, y), '#');
                    }
                    '>' => {
                        position = (x, y);
                        direction = Direction::East;
                        maze.insert((x, y), '#');
                    }
                    '<' => {
                        position = (x, y);
                        direction = Direction::West;
                        maze.insert((x, y), '#');
                    }
                    _ => {
                        maze.insert((x, y), std::char::from_u32(output as u32).unwrap());
                    }
                }

                x += 1;
            }
        }

        Robot {
            computer,
            maze,
            position,
            direction,
        }
    }

    fn try_step(&mut self, direction: Direction) -> bool {
        if let Some('#') = self
            .maze
            .get(&find_neighbour_cell(self.position, direction))
        {
            self.direction = direction;
            self.position = find_neighbour_cell(self.position, direction);
            true
        } else {
            false
        }
    }

    pub fn find_path(&mut self) -> String {
        let mut path = String::new();

        let mut steps = 0;

        loop {
            if self.try_step(self.direction) {
                steps += 1;

                continue;
            }

            if self.try_step(turn_left(self.direction)) {
                if steps != 0 {
                    path.push_str(&steps.to_string());
                    path.push(',');
                }

                steps = 1;

                path.push('L');
                path.push(',');

                continue;
            }

            if self.try_step(turn_right(self.direction)) {
                if steps != 0 {
                    path.push_str(&steps.to_string());
                    path.push(',');
                }

                steps = 1;

                path.push('R');
                path.push(',');

                continue;
            }

            if steps != 0 {
                path.push_str(&steps.to_string());
                path.push(10 as char);
            }

            return path;
        }
    }

    pub fn find_intersections(&mut self) -> isize {
        let mut visited = HashSet::new();
        let mut visited_twice = HashSet::new();

        loop {
            if self.try_step(self.direction) {
                if visited.contains(&self.position) {
                    visited_twice.insert(self.position);
                } else {
                    visited.insert(self.position);
                }

                continue;
            }

            if self.try_step(turn_left(self.direction)) {
                if visited.contains(&self.position) {
                    visited_twice.insert(self.position);
                } else {
                    visited.insert(self.position);
                }

                continue;
            }

            if self.try_step(turn_right(self.direction)) {
                if visited.contains(&self.position) {
                    visited_twice.insert(self.position);
                } else {
                    visited.insert(self.position);
                }

                continue;
            }

            return visited_twice.iter().map(|(x, y)| x * y).sum();
        }
    }

    pub fn print_maze(&mut self) {
        let max_x = self.maze.keys().map(|v| v.0).max().unwrap_or_else(|| 0);
        let max_y = self.maze.keys().map(|v| v.1).max().unwrap_or_else(|| 0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                if (x, y) == self.position {
                    print!(
                        "{}",
                        match self.direction {
                            Direction::North => "^",
                            Direction::South => "V",
                            Direction::East => ">",
                            Direction::West => "<",
                        }
                    );
                } else {
                    print!("{}", self.maze.entry((x, y)).or_insert(' '),);
                }
            }

            println!();
        }

//        println!("Position: ({}, {})", self.position.0, self.position.1);
//        println!(
//            "Direction: {}",
//            match self.direction {
//                Direction::North => "North",
//                Direction::South => "South",
//                Direction::East => "East",
//                Direction::West => "West",
//            }
//        );
    }
}
