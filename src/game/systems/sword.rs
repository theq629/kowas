use std::collections::HashSet;
use bracket_geometry::prelude::{Point, VectorLine};
use hecs::Entity;
use crate::game::state::GameState;
use crate::game::graphics::Graphic;
use crate::game::directions::Direction;
use crate::game::components::{Position, Power};
use super::change::{ChangeResult, ChangeOk};
use super::damage::melee_damage;
use super::flying::impact_shove;
use super::structures::impact;
use super::particles::make_particle;

fn is_clear(src: Point, dst: Point, state: &GameState) -> bool {
    for pos in VectorLine::new(src, dst).skip(1) {
        if state.terrain[pos].is_solid() {
            return false;
        }
    }
    true
}

fn do_slash(pos: Point, dir: Direction, power: i32, state: &mut GameState) -> ChangeResult {
    let end_pos = pos + dir.to_point() * 2 * power;
    let mut damage = 2 * power;
    for pos in VectorLine::new(pos, end_pos).skip(1) {
        if !state.terrain.is_valid(pos) {
            break;
        }
        make_particle(pos, Graphic::DamageEffect, state);
        let _ = melee_damage(pos, damage, state);
        impact_shove(pos, dir.to_point() * damage / 2, state);
        state.terrain[pos] = state.terrain[pos].damaged();
        if let Ok(_) = impact(pos, dir.to_point() * damage, state) {
            damage -= 3;
            if state.terrain[pos].is_solid() {
                break;
            }
        }
        damage -= 1;
        if damage <= 0 {
            break;
        }
    }

    Ok(ChangeOk)
}

pub fn slash_toward(attacker: Entity, dir: Direction, state: &mut GameState) -> ChangeResult {
    let attacker_pos = state.world.get::<Position>(attacker)?.0;
    let attacker_power = state.world.get::<Power>(attacker)?.0;
    do_slash(attacker_pos, dir, attacker_power, state)?;
    Ok(ChangeOk)
}

pub fn do_whirl(centre: Point, power: i32, state: &mut GameState) -> ChangeResult {
    let radius = 1 + power / 4;
    let damage = power;

    let radius2 = radius * radius;
    for dx in (-radius)..(radius + 1) {
        for dy in (-radius)..(radius + 1) {
            let r2 = dx * dx + dy * dy;
            if r2 > 0 && r2 <= radius2 {
                let pos = centre + Point::new(dx, dy);
                if state.terrain.is_valid(pos) && is_clear(centre, pos, state) {
                    make_particle(pos, Graphic::DamageEffect, state);
                    let _ = melee_damage(pos, damage, state);
                    impact_shove(pos, (pos - centre) * damage / 2, state);
                }
            }
        }
    }

    Ok(ChangeOk)
}

pub fn whirl(attacker: Entity, state: &mut GameState) -> ChangeResult {
    let attacker_pos = state.world.get::<Position>(attacker)?.0;
    let attacker_power = state.world.get::<Power>(attacker)?.0;
    do_whirl(attacker_pos, attacker_power, state)?;
    Ok(ChangeOk)
}

fn do_flurry(centre: Point, dir: Direction, power: i32, state: &mut GameState) -> ChangeResult {
    let half_width = 2;
    let depth = 1 + power / 4;
    let damage = power;
    let mut touched = HashSet::new();
    let offsets =
        if dir.is_cardinal () {
            let mut offsets = vec![(Point::zero(), half_width, half_width)];
            for _ in 0..depth {
                offsets.push((dir.to_point(), half_width, half_width));
            }
            offsets
        } else {
            let mut offsets = vec![(Point::zero(), half_width, half_width)];
            for _ in 0..(depth / 2) {
                offsets.push((dir.clockwise().to_point(), half_width - 1, half_width));
                offsets.push((dir.counterclockwise().to_point(), half_width, half_width));
            }
            offsets
        };
    let mut pos0 = centre;
    for (offset, d1, d2) in offsets.iter() {
        pos0 = pos0 + *offset;
        let pos1 = pos0 + dir.clockwise().clockwise().to_point() * *d1;
        let pos2 = pos0 + dir.clockwise().clockwise().to_point() * -*d2;
        for pos in VectorLine::new(pos1, pos2) {
            if pos != centre && state.terrain.is_valid(pos) && !touched.contains(&pos) && is_clear(centre, pos, state) {
                make_particle(pos, Graphic::DamageEffect, state);
                let _ = melee_damage(pos, damage, state);
                impact_shove(pos, (pos - centre) * damage, state);
                touched.insert(pos);
            }
        }
    }
    Ok(ChangeOk)
}

pub fn flurry_toward(attacker: Entity, dir: Direction, state: &mut GameState) -> ChangeResult {
    let attacker_pos = state.world.get::<Position>(attacker)?.0;
    let attacker_power = state.world.get::<Power>(attacker)?.0;
    do_flurry(attacker_pos, dir, attacker_power, state)?;
    Ok(ChangeOk)
}
