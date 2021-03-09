pub mod terrain;
pub mod graphics;
pub mod components;
pub mod directions;
pub mod actions;
mod mapgen;
mod things;
mod systems;
mod state;

use hecs::Entity;
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
pub use state::GameState;
use actions::Action;
use mapgen::gen_map;
use systems::ChangeResult;

pub fn new_game() -> GameState {
    let dim = Point::new(64, 128);
    let mut rng = RandomNumberGenerator::new();
    gen_map(dim, &mut rng)
}

pub fn act(actor: Entity, action: Action, state: &mut GameState) -> ChangeResult {
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
        Action::Get => {
            systems::get(actor, state)
        },
        Action::Drop(entity) => {
            systems::drop(actor, entity, state)
        },
    }
}
