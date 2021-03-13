use bracket_geometry::prelude::Point;
use crate::game::state::GameState;
use super::change::{ChangeResult, ChangeOk, ChangeErr};

fn destroy(pos: Point, state: &mut GameState) {
    state.terrain[pos] = state.terrain[pos].wrecked();
}

fn destroy_radius(centre: Point, radius: i32, state: &mut GameState) {
    for dx in (-radius)..(radius + 1) {
        for dy in (-radius)..(radius + 1) {
            let pos = Point::new(centre.x + dx, centre.y + dy);
            if state.terrain.is_valid(pos) {
                destroy(pos, state);
            }
        }
    }
}

pub fn impact(pos: Point, vel: Point, state: &mut GameState) -> ChangeResult {
    if !state.terrain[pos].is_solid() {
        return Err(ChangeErr);
    }
    let vel_mag = f32::sqrt((vel.x * vel.x + vel.y * vel.y) as f32);
    let radius = f32::log2(f32::max(1.0, vel_mag - 8 as f32)).round() as i32;
    destroy_radius(pos, radius, state);
    Ok(ChangeOk)
}
