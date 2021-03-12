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
use hecs::{World, Entity};
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
pub use state::GameState;
use actions::Action;
use mapgen::gen_map;
use systems::ChangeResult;

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
        rng: rng
    }
}

fn dispatch_action(actor: Entity, action: Action, state: &mut GameState) -> ChangeResult {
    match action {
        Action::DoNothing => {
            systems::do_nothing(actor)
        },
        Action::Move(dir) => {
            systems::move_entity(actor, dir, state)
        },
        Action::MeleeAttack(dir) => {
            systems::melee_attack_toward(actor, dir, state)
        },
        Action::Shove(dir) => {
            systems::shove_toward(actor, dir, state)
        },
        Action::Slash(dir) => {
            systems::slash_toward(actor, dir, state)
        },
        Action::Get => {
            systems::get(actor, state)
        },
        Action::Drop(entity) => {
            systems::drop(actor, entity, state)
        },
        Action::GainPower => {
            systems::gain_power(actor, state)
        },
    }
}

pub fn act(actor: Entity, action: Action, state: &mut GameState) -> ChangeResult {
    debug!("acting");
    dispatch_action(actor, action, state).and_then(|ok| {
        debug!("updating flying");
        systems::update_flying(state);
        debug!("checking deaths");
        systems::check_deaths(state);
        if let Some(player) = state.player {
            if !state.world.contains(player) {
                state.player = None;
            }
        }
        debug!("done updates");
        Ok(ok)
    })
}

pub fn visual_tick(state: &mut GameState) {
    systems::tick_particles(state);
}
