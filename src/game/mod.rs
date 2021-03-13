pub mod terrain;
pub mod liquids;
pub mod graphics;
pub mod components;
pub mod directions;
pub mod actions;
mod mapgen;
mod things;
mod systems;
mod state;

use log::debug;
use hecs::World;
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
pub use state::{GameState, GameStatus};
use actions::Action;
use mapgen::gen_map;
use systems::{ChangeResult, ChangeOk, ChangeErr};

pub fn new_game() -> GameState {
    let dim = Point::new(64, 128);
    let mut rng = RandomNumberGenerator::new();
    let gened = gen_map(dim, &mut rng);
    GameState {
        world: gened.world,
        particles_world: World::new(),
        terrain: gened.terrain,
        liquids: gened.liquids,
        player: gened.player,
        rng: rng,
        status: GameStatus::Playing
    }
}

pub fn act_player(action: Action, state: &mut GameState) -> ChangeResult {
    if let Some(player) = state.player {
        debug!("acting for player");
        systems::act(player, action, state)?;
        systems::act_monsters(state);
        Ok(ChangeOk)
    } else {
        Err(ChangeErr)
    }
}

pub fn visual_tick(state: &mut GameState) {
    systems::tick_particles(state);
}
