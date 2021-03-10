use hecs::Entity;
use crate::game::state::GameState;
use crate::game::components::Power;
use super::change::{ChangeResult, ChangeOk};

pub fn gain_power(gainer: Entity, state: &mut GameState) -> ChangeResult {
    let mut power = state.world.get_mut::<Power>(gainer)?;
    power.0 += 1;
    Ok(ChangeOk)
}
