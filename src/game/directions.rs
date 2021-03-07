use serde::{Serialize, Deserialize};
use bracket_geometry::prelude::Point;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}

impl Direction {
    pub fn to_point(&self) -> Point{
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Right => Point::new(1, 0),
            Direction::Left => Point::new(-1, 0),
            Direction::UpRight => Point::new(1, -1),
            Direction::UpLeft => Point::new(-1, -1),
            Direction::DownRight => Point::new(1, 1),
            Direction::DownLeft => Point::new(-1, 1)
        }
    }
}
