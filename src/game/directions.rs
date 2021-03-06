use serde::{Serialize, Deserialize};
use bracket_geometry::prelude::Point;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW
}

impl Direction {
    pub const ALL: [Self; 8] = [
        Direction::N,
        Direction::S,
        Direction::E,
        Direction::W,
        Direction::NE,
        Direction::NW,
        Direction::SE,
        Direction::SW
    ];

    pub fn is_cardinal(&self) -> bool {
        match self {
            Direction::N => true,
            Direction::S => true,
            Direction::E => true,
            Direction::W => true,
            _ => false
        }
    }

    pub fn to_point(&self) -> Point {
        match self {
            Direction::N => Point::new(0, -1),
            Direction::S => Point::new(0, 1),
            Direction::E => Point::new(1, 0),
            Direction::W => Point::new(-1, 0),
            Direction::NE => Point::new(1, -1),
            Direction::NW => Point::new(-1, -1),
            Direction::SE => Point::new(1, 1),
            Direction::SW => Point::new(-1, 1)
        }
    }

    pub fn of_point(point: Point) -> Option<Self> {
        match (point.x, point.y) {
            (0, -1) => Some(Direction::N),
            (0, 1) => Some(Direction::S),
            (1, 0) => Some(Direction::E),
            (-1, 0) => Some(Direction::W),
            (1, -1) => Some(Direction::NE),
            (-1, -1) => Some(Direction::NW),
            (1, 1) => Some(Direction::SE),
            (-1, 1) => Some(Direction::SW),
            _ => None
        }
    }

    pub fn perpendicular(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
            Direction::W => Direction::N,
            Direction::NE => Direction::NW,
            Direction::NW => Direction::NE,
            Direction::SE => Direction::SW,
            Direction::SW => Direction::SE
        }
    }

    pub fn clockwise(self) -> Direction {
        match self {
            Direction::N => Direction::NE,
            Direction::S => Direction::SW,
            Direction::E => Direction::SE,
            Direction::W => Direction::NW,
            Direction::NE => Direction::E,
            Direction::NW => Direction::N,
            Direction::SE => Direction::S,
            Direction::SW => Direction::W
        }
    }

    pub fn counterclockwise(self) -> Direction {
        match self {
            Direction::N => Direction::NW,
            Direction::S => Direction::SE,
            Direction::E => Direction::NE,
            Direction::W => Direction::SW,
            Direction::NE => Direction::N,
            Direction::NW => Direction::W,
            Direction::SE => Direction::E,
            Direction::SW => Direction::S
        }
    }
}
