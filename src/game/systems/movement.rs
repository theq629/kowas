use hecs::Entity;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::{Position, Blocks};
use super::change::{ChangeResult, ChangeOk, ChangeErr};

pub fn move_entity(entity: Entity, dir: Direction, state: &mut GameState) -> ChangeResult {
    let new_pos = {
        let pos = state.world.get::<Position>(entity)?;
        pos.0 + dir.to_point()
    };

    if state.terrain[new_pos].is_solid() {
        Err(ChangeErr)
    } else {
        let blocked = state.world.query::<(&Position, &Blocks)>()
            .iter()
            .any(|(_, (p, _))| p.0 == new_pos);
        if blocked {
            Err(ChangeErr)
        } else {
            let mut pos = state.world.get_mut::<Position>(entity)?;
            pos.0 = new_pos;
            Ok(ChangeOk)
        }
    }
}
