use serde::{Serialize, Deserialize};
use enum_map::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Enum, Serialize, Deserialize)]
pub enum Graphic {
    Player,
    Floor,
    FloorUnderRoof,
    DamagedFloor,
    DamagedFloorUnderRoof,
    Wall,
    BoundaryWall,
    Gore,
    Goblin,
    Orc,
    DamageEffect
}

impl Graphic {
    pub fn name(self) -> &'static str {
        match self {
            Graphic::Player => "player",
            Graphic::Floor => "floor",
            Graphic::FloorUnderRoof => "floor under roof",
            Graphic::DamagedFloor => "damaged floor",
            Graphic::DamagedFloorUnderRoof => "damaged floor under roof",
            Graphic::Wall => "wall",
            Graphic::BoundaryWall => "boundary wall",
            Graphic::Gore => "gore",
            Graphic::Goblin => "goblin",
            Graphic::Orc => "orc",
            Graphic::DamageEffect => "damage effect"
        }
    }
}
