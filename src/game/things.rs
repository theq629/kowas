use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::game::stuff::Stuff;
use crate::game::state::GameState;
use crate::game::components;

pub fn player(pos: Point, state: &mut GameState) -> Entity {
    state.stuff[pos] = Stuff::Body;
    state.world.spawn((
        components::Position(pos),
        components::Inventory(Vec::new())
    ))
}

pub fn water_potion(pos: Point, state: &mut GameState) -> Entity {
    state.stuff[pos] = Stuff::Bomb;
    state.world.spawn((
        components::Position(pos),
        components::Bomb,
        components::ContainsStuff {
            stuff: Stuff::Water,
            amount: 10
        }
    ))
}
