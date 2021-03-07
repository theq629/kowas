use hecs::Entity;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::Position;
use super::change::{ChangeResult, ChangeOk, ChangeErr};

pub fn move_entity(entity: Entity, dir: Direction, state: &mut GameState) -> ChangeResult {
    let mut pos = state.world.get_mut::<Position>(entity)?;
    let new_pos = pos.0 + dir.to_point();

    if state.terrain[new_pos].is_solid() {
        Err(ChangeErr)
    } else {
        pos.0 = new_pos;
        Ok(ChangeOk)
    }
}
