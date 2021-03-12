use serde::{Serialize, Deserialize};
use hecs::Entity;
use crate::game::directions::Direction;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    DoNothing,
    Move(Direction),
    MeleeAttack(Direction),
    Shove(Direction),
    SwordSlash(Direction),
    SwordWhirl,
    SwordFlurry(Direction),
    Get,
    Drop(Entity),
    GainPower
}
