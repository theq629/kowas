use hecs::Entity;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::{Position, Health};
use super::change::{ChangeResult, ChangeErr};
use super::damage::melee_damage;

pub fn melee_attack_toward(attacker: Entity, direction: Direction, state: &mut GameState) -> ChangeResult {
    let pos = state.world.get::<Position>(attacker)?.0.clone();
    let target_pos = pos + direction.to_point();

    let targets: Vec<_> = state.world.query::<(&Position,)>()
        .iter()
        .filter(|(_, (p,))| p.0 == target_pos)
        .map(|(e, _)| e)
        .collect();
    for target in targets {
        if state.world.get::<Health>(target).is_ok() {
            return melee_damage(attacker, target, state);
        }
    }

    Err(ChangeErr)
}
