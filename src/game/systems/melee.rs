use hecs::Entity;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::Position;
use super::change::{ChangeResult, ChangeOk};
use super::damage::melee_damage;

pub fn melee_attack_toward(attacker: Entity, direction: Direction, state: &mut GameState) -> ChangeResult {
    let pos = state.world.get::<Position>(attacker)?.0.clone();
    let target_pos = pos + direction.to_point();
    melee_damage(target_pos, 1, state)?;
    Ok(ChangeOk)
}
