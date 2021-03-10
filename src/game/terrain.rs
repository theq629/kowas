use enum_map::Enum;
use serde::{Serialize, Deserialize};
pub use super::graphics::Graphic;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum Terrain {
    Floor,
    FloorUnderRoof,
    Wall
}

impl Terrain {
    pub fn is_solid(self) -> bool {
        match self {
            Terrain::Floor => false,
            Terrain::FloorUnderRoof => false,
            Terrain::Wall => true
        }
    }

    pub fn graphic(self) -> Graphic {
        match self {
            Terrain::Floor => Graphic::Floor,
            Terrain::FloorUnderRoof => Graphic::FloorUnderRoof,
            Terrain::Wall => Graphic::Wall
        }
    }
}
