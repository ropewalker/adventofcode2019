use super::maze::CellType;
use super::maze::Direction;
use super::maze::*;
use crate::read_input::read_input;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Explorer {
    pub maze_map: HashMap<Point, CellType>,
    pub portals: HashMap<String, Vec<Point>>,
    pub start: Point,
    pub end: Point,
}

impl Explorer {
    pub fn init() -> Explorer {
        let s = read_input();
        let mut maze_map = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                maze_map.insert((x, y), CellType::from(c));
            }
        }

        let mut portals = HashMap::new();

        let max_x = maze_map.keys().map(|v| v.0).max().unwrap_or_else(|| 0);
        let max_y = maze_map.keys().map(|v| v.1).max().unwrap_or_else(|| 0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                if let Some(&CellType::Nothing(a @ 'A'..='Z')) = maze_map.get(&(x, y)) {
                    let mut key = a.to_string();

                    if let Some(&CellType::Nothing(b @ 'A'..='Z')) = maze_map.get(&(x + 1, y)) {
                        key.push(b);

                        let exits: &mut Vec<Point> = portals.entry(key).or_insert_with(Vec::new);

                        if let Some(&CellType::Empty) = maze_map.get(&(x + 2, y)) {
                            (*exits).push((x + 2, y));
                        } else if let Some(&CellType::Empty) = maze_map.get(&(x - 1, y)) {
                            (*exits).push((x - 1, y));
                        }
                    } else if let Some(&CellType::Nothing(b @ 'A'..='Z')) =
                        maze_map.get(&(x, y + 1))
                    {
                        key.push(b);

                        let exits: &mut Vec<Point> = portals.entry(key).or_insert_with(Vec::new);

                        if let Some(&CellType::Empty) = maze_map.get(&(x, y + 2)) {
                            (*exits).push((x, y + 2));
                        } else if let Some(&CellType::Empty) = maze_map.get(&(x, y - 1)) {
                            (*exits).push((x, y - 1));
                        }
                    }
                }
            }
        }

        let start = portals.get("AA").unwrap()[0];
        let end = portals.get("ZZ").unwrap()[0];

        for exits in portals.values() {
            if exits.len() == 2 {
                let entry = exits[0];
                let exit = exits[1];

                if entry.0 == 2 || entry.0 == max_x - 2 || entry.1 == 2 || entry.1 == max_y - 2 {
                    maze_map.insert(entry, CellType::StairUp(exit));
                    maze_map.insert(exit, CellType::StairDown(entry));
                } else {
                    maze_map.insert(entry, CellType::StairDown(exit));
                    maze_map.insert(exit, CellType::StairUp(entry));
                }
            }
        }

        Explorer {
            maze_map,
            portals,
            start,
            end,
        }
    }

    pub fn shortest_path(&self) -> Option<usize> {
        let mut visited_by_level: HashMap<usize, HashSet<Point>> =
            [(0, [self.start].iter().cloned().collect())]
                .iter()
                .cloned()
                .collect();
        let mut queue: VecDeque<Node> = VecDeque::from(vec![Node {
            coordinates: self.start,
            distance: 0,
            level: 0,
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
                let mut visited = visited_by_level
                    .entry(node.level)
                    .or_insert_with(HashSet::new);

                if !visited.contains(neighbour) {
                    match self.maze_map.get(neighbour) {
                        Some(&CellType::Empty) => {
                            if *neighbour == self.end && node.level == 0 {
                                return Some(node.distance + 1);
                            }

                            queue.push_back(Node {
                                coordinates: *neighbour,
                                distance: node.distance + 1,
                                level: node.level,
                            });
                            visited.insert(*neighbour);
                        }
                        Some(&CellType::StairDown(exit)) => {
                            queue.push_back(Node {
                                coordinates: exit,
                                distance: node.distance + 2,
                                level: node.level + 1,
                            });
                            visited.insert(*neighbour);

                            visited = visited_by_level
                                .entry(node.level + 1)
                                .or_insert_with(HashSet::new);

                            visited.insert(exit);
                        }
                        Some(&CellType::StairUp(exit)) => {
                            if node.level > 0 {
                                queue.push_back(Node {
                                    coordinates: exit,
                                    distance: node.distance + 2,
                                    level: node.level - 1,
                                });
                                visited.insert(*neighbour);

                                visited = visited_by_level
                                    .entry(node.level - 1)
                                    .or_insert_with(HashSet::new);

                                visited.insert(exit);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        None
    }

    pub fn print_maze(&self) {
        let max_x = self.maze_map.keys().map(|v| v.0).max().unwrap_or_else(|| 0);
        let max_y = self.maze_map.keys().map(|v| v.1).max().unwrap_or_else(|| 0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                let cell_type = self.maze_map.get(&(x, y));

                print!(
                    "{}",
                    match cell_type {
                        Some(CellType::Wall) => '#',
                        Some(CellType::Empty) => '.',
                        Some(CellType::Nothing(c)) => *c,
                        Some(CellType::StairDown(_)) => 'v',
                        Some(CellType::StairUp(_)) => '^',
                        None => ' ',
                    }
                );
            }

            println!();
        }
    }
}
