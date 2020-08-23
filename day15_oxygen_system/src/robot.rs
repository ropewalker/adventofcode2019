use super::intcode::Computer;
use super::maze::CellType;
use super::maze::Direction;
use super::maze::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Robot {
    pub computer: Computer,
    pub maze: HashMap<(isize, isize), CellType>,
    pub position: (isize, isize),
    pub direction: Direction,
}

impl Robot {
    pub fn init(program: &str) -> Robot {
        Robot {
            computer: Computer::init(program, Vec::new()),
            maze: [((0, 0), CellType::Empty)].iter().cloned().collect(),
            position: (0, 0),
            direction: Direction::North,
        }
    }

    fn try_step(&mut self, direction: Direction) -> CellType {
        self.computer.set_input(vec![direction as isize]);
        self.computer.compute();

        let status = CellType::from(self.computer.read_output());
        self.maze
            .insert(find_neighbour_cell(self.position, direction), status);

        if status != CellType::Wall {
            self.position = find_neighbour_cell(self.position, direction);
            self.direction = direction;
        }

        status
    }

    pub fn find_oxygen_system(&mut self) -> (isize, isize) {
        let mut oxygen_system_coords: (isize, isize) = (0, 0);
        let mut path: Vec<(isize, isize)> = Vec::new();
        let mut direction;

        loop {
            let current_position = self.position;

            direction = turn_left(self.direction);
            let mut adjacent_cell_coords = find_neighbour_cell(self.position, direction);

            if !self.maze.contains_key(&adjacent_cell_coords) {
                let adjacent_cell_type = self.try_step(direction);

                match adjacent_cell_type {
                    CellType::Wall => {}
                    CellType::OxygenSystem => {
                        path.push(current_position);
                        oxygen_system_coords = self.position;
                        continue;
                    }
                    CellType::Empty => {
                        path.push(current_position);
                        continue;
                    }
                    CellType::Unknown => panic!("Unknown cell type!"),
                }
            }

            direction = self.direction;
            adjacent_cell_coords = find_neighbour_cell(self.position, direction);

            if !self.maze.contains_key(&adjacent_cell_coords) {
                let adjacent_cell_type = self.try_step(direction);

                match adjacent_cell_type {
                    CellType::Wall => {}
                    CellType::OxygenSystem => {
                        path.push(current_position);
                        oxygen_system_coords = self.position;
                        continue;
                    }
                    CellType::Empty => {
                        path.push(current_position);
                        continue;
                    }
                    CellType::Unknown => panic!("Unknown cell type!"),
                }
            }

            direction = turn_right(self.direction);
            adjacent_cell_coords = find_neighbour_cell(self.position, direction);

            if !self.maze.contains_key(&adjacent_cell_coords) {
                let adjacent_cell_type = self.try_step(direction);

                match adjacent_cell_type {
                    CellType::Wall => {}
                    CellType::OxygenSystem => {
                        path.push(current_position);
                        oxygen_system_coords = self.position;
                        continue;
                    }
                    CellType::Empty => {
                        path.push(current_position);
                        continue;
                    }
                    CellType::Unknown => panic!("Unknown cell type!"),
                }
            }

            direction = -self.direction;
            adjacent_cell_coords = find_neighbour_cell(self.position, direction);

            if !self.maze.contains_key(&adjacent_cell_coords) {
                let adjacent_cell_type = self.try_step(direction);

                match adjacent_cell_type {
                    CellType::Wall => {}
                    CellType::OxygenSystem => {
                        path.push(current_position);
                        oxygen_system_coords = self.position;
                        continue;
                    }
                    CellType::Empty => {
                        path.push(current_position);
                        continue;
                    }
                    CellType::Unknown => panic!("Unknown cell type!"),
                }
            }

            if self.position == (0, 0) {
                break;
            }

            let previous_position = path.pop().unwrap_or_else(|| (0, 0));

            direction = match (
                self.position.0 - previous_position.0,
                self.position.1 - previous_position.1,
            ) {
                (-1, 0) => Direction::East,
                (1, 0) => Direction::West,
                (0, -1) => Direction::North,
                (0, 1) => Direction::South,
                _ => {
                    dbg!(path);
                    panic!("Something wrong!")
                }
            };

            self.try_step(direction);
        }

        oxygen_system_coords
    }

    pub fn find_shortest_path(&mut self) -> isize {
        let oxygen_system_coords = self.find_oxygen_system();

        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut queue: VecDeque<Node> = VecDeque::new();

        visited.insert((0, 0));
        queue.push_back(Node {
            coordinates: (0, 0),
            distance: 0,
        });

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if node.coordinates == oxygen_system_coords {
                return node.distance;
            }

            let neighbour = find_neighbour_cell(node.coordinates, Direction::North);

            if !visited.contains(&neighbour) {
                match self.maze.get(&neighbour) {
                    Some(&CellType::OxygenSystem) => return node.distance + 1,
                    Some(&CellType::Empty) => {
                        queue.push_back(Node {
                            coordinates: neighbour,
                            distance: node.distance + 1,
                        });
                        visited.insert(neighbour);
                    }
                    _ => {}
                }
            }

            let neighbour = find_neighbour_cell(node.coordinates, Direction::South);

            if !visited.contains(&neighbour) {
                match self.maze.get(&neighbour) {
                    Some(&CellType::OxygenSystem) => return node.distance + 1,
                    Some(&CellType::Empty) => {
                        queue.push_back(Node {
                            coordinates: neighbour,
                            distance: node.distance + 1,
                        });
                        visited.insert(neighbour);
                    }
                    _ => {}
                }
            }

            let neighbour = find_neighbour_cell(node.coordinates, Direction::East);

            if !visited.contains(&neighbour) {
                match self.maze.get(&neighbour) {
                    Some(&CellType::OxygenSystem) => return node.distance + 1,
                    Some(&CellType::Empty) => {
                        queue.push_back(Node {
                            coordinates: neighbour,
                            distance: node.distance + 1,
                        });
                        visited.insert(neighbour);
                    }
                    _ => {}
                }
            }

            let neighbour = find_neighbour_cell(node.coordinates, Direction::West);

            if !visited.contains(&neighbour) {
                match self.maze.get(&neighbour) {
                    Some(&CellType::OxygenSystem) => return node.distance + 1,
                    Some(&CellType::Empty) => {
                        queue.push_back(Node {
                            coordinates: neighbour,
                            distance: node.distance + 1,
                        });
                        visited.insert(neighbour);
                    }
                    _ => {}
                }
            }
        }

        -1
    }

    pub fn oxygen_fill(&mut self) -> isize {
        let oxygen_system_coords = self.find_oxygen_system();

        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut queue: VecDeque<Node> = VecDeque::new();

        visited.insert((oxygen_system_coords));
        queue.push_back(Node {
            coordinates: (oxygen_system_coords),
            distance: 0,
        });

        let mut max_dist = 0;

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if node.distance > max_dist {
                max_dist = node.distance;
            }

            let neighbour = find_neighbour_cell(node.coordinates, Direction::North);

            if !visited.contains(&neighbour) {
                match self.maze.get(&neighbour) {
                    Some(&CellType::Empty) => {
                        queue.push_back(Node {
                            coordinates: neighbour,
                            distance: node.distance + 1,
                        });
                        visited.insert(neighbour);
                    }
                    _ => {}
                }
            }

            let neighbour = find_neighbour_cell(node.coordinates, Direction::South);

            if !visited.contains(&neighbour) {
                match self.maze.get(&neighbour) {
                    Some(&CellType::Empty) => {
                        queue.push_back(Node {
                            coordinates: neighbour,
                            distance: node.distance + 1,
                        });
                        visited.insert(neighbour);
                    }
                    _ => {}
                }
            }

            let neighbour = find_neighbour_cell(node.coordinates, Direction::East);

            if !visited.contains(&neighbour) {
                match self.maze.get(&neighbour) {
                    Some(&CellType::Empty) => {
                        queue.push_back(Node {
                            coordinates: neighbour,
                            distance: node.distance + 1,
                        });
                        visited.insert(neighbour);
                    }
                    _ => {}
                }
            }

            let neighbour = find_neighbour_cell(node.coordinates, Direction::West);

            if !visited.contains(&neighbour) {
                match self.maze.get(&neighbour) {
                    Some(&CellType::Empty) => {
                        queue.push_back(Node {
                            coordinates: neighbour,
                            distance: node.distance + 1,
                        });
                        visited.insert(neighbour);
                    }
                    _ => {}
                }
            }
        }

        max_dist
    }

    pub fn print_maze(&mut self) {
        let min_x = self.maze.keys().map(|v| v.0).min().unwrap_or_else(|| 0);
        let max_x = self.maze.keys().map(|v| v.0).max().unwrap_or_else(|| 0);
        let min_y = self.maze.keys().map(|v| v.1).min().unwrap_or_else(|| 0);
        let max_y = self.maze.keys().map(|v| v.1).max().unwrap_or_else(|| 0);

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let cell_type = self.maze.entry((x, y)).or_insert(CellType::Unknown);

                print!(
                    "{}",
                    match cell_type {
                        CellType::Wall => "█",
                        CellType::Empty => " ",
                        CellType::OxygenSystem => "X",
                        CellType::Unknown => "?",
                    }
                );
            }

            println!();
        }
    }
}
