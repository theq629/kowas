use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::graphics::Graphic;
use crate::game::components::{Position, Power, Blocks, Health};
use super::change::{ChangeResult, ChangeOk, ChangeErr};
use super::flying::impact_shove;
use super::particles::make_particle;
use super::flying::shove;

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
            make_particle(target_pos, Graphic::ShoveEffect, state);
            return shove(shover, target, dir.to_point(), state);
        }
    }

    Err(ChangeErr)
}

pub fn do_throw_off(centre: Point, power: i32, state: &mut GameState) -> ChangeResult {
    let radius = 1 + power / 4;
    let damage = power;

    let radius2 = radius * radius;
    for dx in (-radius)..(radius + 1) {
        for dy in (-radius)..(radius + 1) {
            let r2 = dx * dx + dy * dy;
            if r2 > 0 && r2 <= radius2 {
                let pos = centre + Point::new(dx, dy);
                if state.terrain.is_valid(pos) {
                    make_particle(pos, Graphic::ShoveEffect, state);
                    impact_shove(pos, (pos - centre) * damage / 2, state);
                }
            }
        }
    }

    Ok(ChangeOk)
}

pub fn throw_off(attacker: Entity, state: &mut GameState) -> ChangeResult {
    let attacker_pos = state.world.get::<Position>(attacker)?.0;
    let attacker_power = state.world.get::<Power>(attacker)?.0;
    do_throw_off(attacker_pos, attacker_power, state)?;
    Ok(ChangeOk)
}

pub fn heal(healee: Entity, state: &mut GameState) -> ChangeResult {
    let mut healee_health = state.world.get_mut::<Health>(healee)?;
    healee_health.value += 10;
    Ok(ChangeOk)
}
