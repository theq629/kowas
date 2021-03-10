use std::cmp::max;
use hecs::Entity;
use crate::game::state::GameState;
use crate::game::graphics::Graphic;
use crate::game::components::{Health, Position};
use super::change::{ChangeResult, ChangeOk};
use super::particles::make_particle;

pub fn melee_damage(_attacker: Entity, attackee: Entity, state: &mut GameState) -> ChangeResult {
    let attackee_pos = state.world.get::<Position>(attackee)?.0;
    make_particle(attackee_pos, Graphic::DamageEffect, state);
    let mut attackee_health = state.world.get_mut::<Health>(attackee)?;
    attackee_health.value -= 1;
    Ok(ChangeOk)
}

pub fn collision_damage(collider: Entity, collidee: Entity, velocity: i32, state: &mut GameState) -> ChangeResult {
    {
        let health_loss = max(0, velocity - 1);
        let mut collider_health = state.world.get_mut::<Health>(collider)?;
        let mut collidee_health = state.world.get_mut::<Health>(collidee)?;
        collider_health.value -= health_loss;
        collidee_health.value -= health_loss;
    }

    {
        let collider_pos = state.world.get::<Position>(collider)?.0;
        let collidee_pos = state.world.get::<Position>(collidee)?.0;
        make_particle(collider_pos, Graphic::DamageEffect, state);
        make_particle(collidee_pos, Graphic::DamageEffect, state);
    }

    Ok(ChangeOk)
}
