use bracket_geometry::prelude::Point;
use crate::game::terrain::Terrain;
use crate::game::state::GameState;
use super::change::{ChangeResult, ChangeOk, ChangeErr};

fn destroy(pos: Point, state: &mut GameState) {
    state.terrain[pos] = Terrain::Rubble;
}

pub fn impact(pos: Point, vel: Point, state: &mut GameState) -> ChangeResult {
    if !state.terrain[pos].is_solid() {
        return Err(ChangeErr);
    }
    if vel.x * vel.x + vel.y * vel.y > 8 {
        destroy(pos, state);
    }
    Ok(ChangeOk)
}
