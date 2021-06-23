use crate::game::state::GameState;
use crate::game::components::{Health, Energy, MaxHealthEstimate, MaxEnergyEstimate};

pub fn update_stat_max_estimates(state: &mut GameState) {
    for (_, (stat, estimate)) in state.world.query::<(&Health, &mut MaxHealthEstimate)>().iter() {
        estimate.estimate.sample(stat.value);
    }
    for (_, (stat, estimate)) in state.world.query::<(&Energy, &mut MaxEnergyEstimate)>().iter() {
        estimate.estimate.sample(stat.value);
    }
}
