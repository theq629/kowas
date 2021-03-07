use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::tilemap::TileMap;
use crate::game::state::GameState;
use crate::game::stuff::Stuff;
use crate::game::directions::Direction;
use crate::game::components::Position;
use super::change::{ChangeResult, ChangeOk};

pub fn move_stuff(from: Point, to: Point, stuff: &mut TileMap<Stuff>) {
    let stuff_at_to = stuff[to].clone();
    stuff[to] = stuff[from];
    stuff[from] = stuff_at_to;
}

pub fn move_entity(entity: Entity, dir: Direction, state: &mut GameState) -> ChangeResult {
    let mut pos = state.world.get_mut::<Position>(entity)?;
    let new_pos = pos.0 + dir.to_point();
    if !state.stuff[new_pos].is_solid() {
        move_stuff(pos.0, new_pos, &mut state.stuff);
        pos.0 = new_pos;
    }
    Ok(ChangeOk)
}
