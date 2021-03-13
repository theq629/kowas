use bracket_geometry::prelude::Point;
use kowas::game::GameState;
use kowas::game::components::{Position, Renderable};

pub fn cell_info(pos: Point, state: &GameState) -> Vec<&str> {
    let mut info = Vec::new();
    info.push(state.terrain[pos].name());
    if let Some(liquid) = state.liquids[pos] {
        info.push(liquid.name());
    }
    for (_, (_, renderable)) in state.world.query::<(&Position, &Renderable)>()
        .iter()
        .filter(|(_, (p, _))| p.0 == pos) {
        info.push(renderable.0.name());
    }
    info
}
