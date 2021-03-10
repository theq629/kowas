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
    pub fn name(self) -> String {
        match self {
            Terrain::Floor => "floor".to_string(),
            Terrain::FloorUnderRoof => "floor under a roof".to_string(),
            Terrain::DamagedFloor => "damaged floor".to_string(),
            Terrain::DamagedFloorUnderRoof => "damaged floor under a roof".to_string(),
            Terrain::Wall => "wall".to_string()
        }
    }

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
