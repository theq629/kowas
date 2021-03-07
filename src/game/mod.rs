pub mod terrain;
pub mod graphics;
pub mod components;
pub mod directions;
pub mod actions;
mod mapgen;
mod things;
mod systems;
mod state;

use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use hecs::World;
use crate::log_err::result_error;
pub use state::GameState;
use actions::Action;
use mapgen::gen_map;

pub fn new_game() -> GameState {
    let dim = Point::new(64, 128);
    let mut rng = RandomNumberGenerator::new();
    let terrain = gen_map(dim, &mut rng);

    let mut state = GameState {
        world: World::new(),
        terrain: terrain,
        player: None
    };

    things::water_potion(Point::new(dim.x / 2 + 1, 1), &mut state);

    let player = things::player(Point::new(dim.x / 2, 1), &mut state);
    state.player = Some(player);

    state
}

pub fn tick(state: &mut GameState, player_action: Action) {
    match player_action {
        Action::DoNothing => {},
        Action::Move(dir) => {
            if let Some(player) = state.player {
                result_error(systems::move_entity(player, dir, state));
            }
        },
        Action::Get => {
            if let Some(player) = state.player {
                result_error(systems::get(player, state));
            }
        },
    }
}
