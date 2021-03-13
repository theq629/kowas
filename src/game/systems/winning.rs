use crate::game::state::{GameState, GameStatus};
use crate::game::components::{Health, IsPlayerGoal};

pub fn check_win(state: &mut GameState) {
    for _ in state.world.query::<(&Health, &IsPlayerGoal)>()
        .iter()
        .filter(|(_, (h, _))| h.value <= 0)
    {
        state.status = GameStatus::Won;
    }
}
