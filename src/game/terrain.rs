use enum_map::Enum;
use serde::{Serialize, Deserialize};
pub use super::graphics::Graphic;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum Terrain {
    Floor,
    DamagedFloor,
    Rubble,
    Wall,
    BoundaryWall
}

impl Terrain {
    pub fn name(self) -> &'static str {
        match self {
            Terrain::Floor => "floor",
            Terrain::DamagedFloor => "damaged floor",
            Terrain::Rubble => "rubble",
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
            Terrain::DamagedFloor => Graphic::DamagedFloor,
            Terrain::Rubble => Graphic::Rubble,
            Terrain::Wall => Graphic::Wall,
            Terrain::BoundaryWall => Graphic::BoundaryWall
        }
    }

    pub fn damaged(self) -> Terrain {
        match self {
            Terrain::Floor => Terrain::DamagedFloor,
            t => t
        }
    }

    pub fn wrecked(self) -> Terrain {
        match self {
            Terrain::Wall => Terrain::Rubble,
            t => t.damaged()
        }
    }
}
