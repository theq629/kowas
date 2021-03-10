use enum_map::Enum;
use serde::{Serialize, Deserialize};
pub use super::graphics::Graphic;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum Terrain {
    Floor,
    FloorUnderRoof,
    DamagedFloor,
    DamagedFloorUnderRoof,
    Wall
}

impl Terrain {
    pub fn is_solid(self) -> bool {
        match self {
            Terrain::Wall => true,
            _ => false
        }
    }

    pub fn graphic(self) -> Graphic {
        match self {
            Terrain::Floor => Graphic::Floor,
            Terrain::FloorUnderRoof => Graphic::FloorUnderRoof,
            Terrain::DamagedFloor => Graphic::DamagedFloor,
            Terrain::DamagedFloorUnderRoof => Graphic::DamagedFloorUnderRoof,
            Terrain::Wall => Graphic::Wall
        }
    }

    pub fn damaged(self) -> Terrain {
        match self {
            Terrain::Floor => Terrain::DamagedFloor,
            Terrain::FloorUnderRoof => Terrain::DamagedFloorUnderRoof,
            t => t
        }
    }
}
