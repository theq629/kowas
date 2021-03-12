use hecs::Entity;
use crate::log_err::result_error;
use crate::game::state::GameState;
use crate::game::liquids::Liquid;
use crate::game::components::{Health, Position, Energy, ProvidesEnergy};
use super::change::{ChangeResult, ChangeOk};
use super::splatter::splatter_blood;

fn die(dier: Entity, state: &mut GameState) -> ChangeResult {
    {
        let pos = state.world.get::<Position>(dier)?.0;
        splatter_blood(pos, 50, state);
        state.liquids[pos] = Some(Liquid::Gore);
    }
    if let Some(player) = state.player {
        let dier_prov_energy = state.world.get::<ProvidesEnergy>(dier)?.0;
        let mut player_energy = state.world.get_mut::<Energy>(player)?;
        player_energy.value += dier_prov_energy;
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
