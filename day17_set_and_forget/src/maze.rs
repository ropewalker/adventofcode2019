use std::ops::Neg;

//#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
//pub enum CellType {
//    Wall = 0,
//    Empty = 1,
//    OxygenSystem = 2,
//    Unknown = -1,
//}
//
//impl From<isize> for CellType {
//    fn from(status: isize) -> CellType {
//        match status {
//            0 => CellType::Wall,
//            1 => CellType::Empty,
//            2 => CellType::OxygenSystem,
//            _ => panic!("Not a valid cell type!"),
//        }
//    }
//}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
pub enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl From<isize> for Direction {
    fn from(direction: isize) -> Direction {
        match direction {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("Not a valid direction!"),
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

pub fn find_neighbour_cell(position: (isize, isize), direction: Direction) -> (isize, isize) {
    match direction {
        Direction::North => (position.0, position.1 - 1),
        Direction::South => (position.0, position.1 + 1),
        Direction::West => (position.0 - 1, position.1),
        Direction::East => (position.0 + 1, position.1),
    }
}

pub fn turn_left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
        Direction::East => Direction::North,
    }
}

pub fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::East => Direction::South,
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
pub struct Node {
    pub coordinates: (isize, isize),
    pub distance: isize,
}
