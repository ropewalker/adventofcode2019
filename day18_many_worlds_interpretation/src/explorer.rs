use super::maze::CellType;
use super::maze::Direction;
use super::maze::*;
use crate::read_input::read_input;
use std::collections::{HashMap, HashSet, VecDeque};

type Point = (usize, usize);

pub struct Explorer {
    pub maze_map: HashMap<Point, CellType>,
    pub keys: HashMap<char, Point>,
    pub starting_positions: Vec<Point>,
    pub min_path_length: Option<usize>,
    pub distances: HashMap<(Point, char), usize>,
    pub keys_between: HashMap<(Point, char), HashSet<char>>,
    pub doors_per_key: HashMap<char, HashSet<char>>,
    pub found: HashMap<(Vec<Point>, Vec<char>), Option<usize>>,
}

impl Explorer {
    pub fn init() -> Explorer {
        let s = read_input();
        let mut maze_map = HashMap::new();
        let mut keys = HashMap::new();
        let mut starting_positions = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    starting_positions.push((x, y));
                } else if c.is_ascii_lowercase() {
                    keys.insert(c.to_ascii_uppercase(), (x, y));
                }

                maze_map.insert((x, y), CellType::from(c));
            }
        }

        let mut distances: HashMap<(Point, char), usize> = HashMap::new();
        let mut keys_between: HashMap<(Point, char), HashSet<char>> = HashMap::new();

        for &key_position in keys.values().chain(starting_positions.iter()) {
            let mut visited: HashSet<Point> = [key_position].iter().cloned().collect();
            let mut queue: VecDeque<KeyNode> = VecDeque::from(vec![KeyNode {
                coordinates: key_position,
                chars: HashSet::new(),
                distance: 0,
            }]);

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();

                for neighbour in [
                    find_neighbour_cell(node.coordinates, Direction::North),
                    find_neighbour_cell(node.coordinates, Direction::South),
                    find_neighbour_cell(node.coordinates, Direction::East),
                    find_neighbour_cell(node.coordinates, Direction::West),
                ]
                .iter()
                {
                    if !visited.contains(neighbour) {
                        match maze_map.get(neighbour) {
                            Some(&CellType::Wall) => {}
                            Some(&CellType::Key(some_key)) => {
                                distances.insert((key_position, some_key), node.distance + 1);
                                keys_between.insert((key_position, some_key), node.chars.clone());

                                let mut chars = node.chars.clone();
                                chars.insert(some_key);

                                queue.push_back(KeyNode {
                                    coordinates: *neighbour,
                                    distance: node.distance + 1,
                                    chars,
                                });
                                visited.insert(*neighbour);
                            }
                            Some(&CellType::Door(_)) => {
                                queue.push_back(KeyNode {
                                    coordinates: *neighbour,
                                    chars: node.chars.clone(),
                                    distance: node.distance + 1,
                                });
                                visited.insert(*neighbour);
                            }
                            Some(&CellType::Empty) => {
                                queue.push_back(KeyNode {
                                    coordinates: *neighbour,
                                    chars: node.chars.clone(),
                                    distance: node.distance + 1,
                                });
                                visited.insert(*neighbour);
                            }
                            None => panic!("AAAAA!!"),
                        }
                    }
                }
            }
        }

        let mut doors_per_key: HashMap<char, HashSet<char>> = HashMap::new();

        for &starting_position in starting_positions.iter() {
            let mut visited: HashSet<Point> = [starting_position].iter().cloned().collect();
            let mut queue: VecDeque<KeyNode> = VecDeque::from(vec![KeyNode {
                coordinates: starting_position,
                chars: HashSet::new(),
                distance: 0,
            }]);

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();

                for neighbour in [
                    find_neighbour_cell(node.coordinates, Direction::North),
                    find_neighbour_cell(node.coordinates, Direction::South),
                    find_neighbour_cell(node.coordinates, Direction::East),
                    find_neighbour_cell(node.coordinates, Direction::West),
                ]
                .iter()
                {
                    if !visited.contains(neighbour) {
                        match maze_map.get(neighbour) {
                            Some(&CellType::Wall) => {}
                            Some(&CellType::Key(some_key)) => {
                                doors_per_key.insert(some_key, node.chars.clone());
                                queue.push_back(KeyNode {
                                    coordinates: *neighbour,
                                    chars: node.chars.clone(),
                                    distance: node.distance + 1,
                                });
                                visited.insert(*neighbour);
                            }
                            Some(&CellType::Door(door_key)) => {
                                let mut doors_passsed = node.chars.clone();
                                doors_passsed.insert(door_key);

                                queue.push_back(KeyNode {
                                    coordinates: *neighbour,
                                    chars: doors_passsed,
                                    distance: node.distance + 1,
                                });
                                visited.insert(*neighbour);
                            }
                            _ => {
                                queue.push_back(KeyNode {
                                    coordinates: *neighbour,
                                    chars: node.chars.clone(),
                                    distance: node.distance + 1,
                                });
                                visited.insert(*neighbour);
                            }
                        }
                    }
                }
            }
        }

        Explorer {
            maze_map,
            keys,
            starting_positions,
            min_path_length: None,
            distances,
            keys_between,
            doors_per_key,
            found: HashMap::new(),
        }
    }
    //
    pub fn reachable_from(&self, point: Point) -> HashSet<char> {
        self.distances
            .keys()
            .filter(|&(x, _)| *x == point)
            .map(|&(_, c)| c)
            .collect()
    }

    pub fn find_keys(
        &mut self,
        starting_coords: Vec<Point>,
        keys_collected: HashSet<char>,
    ) -> Option<usize> {
        if keys_collected.len() == self.keys.len() {
            return Some(0);
        }

        let mut min_result = None;
        let keys: HashSet<char> = self
            .distances
            .keys()
            .filter(|&(x, _)| {
                *x == starting_coords[0]
                    || *x == starting_coords[1]
                    || *x == starting_coords[2]
                    || *x == starting_coords[3]
            })
            .map(|&(_, c)| c)
            .filter(|&k| !keys_collected.contains(&k))
            .filter(|&k| {
                self.doors_per_key
                    .get(&k)
                    .unwrap()
                    .difference(&keys_collected)
                    .count()
                    == 0
            })
            .collect();

        for key in keys {
            let mut j = 0;

            if self.reachable_from(starting_coords[0]).contains(&key) {
                j = 0;
            } else if self.reachable_from(starting_coords[1]).contains(&key) {
                j = 1;
            } else if self.reachable_from(starting_coords[2]).contains(&key) {
                j = 2;
            } else if self.reachable_from(starting_coords[3]).contains(&key) {
                j = 3;
            }

            let dist = *self.distances.get(&(starting_coords[j], key)).unwrap();

            let mut keys_collected = keys_collected.clone();
            keys_collected.insert(key);

            for &i in self.keys_between.get(&(starting_coords[j], key)).unwrap() {
                keys_collected.insert(i);
            }

            let mut key_coords = starting_coords.clone();
            key_coords[j] = *self.keys.get(&key).unwrap();

            let mut keys_vector: Vec<char> = keys_collected.iter().cloned().collect::<Vec<char>>();
            keys_vector.sort();

            let result: Option<usize>;

            if let Some(&r) = self.found.get(&(key_coords.clone(), keys_vector.clone())) {
                result = r;
            } else {
                result = self.find_keys(key_coords.clone(), keys_collected);
                self.found
                    .insert((key_coords.clone(), keys_vector), result);
            }

            if let Some(length) = result {
                if let Some(min_length) = min_result {
                    if length + dist < min_length {
                        min_result = Some(length + dist);
                    }
                } else {
                    min_result = Some(length + dist);
                }
            }
        }

        min_result
    }

    //    pub fn print_maze(&self) {
    //        let max_x = self.maze_map.keys().map(|v| v.0).max().unwrap_or_else(|| 0);
    //        let max_y = self.maze_map.keys().map(|v| v.1).max().unwrap_or_else(|| 0);
    //
    //        for y in 0..=max_y {
    //            for x in 0..=max_x {
    //                let cell_type = self.maze_map.get(&(x, y));
    //
    //                print!(
    //                    "{}",
    //                    match cell_type {
    //                        Some(CellType::Wall) => '#',
    //                        Some(CellType::Empty) => '.',
    //                        Some(CellType::Key(c)) => c.to_ascii_lowercase(),
    //                        Some(CellType::Door(c)) => *c,
    //                        None => '.',
    //                    }
    //                );
    //            }
    //
    //            println!();
    //        }
    //    }
}
