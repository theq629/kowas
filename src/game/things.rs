use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::game::graphics::Graphic;
use crate::game::state::GameState;
use crate::game::components;

pub fn player(pos: Point, state: &mut GameState) -> Entity {
    state.world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Player),
        components::Blocks,
        components::Health::new(20),
        components::Inventory(Vec::new())
    ))
}

pub fn goblin(pos: Point, state: &mut GameState) -> Entity {
    state.world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Goblin),
        components::Blocks,
        components::Health::new(10)
    ))
}

pub fn orc(pos: Point, state: &mut GameState) -> Entity {
    state.world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Orc),
        components::Blocks,
        components::Health::new(10)
    ))
}
