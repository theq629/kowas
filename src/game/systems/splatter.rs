use bracket_geometry::prelude::Point;
use crate::game::state::GameState;
use crate::game::liquids::Liquid;

pub fn splatter_blood(pos: Point, prob: i32, state: &mut GameState) {
    {
        for x in (pos.x - 1)..(pos.x + 2) {
            for y in (pos.y - 1)..(pos.y + 2) {
                if state.rng.range(0, 100) < prob {
                    let pos = Point::new(x, y);
                    if state.liquids[pos] != Some(Liquid::Gore) {
                        state.liquids[pos] = Some(Liquid::Blood);
                    }
                }
            }
        }
    }
}
