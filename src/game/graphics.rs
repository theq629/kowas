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
    Gore,
    Goblin,
    Orc,
    DamageEffect
}

impl Graphic {
    pub fn name(self) -> String {
        match self {
            Graphic::Player => "player".to_string(),
            Graphic::Floor => "floor".to_string(),
            Graphic::FloorUnderRoof => "floor under roof".to_string(),
            Graphic::DamagedFloor => "damaged floor".to_string(),
            Graphic::DamagedFloorUnderRoof => "damaged floor under roof".to_string(),
            Graphic::Wall => "wall".to_string(),
            Graphic::Gore => "gore".to_string(),
            Graphic::Goblin => "goblin".to_string(),
            Graphic::Orc => "orc".to_string(),
            Graphic::DamageEffect => "damage effect".to_string()
        }
    }
}
