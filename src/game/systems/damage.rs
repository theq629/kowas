use std::cmp::max;
use bracket_geometry::prelude::{Point, VectorLine};
use hecs::Entity;
use crate::game::state::GameState;
use crate::game::graphics::Graphic;
use crate::game::directions::Direction;
use crate::game::components::{Health, Position};
use super::change::{ChangeResult, ChangeOk};
use super::particles::make_particle;
use super::splatter::splatter_blood;

pub fn melee_damage(_attacker: Entity, attackee: Entity, state: &mut GameState) -> ChangeResult {
    let attackee_pos = state.world.get::<Position>(attackee)?.0;
    splatter_blood(attackee_pos, 10, state);
    make_particle(attackee_pos, Graphic::DamageEffect, state);
    let mut attackee_health = state.world.get_mut::<Health>(attackee)?;
    attackee_health.value -= 1;
    Ok(ChangeOk)
}

pub fn collision_damage(collider: Entity, collidee: Entity, velocity: i32, state: &mut GameState) -> ChangeResult {
    let health_loss = max(0, velocity - 1);
    {
        if let Ok(mut collider_health) = state.world.get_mut::<Health>(collider) {
            collider_health.value -= health_loss;
        }
        if let Ok(mut collidee_health) = state.world.get_mut::<Health>(collidee) {
            collidee_health.value -= health_loss;
        }
    }

    {
        let collider_pos = state.world.get::<Position>(collider)?.0;
        let collidee_pos = state.world.get::<Position>(collidee)?.0;
        if velocity > 8 {
            state.terrain[collider_pos] = state.terrain[collider_pos].damaged();
            state.terrain[collidee_pos] = state.terrain[collidee_pos].damaged();
        }
        make_particle(collider_pos, Graphic::DamageEffect, state);
        make_particle(collidee_pos, Graphic::DamageEffect, state);
    }

    Ok(ChangeOk)
}

pub fn slash_damage(pos: Point, dir: Direction, power: i32, state: &mut GameState) -> ChangeResult {
    let end_pos = pos + dir.to_point() * 2 * power;
    for (i, pos) in VectorLine::new(pos, end_pos).skip(1).enumerate() {
        let damage = 2 * power - i as i32;
        make_particle(pos, Graphic::DamageEffect, state);
        for (_, (_, mut health)) in state.world.query::<(&Position, &mut Health)>().iter()
            .filter(|(_, (p, _))| p.0 == pos)
        {
            health.value -= damage;
        }
        if power > 2 {
            state.terrain[pos] = state.terrain[pos].damaged();
        }
    }

    Ok(ChangeOk)
}
