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

impl Action {
    pub fn needs_energy(self) -> i32 {
        match self {
            Action::Shove(_) => 1,
            Action::SwordSlash(_) => 3,
            Action::SwordWhirl => 5,
            Action::SwordFlurry(_) => 2,
            _ => 0
        }
    }
}
