use bracket_terminal::prelude::*;
use sevendrl_2021::bracket_views::{Input, View};
use sevendrl_2021::game::GameState;
use crate::input::{Key, InputImpl};
    use crate::state::{UiState, UiStateAction};
use crate::graphics::GraphicLookup;

pub struct GameView {
    bg_col: RGB
}

impl GameView {
    pub fn new() -> Self {
        GameView {
            bg_col: RGB::named(BLACK)
        }
    }
}

impl GameView {
    fn draw(&mut self, game_state: &GameState, graphics: &GraphicLookup, ctx: &mut BTerm) {
        let graphic = &graphics[game_state.test_graphic];
        ctx.set(0, 0, graphic.colour, self.bg_col, graphic.glyph);
    }

    fn handle_input(&mut self, _state: &mut UiState, input: &InputImpl) -> Option<UiStateAction> {
        if input.is_pressed(Key::Quit) {
            return Some(UiStateAction::SaveAndMainMenu)
        }
        None
    }
}

impl View<UiState, Key, InputImpl, UiStateAction> for GameView {
    fn tick(&mut self, state: &mut UiState, input: &InputImpl, ctx: &mut BTerm) -> Option<UiStateAction> {
        ctx.set_active_console(0);
        ctx.cls();
        if let Some(game_state) = &state.game_state {
            self.draw(game_state, &state.graphics, ctx);
        }
        ctx.set_active_console(1);
        ctx.cls();
        self.handle_input(state, input)
    }
}
