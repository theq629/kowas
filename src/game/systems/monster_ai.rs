use log::debug;
use crate::log_err::result_error;
use crate::game::state::GameState;
use crate::game::actions::Action;
use crate::game::components::IsAi;

pub fn act_monsters(state: &mut GameState) {
    debug!("starting monster ai");

    let monsters: Vec<_> = state.world.query::<&IsAi>()
        .iter()
        .map(|(e, _)| e)
        .collect();

    for monster in monsters {
        result_error(super::act(monster, Action::DoNothing, state));
    }
}
