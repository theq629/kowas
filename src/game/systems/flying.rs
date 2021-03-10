use bracket_geometry::prelude::{Point, VectorLine};
use hecs::Entity;
use crate::log_err::result_error;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::{Position, Flying, Blocks, Power};
use super::change::{ChangeResult, ChangeOk, ChangeErr};
use super::damage::collision_damage;

fn shove(shover: Entity, shovee: Entity, dir: Point, state: &mut GameState) -> ChangeResult {
    let shover_power = state.world.get::<Power>(shover)?.0;
    let _ = state.world.insert_one(shovee, Flying { velocity: Point::zero() });
    let mut shovee_flying = state.world.get_mut::<Flying>(shovee)?;
    shovee_flying.velocity = dir * shover_power;
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

fn move_flying(entity: Entity, cur_pos: Point, vel: Point, state: &mut GameState) {
    let new_pos = cur_pos + vel;
    let mut last_ok_pos = cur_pos;
    let mut collision = None;
    'posloop: for pos in VectorLine::new(cur_pos, new_pos) {
        for (entity, _) in state.world.query::<(&Position, &Blocks)>()
            .iter()
            .filter(|(e, (p, _))| *e != entity && p.0 == pos)
        {
            collision = Some(entity);
            break 'posloop;
        }
        last_ok_pos = pos;
    }
    if let Ok(mut entity_pos) = state.world.get_mut::<Position>(entity) {
        entity_pos.0 = last_ok_pos;
    }
    if let Some(collision) = collision {
        let v = ((vel.x * vel.x + vel.y * vel.y) as f32).sqrt() as i32;
        let _ = collision_damage(entity, collision, v, state);
    }
}

pub fn update_flying(state: &mut GameState) {
    let flying: Vec<_> = state.world.query::<(&Position, &Flying)>()
        .iter()
        .map(|(e, (p, f))| (e, p.0, f.velocity))
        .collect();
    for (entity, cur_pos, vel) in flying {
        move_flying(entity, cur_pos, vel, state);
    }
    let flying: Vec<_> = state.world.query::<&Flying>().iter().map(|(e, _)| e).collect();
    for entity in flying {
        result_error(state.world.remove_one::<Flying>(entity));
    }
}
