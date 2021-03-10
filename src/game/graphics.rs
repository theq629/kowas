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
