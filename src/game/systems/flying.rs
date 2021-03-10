use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::log_err::result_error;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::{Position, Flying, Blocks};
use super::change::{ChangeResult, ChangeOk, ChangeErr};

fn shove(_shover: Entity, shovee: Entity, dir: Point, state: &mut GameState) -> ChangeResult {
    let _ = state.world.insert_one(shovee, Flying { velocity: Point::zero() });
    let mut shovee_flying = state.world.get_mut::<Flying>(shovee)?;
    shovee_flying.velocity = dir;
    Ok(ChangeOk)
}

pub fn shove_toward(shover: Entity, dir: Direction, state: &mut GameState) -> ChangeResult {
    let pos = state.world.get::<Position>(shover)?.0.clone();
    let target_pos = pos + dir.to_point();

    let targets: Vec<_> = state.world.query::<(&Position,)>()
        .iter()
        .filter(|(_, (p,))| p.0 == target_pos)
        .map(|(e, _)| e)
        .collect();
    for target in targets {
        if state.world.get::<Blocks>(target).is_ok() {
            return shove(shover, target, dir.to_point(), state);
        }
    }

    Err(ChangeErr)
}

pub fn update_flying(state: &mut GameState) {
    let mut to_defly = Vec::new();
    for (entity, (mut pos, mut flying)) in state.world.query::<(&mut Position, &mut Flying)>().iter() {
        pos.0 = pos.0 + flying.velocity;
        flying.velocity = flying.velocity * 0.5;
        if flying.velocity.x < 1 && flying.velocity.y < 1 {
            to_defly.push(entity);
        }
    }
    for entity in to_defly.iter() {
        result_error(state.world.remove_one::<Flying>(*entity));
    }
}
