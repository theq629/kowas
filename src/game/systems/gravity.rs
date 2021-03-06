use bracket_geometry::prelude::Point;
use crate::game::state::GameState;
use crate::game::components::Position;

pub fn gravity(state: &mut GameState) {
    for (_, (mut pos,)) in state.world.query::<(&mut Position,)>().iter() {
        if !state.terrain[pos.0 + Point::new(0, 1)].is_solid() {
            pos.0.y += 1;
        }
    }
}
