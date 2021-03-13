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
pub use state::{GameState, GameStatus};
use actions::Action;
use mapgen::gen_map;
use systems::{ChangeResult, ChangeOk, ChangeErr};
use components::Energy;

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
        Action::SwordSlash(dir) => {
            systems::sword::slash_toward(actor, dir, state)
        },
        Action::SwordWhirl => {
            systems::sword::whirl(actor, state)
        },
        Action::SwordFlurry(dir) => {
            systems::sword::flurry_toward(actor, dir, state)
        },
        Action::Get => {
            systems::get(actor, state)
        },
        Action::Drop(entity) => {
            systems::drop(actor, entity, state)
        },
        Action::GetALotOfEnergy => {
            systems::get_a_lot_of_energy(actor, state)
        },
        Action::GainPower => {
            systems::gain_power(actor, state)
        },
    }
}

fn run_action(actor: Entity, action: Action, state: &mut GameState) -> ChangeResult {
    let needs_energy = action.needs_energy();
    if needs_energy > 0 {
        let actor_energy = state.world.get::<Energy>(actor)?;
        if actor_energy.value < needs_energy {
            return Err(ChangeErr);
        }
    }
    dispatch_action(actor, action, state)?;
    if let Ok(mut actor_energy) = state.world.get_mut::<Energy>(actor) {
        actor_energy.value -= needs_energy;
    }
    Ok(ChangeOk)
}

pub fn act(actor: Entity, action: Action, state: &mut GameState) -> ChangeResult {
    debug!("acting");
    run_action(actor, action, state).and_then(|ok| {
        debug!("updating flying");
        systems::update_flying(state);
        debug!("checking win");
        systems::check_win(state);
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
