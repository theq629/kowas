use std::cmp::max;
use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::game::state::GameState;
use crate::game::graphics::Graphic;
use crate::game::components::{Health, Position};
use super::change::{ChangeResult, ChangeOk, ChangeErr};
use super::particles::make_particle;
use super::splatter::splatter_blood;

pub fn melee_damage(pos: Point, value: i32, state: &mut GameState) -> ChangeResult {
    let mut hit_something = false;
    for (_, (_, mut health)) in state.world.query::<(&Position, &mut Health)>().iter()
        .filter(|(_, (p, _))| p.0 == pos)
    {
        health.value -= value;
        hit_something = true;
    }
    if hit_something {
        make_particle(pos, Graphic::DamageEffect, state);
        splatter_blood(pos, 10, state);
        Ok(ChangeOk)
    } else {
        Err(ChangeErr)
    }
}

pub fn terrain_collision_damage(collider: Entity, velocity: i32, state: &mut GameState) -> ChangeResult {
    let health_loss = max(0, velocity - 1);
    {
        if let Ok(mut collider_health) = state.world.get_mut::<Health>(collider) {
            collider_health.value -= health_loss;
        }
    }

    {
        let collider_pos = state.world.get::<Position>(collider)?.0;
        if velocity > 8 {
            state.terrain[collider_pos] = state.terrain[collider_pos].damaged();
        }
        make_particle(collider_pos, Graphic::DamageEffect, state);
    }

    Ok(ChangeOk)
}

pub fn collision_damage(collider: Entity, pos: Point, velocity: i32, state: &mut GameState) -> ChangeResult {
    let health_loss = max(0, velocity - 1);

    {
        if let Ok(mut collider_health) = state.world.get_mut::<Health>(collider) {
            collider_health.value -= health_loss;
        }
    }

    for (_, (_, mut collidee_health)) in state.world.query::<(&Position, &mut Health)>().iter()
        .filter(|(_, (p, _))| p.0 == pos)
    {
        collidee_health.value -= health_loss;
    }

    {
        let collider_pos = state.world.get::<Position>(collider)?.0;
        if velocity > 8 {
            state.terrain[collider_pos] = state.terrain[collider_pos].damaged();
            state.terrain[pos] = state.terrain[pos].damaged();
        }
        make_particle(collider_pos, Graphic::DamageEffect, state);
        make_particle(pos, Graphic::DamageEffect, state);
    }

    Ok(ChangeOk)
}
