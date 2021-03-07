use enum_map::Enum;
use serde::{Serialize, Deserialize};
pub use super::graphics::Graphic;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum Stuff {
    Air,
    Floor,
    Body
}

impl Stuff {
    pub fn is_solid(self) -> bool {
        match self {
            Stuff::Air => false,
            Stuff::Floor => true,
            Stuff::Body => false,
        }
    }

    pub fn graphic(self) -> Graphic {
        match self {
            Stuff::Air => Graphic::Empty,
            Stuff::Floor => Graphic::Floor,
            Stuff::Body => Graphic::Body,
        }
    }
}
