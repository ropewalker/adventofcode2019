use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CellType {
    Bug,
    Empty,
    LevelBelow,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Empty
    }
}

pub struct Eris {
    pub levels: HashMap<i32, Level>,
}

impl Eris {
    pub fn init(s: &str) -> Eris {
        Eris {
            levels: [(0, Level::init(s))].iter().cloned().collect(),
        }
    }

    pub fn count_bugs(&self) -> usize {
        self.levels
            .values()
            .map(|level| {
                level
                    .area
                    .iter()
                    .filter(|(_, &ct)| ct == CellType::Bug)
                    .count()
            })
            .sum()
    }

    fn count_bugs_around(&self, point: &Point, level_index: i32) -> usize {
        let current_level = self.levels.get(&level_index).unwrap();

        let mut bugs_count = 0;

        bugs_count += current_level
            .area
            .iter()
            .filter(|(&p, _)| {
                (p.x == point.x) && ((p.y == point.y + 1) || (p.y as i32 == point.y as i32 - 1))
                    || (p.y == point.y)
                        && ((p.x == point.x + 1) || (p.x as i32 == point.x as i32 - 1))
            })
            .filter(|(_, &ct)| ct == CellType::Bug)
            .count();

        if point.x == 0 {
            if let Some(level_above) = self.levels.get(&(level_index - 1)) {
                if *level_above.area.get(&Point { x: 1, y: 2 }).unwrap() == CellType::Bug {
                    bugs_count += 1;
                }
            }
        }

        if point.x == 4 {
            if let Some(level_above) = self.levels.get(&(level_index - 1)) {
                if *level_above.area.get(&Point { x: 3, y: 2 }).unwrap() == CellType::Bug {
                    bugs_count += 1;
                }
            }
        }

        if point.y == 0 {
            if let Some(level_above) = self.levels.get(&(level_index - 1)) {
                if *level_above.area.get(&Point { x: 2, y: 1 }).unwrap() == CellType::Bug {
                    bugs_count += 1;
                }
            }
        }

        if point.y == 4 {
            if let Some(level_above) = self.levels.get(&(level_index - 1)) {
                if *level_above.area.get(&Point { x: 2, y: 3 }).unwrap() == CellType::Bug {
                    bugs_count += 1;
                }
            }
        }

        if (point.x, point.y) == (1, 2) {
            if let Some(level_below) = self.levels.get(&(level_index + 1)) {
                bugs_count += level_below.bugs_on_edge(Edge::Left);
            }
        }

        if (point.x, point.y) == (3, 2) {
            if let Some(level_below) = self.levels.get(&(level_index + 1)) {
                bugs_count += level_below.bugs_on_edge(Edge::Right);
            }
        }

        if (point.x, point.y) == (2, 1) {
            if let Some(level_below) = self.levels.get(&(level_index + 1)) {
                bugs_count += level_below.bugs_on_edge(Edge::Top);
            }
        }

        if (point.x, point.y) == (2, 3) {
            if let Some(level_below) = self.levels.get(&(level_index + 1)) {
                bugs_count += level_below.bugs_on_edge(Edge::Bottom);
            }
        }

        bugs_count
    }

    pub fn pass_a_minute(&mut self) {
        let mut new_levels = HashMap::new();

        let min_level = *self.levels.keys().min().unwrap();
        let max_level = *self.levels.keys().max().unwrap();

        self.levels.insert(min_level - 1, Level::default());
        self.levels.insert(max_level + 1, Level::default());

        for (i, level) in self.levels.iter() {
            let mut new_area = HashMap::new();

            for (p, ct) in level.area.iter() {
                match *ct {
                    CellType::Bug => {
                        let count = self.count_bugs_around(p, *i);
                        let dies = count != 1;

                        if dies {
                            new_area.insert(*p, CellType::Empty);
                        } else {
                            new_area.insert(*p, CellType::Bug);
                        }
                    }
                    CellType::Empty => {
                        let count = self.count_bugs_around(p, *i);
                        let becomes_infested = (count == 1) || (count == 2);

                        if becomes_infested {
                            new_area.insert(*p, CellType::Bug);
                        } else {
                            new_area.insert(*p, CellType::Empty);
                        }
                    }
                    CellType::LevelBelow => {
                        new_area.insert(*p, CellType::LevelBelow);
                    }
                }
            }

            new_levels.insert(*i, Level { area: new_area });
        }

        self.levels = new_levels;
    }
}

impl Display for Eris {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut display = String::new();
        let mut sorted_levels = self.levels.keys().collect::<Vec<&i32>>();
        sorted_levels.sort_by_key(|x| **x);

        for i in sorted_levels.iter() {
            display.push_str("Depth: ");
            display.push_str(&i.to_string());
            display.push_str(":\n");
            display.push_str(&self.levels.get(*i).unwrap().to_string());
            display.push_str(":\n");
        }

        write!(f, "{}", display)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Level {
    pub area: HashMap<Point, CellType>,
}

impl Default for Level {
    fn default() -> Self {
        let mut area = HashMap::new();

        for y in 0..5 {
            for x in 0..5 {
                area.insert(Point { x, y }, CellType::default());
            }
        }

        area.insert(Point { x: 2, y: 2 }, CellType::LevelBelow);

        Level { area }
    }
}

impl Level {
    pub fn init(s: &str) -> Level {
        let mut area = HashMap::new();

        let lines = s.lines();

        for (y, line) in lines.enumerate() {
            let chars = line.chars();

            for (x, c) in chars.enumerate() {
                let cell_type = match c {
                    '.' => CellType::Empty,
                    '#' => CellType::Bug,
                    '?' => CellType::LevelBelow,
                    _ => panic!("{} is a wrong cell type", c),
                };

                area.insert(Point { x, y }, cell_type);
            }
        }

        area.insert(Point { x: 2, y: 2 }, CellType::LevelBelow);

        Level { area }
    }

    pub fn biodiversity_rating(&self) -> usize {
        self.area
            .iter()
            .filter(|&(_, ct)| *ct == CellType::Bug)
            .map(|(&p, _)| 2_usize.pow((p.x + p.y * 5) as u32))
            .sum()
    }

    pub fn bugs_on_edge(&self, edge: Edge) -> usize {
        self.area
            .iter()
            .filter(|(&point, _)| match edge {
                Edge::Top => point.y == 0,
                Edge::Bottom => point.y == 4,
                Edge::Left => point.x == 0,
                Edge::Right => point.x == 4,
            })
            .filter(|(_, &ct)| ct == CellType::Bug)
            .count()
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut display = String::new();

        for y in 0..5 {
            for x in 0..5 {
                let cell_type = self.area.get(&Point { x, y }).unwrap();

                let c = match *cell_type {
                    CellType::Empty => '.',
                    CellType::Bug => '#',
                    CellType::LevelBelow => '?',
                };

                display.push(c);
            }

            if y != 4 {
                display.push('\n');
            }
        }

        write!(f, "{}", display)
    }
}
