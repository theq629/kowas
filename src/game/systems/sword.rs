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

fn slash(pos: Point, dir: Direction, power: i32, state: &mut GameState) -> ChangeResult {
    let end_pos = pos + dir.to_point() * 2 * power;
    let mut damage = 2 * power;
    for pos in VectorLine::new(pos, end_pos).skip(1) {
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
    slash(attacker_pos, dir, attacker_power, state)?;
    Ok(ChangeOk)
}
