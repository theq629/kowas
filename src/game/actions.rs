use serde::{Serialize, Deserialize};
use crate::game::directions::Direction;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    DoNothing,
    Move(Direction),
    MeleeAttack(Direction),
    Shove(Direction),
    ThrowOff,
    Heal,
    SwordSlash(Direction),
    SwordWhirl,
    SwordFlurry(Direction),
    GetALotOfEnergy,
    GetALotOfHealth,
    GainPower
}

impl Action {
    pub fn needs_energy(self) -> i32 {
        match self {
            Action::Shove(_) => 2,
            Action::ThrowOff => 6,
            Action::Heal => 20,
            Action::SwordSlash(_) => 6,
            Action::SwordWhirl => 10,
            Action::SwordFlurry(_) => 4,
            _ => 0
        }
    }
}
