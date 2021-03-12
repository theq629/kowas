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
    pub fn to_point(&self) -> Point{
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
}
