use serde::{Serialize, Deserialize};
use enum_map::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Enum, Serialize, Deserialize)]
pub enum Graphic {
    Player,
    Floor,
    DamagedFloor,
    Wall,
    Rubble,
    BoundaryWall,
    Gore,
    Goblin,
    Orc,
    BigOrc,
    OrcLord,
    ShoveEffect,
    DamageEffect
}

impl Graphic {
    pub fn name(self) -> &'static str {
        match self {
            Graphic::Player => "player",
            Graphic::Floor => "floor",
            Graphic::DamagedFloor => "damaged floor",
            Graphic::Wall => "wall",
            Graphic::Rubble => "rubble",
            Graphic::BoundaryWall => "boundary wall",
            Graphic::Gore => "gore",
            Graphic::Goblin => "goblin",
            Graphic::Orc => "orc",
            Graphic::BigOrc => "big orc",
            Graphic::OrcLord => "orc lord",
            Graphic::ShoveEffect => "shove effect",
            Graphic::DamageEffect => "damage effect"
        }
    }
}
