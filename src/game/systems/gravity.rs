use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::components::Position;
use super::movement::move_stuff;

pub fn apply_gravity(state: &mut GameState) {
    let down = Direction::Down.to_point();
    for (_, (mut pos,)) in state.world.query::<(&mut Position,)>().iter() {
        let new_pos = pos.0 + down;
        if !state.stuff[new_pos].is_solid() {
            move_stuff(pos.0, new_pos, &mut state.stuff);
            pos.0 = new_pos;
        }
    }
}
