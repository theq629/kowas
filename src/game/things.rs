use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::game::graphics::Graphic;
use crate::game::state::GameState;
use crate::game::components;

pub fn player(pos: Point, state: &mut GameState) -> Entity {
    state.world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Player),
        components::Inventory(Vec::new())
    ))
}

pub fn water_potion(pos: Point, state: &mut GameState) -> Entity {
    state.world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Bomb),
        components::Bomb
    ))
}
