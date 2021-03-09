use hecs::Entity;
use crate::log_err::result_error;
use crate::game::state::GameState;
use crate::game::liquids::Liquid;
use crate::game::components::{Health, Position};
use super::change::{ChangeResult, ChangeOk};

fn die(dier: Entity, state: &mut GameState) -> ChangeResult {
    {
        let pos = state.world.get::<Position>(dier)?;
        state.liquids[pos.0] = Some(Liquid::Gore);
    }
    state.world.despawn(dier)?;
    Ok(ChangeOk)
}

pub fn check_deaths(state: &mut GameState) {
    let to_die: Vec<_> = state.world.query::<(&Health,)>()
        .iter()
        .filter(|(_, (h,))| h.value <= 0)
        .map(|(e, _)| e)
        .collect();
    for dier in to_die.iter() {
        result_error(die(*dier, state));
    }
}
