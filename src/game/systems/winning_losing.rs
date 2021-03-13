use crate::game::state::{GameState, GameStatus};
use crate::game::components::{Health, IsPlayer, IsPlayerGoal};

pub fn check_win_lose(state: &mut GameState) {
    for _ in state.world.query::<(&Health, &IsPlayer)>()
        .iter()
        .filter(|(_, (h, _))| h.value <= 0)
    {
        state.status = GameStatus::Lost;
    }
    for _ in state.world.query::<(&Health, &IsPlayerGoal)>()
        .iter()
        .filter(|(_, (h, _))| h.value <= 0)
    {
        state.status = GameStatus::Won;
    }
}
