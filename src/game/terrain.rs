use serde::{Serialize, Deserialize};
pub use super::graphics::Graphic;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Terrain {
    Empty,
    Floor
}

impl Terrain {
    pub fn graphic(&self) -> Graphic {
        match self {
            Terrain::Empty => Graphic::Empty,
            Terrain::Floor => Graphic::Floor
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            Terrain::Empty => false,
            Terrain::Floor => true
        }
    }
}
