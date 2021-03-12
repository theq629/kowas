use bracket_geometry::prelude::{Point, VectorLine};
use hecs::Entity;
use crate::log_err::result_error;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::{Position, Flying, Blocks, Power};
use super::change::{ChangeResult, ChangeOk, ChangeErr};
use super::damage::{collision_damage, terrain_collision_damage};
use super::structures::impact;

fn shove(shover: Entity, shovee: Entity, dir: Point, state: &mut GameState) -> ChangeResult {
    let shover_power = state.world.get::<Power>(shover)?.0;
    let _ = state.world.insert_one(shovee, Flying { velocity: Point::zero() });
    let mut shovee_flying = state.world.get_mut::<Flying>(shovee)?;
    shovee_flying.velocity = dir * shover_power;
    Ok(ChangeOk)
}

fn rotate_vec(vec: Point, angle: f32) -> Point {
    let x = vec.x as f32;
    let y = vec.y as f32;
    Point::new(
        (x * f32::cos(angle) - y * f32::sin(angle)).round() as i32,
        (x * f32::sin(angle) + y * f32::cos(angle)).round() as i32
    )
}

pub fn impact_shove(shovee: Entity, vel: Point, state: &mut GameState) {
    let new_vel =
        if state.rng.range(0, 100) < 50 {
            vel
        } else {
            let angle = state.rng.range(-90, 90);
            let angle = angle as f32 * (std::f32::consts::PI / 180.);
            rotate_vec(vel, angle)
        };
    let _ = state.world.insert_one(shovee, Flying { velocity: new_vel });
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
    let vel_mag = ((vel.x * vel.x + vel.y * vel.y) as f32).sqrt() as i32;
    let new_pos = cur_pos + vel;
    let mut last_ok_pos = cur_pos;
    let mut collision = None;
    let mut remaining_dist = vel_mag;
    'posloop: for pos in VectorLine::new(cur_pos, new_pos) {
        if remaining_dist < 0 {
            break 'posloop;
        }
        if state.terrain[pos].is_solid() {
            result_error(impact(pos, vel, state));
            result_error(terrain_collision_damage(entity, vel_mag, state));
            if state.terrain[pos].is_solid() {
                break 'posloop;
            }
            remaining_dist /= 2;
        }
        for (entity, _) in state.world.query::<(&Position, &Blocks)>()
            .iter()
            .filter(|(e, (p, _))| *e != entity && p.0 == pos)
        {
            collision = Some(entity);
            break 'posloop;
        }
        last_ok_pos = pos;
        remaining_dist -= 1;
    }
    if let Ok(mut entity_pos) = state.world.get_mut::<Position>(entity) {
        entity_pos.0 = last_ok_pos;
    }
    if let Some(collision) = collision {
        let _ = collision_damage(entity, collision, vel_mag, state);
        impact_shove(collision, vel / 2, state);
    }
}

pub fn update_flying(state: &mut GameState) {
    loop {
        let flying: Vec<_> = state.world.query::<(&Position, &Flying)>()
            .iter()
            .map(|(e, (p, f))| (e, p.0, f.velocity))
            .collect();
        let done: Vec<_> = state.world.query::<&Flying>().iter().map(|(e, _)| e).collect();
        if flying.is_empty() {
            break;
        }
        for (entity, cur_pos, vel) in flying {
            move_flying(entity, cur_pos, vel, state);
        }
        for entity in done {
            result_error(state.world.remove_one::<Flying>(entity));
        }
    }
}
