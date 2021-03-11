use enum_map::Enum;
use serde::{Serialize, Deserialize};
pub use super::graphics::Graphic;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum Terrain {
    Floor,
    FloorUnderRoof,
    DamagedFloor,
    DamagedFloorUnderRoof,
    Wall,
    BoundaryWall
}

impl Terrain {
    pub fn name(self) -> &'static str {
        match self {
            Terrain::Floor => "floor",
            Terrain::FloorUnderRoof => "floor under a roof",
            Terrain::DamagedFloor => "damaged floor",
            Terrain::DamagedFloorUnderRoof => "damaged floor under a roof",
            Terrain::Wall => "wall",
            Terrain::BoundaryWall => "boundary wall"
        }
    }

    pub fn is_solid(self) -> bool {
        match self {
            Terrain::Wall => true,
            Terrain::BoundaryWall => true,
            _ => false
        }
    }

    pub fn graphic(self) -> Graphic {
        match self {
            Terrain::Floor => Graphic::Floor,
            Terrain::FloorUnderRoof => Graphic::FloorUnderRoof,
            Terrain::DamagedFloor => Graphic::DamagedFloor,
            Terrain::DamagedFloorUnderRoof => Graphic::DamagedFloorUnderRoof,
            Terrain::Wall => Graphic::Wall,
            Terrain::BoundaryWall => Graphic::BoundaryWall
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
