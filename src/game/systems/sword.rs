use hecs::Entity;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::{Position, Power};
use super::change::{ChangeResult, ChangeOk};
use super::damage::slash_damage;

pub fn slash_toward(attacker: Entity, dir: Direction, state: &mut GameState) -> ChangeResult {
    let attacker_pos = state.world.get::<Position>(attacker)?.0;
    let attacker_power = state.world.get::<Power>(attacker)?.0;
    slash_damage(attacker_pos, dir, attacker_power, state)?;
    Ok(ChangeOk)
}
