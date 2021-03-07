pub mod stuff;
pub mod graphics;
pub mod components;
pub mod directions;
pub mod actions;
mod mapgen;
mod systems;
mod state;

use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use hecs::{World, Entity};
pub use state::GameState;
use stuff::Stuff;
use actions::Action;
use mapgen::gen_map;

pub fn make_player(pos: Point, state: &mut GameState) -> Entity {
    state.stuff[pos] = Stuff::Body;
    state.world.spawn((
        components::Position(pos),
    ))
}

pub fn new_game() -> GameState {
    let dim = Point::new(64, 128);
    let mut rng = RandomNumberGenerator::new();
    let stuff = gen_map(dim, &mut rng);

    let mut state = GameState {
        world: World::new(),
        stuff: stuff,
        player: None
    };

    let player = make_player(Point::new(dim.x / 2, 1), &mut state);
    state.player = Some(player);

    state
}

pub fn tick(state: &mut GameState, player_action: Action) {
    match player_action {
        Action::DoNothing => {},
        Action::Move(dir) => {
            if let Some(player) = state.player {
                systems::move_entity(player, dir, state);
            }
        }
    }
    systems::apply_gravity(state);
}
