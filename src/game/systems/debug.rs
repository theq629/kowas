use hecs::Entity;
use crate::game::state::GameState;
use crate::game::components::{Power, Energy, Health};
use super::change::{ChangeResult, ChangeOk};

pub fn get_a_lot_of_energy(getter: Entity, state: &mut GameState) -> ChangeResult {
    let mut energy = state.world.get_mut::<Energy>(getter)?;
    energy.change(1000);
    Ok(ChangeOk)
}

pub fn get_a_lot_of_health(getter: Entity, state: &mut GameState) -> ChangeResult {
    let mut health = state.world.get_mut::<Health>(getter)?;
    health.change(1000);
    Ok(ChangeOk)
}

pub fn gain_power(gainer: Entity, state: &mut GameState) -> ChangeResult {
    let mut power = state.world.get_mut::<Power>(gainer)?;
    power.0 += 1;
    Ok(ChangeOk)
}
