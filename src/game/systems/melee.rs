use hecs::Entity;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::{Position, Power};
use super::change::{ChangeResult, ChangeOk};
use super::damage::melee_damage;

pub fn melee_attack_toward(attacker: Entity, direction: Direction, state: &mut GameState) -> ChangeResult {
    let attacker_power = state.world.get::<Power>(attacker)?.0;
    let pos = state.world.get::<Position>(attacker)?.0.clone();
    let target_pos = pos + direction.to_point();
    melee_damage(target_pos, attacker_power, state)?;
    Ok(ChangeOk)
}
