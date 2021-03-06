use std::cmp::{max, min};
use bracket_terminal::prelude::*;
use sevendrl_2021::bracket_views::{Input, View};
use sevendrl_2021::game::{GameState, tick};
use sevendrl_2021::game::components::{Position, Renderable};
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
        let terrain = &game_state.terrain;
        let player = &game_state.player;

        let view_centre = game_state.world.get::<Position>(*player)
            .map(|p| p.0)
            .unwrap_or(Point::new(0, 0));

        let screen_dim = Point::from_tuple(ctx.get_char_size());
        let world_min = Point::new(
            max(0, view_centre.x - (screen_dim.x / 2) as i32),
            max(0, view_centre.y - (screen_dim.y / 2) as i32)
        );
        let screen_min = Point::new(
            max(0, (screen_dim.x / 2) as i32 - view_centre.x),
            max(0, (screen_dim.y / 2) as i32 - view_centre.y)
        );
        let world_max = Point::new(
            min(terrain.dim.x, world_min.x + screen_dim.x as i32),
            min(terrain.dim.y, world_min.y + screen_dim.y as i32)
        );

        let mut screen_y = screen_min.y;
        for world_y in world_min.y..world_max.y {
            let mut screen_x = screen_min.x;
            for world_x in world_min.x..world_max.x {
                let pos = Point::new(world_x, world_y);
                let graphic = &graphics[terrain[pos].graphic()];
                ctx.set(screen_x, screen_y, graphic.colour, self.bg_col, graphic.glyph);
                screen_x += 1;
            }
            screen_y += 1;
        }

        let offset = screen_min - world_min;
        for (_, (pos, renderable)) in game_state.world.query::<(&Position, &Renderable)>().iter() {
            let pos = pos.0;
            let graphic = &graphics[renderable.0];
            if pos.x >= world_min.x && pos.x < world_max.x && pos.y >= world_min.y && pos.y < world_max.y {
                let screen_pos = offset + pos;
                ctx.set(screen_pos.x, screen_pos.y, graphic.colour, self.bg_col, graphic.glyph);
            }
        }
    }

    fn handle_input(&mut self, state: &mut UiState, input: &InputImpl) -> Option<UiStateAction> {
        let mut game_change = false;
        if input.is_pressed(Key::DoNothing) {
            game_change = true;
        }
        if input.is_pressed(Key::MoveLeft) {
            game_change = true;
        }
        if input.is_pressed(Key::MoveRight) {
            game_change = true;
        }

        if input.is_pressed(Key::Quit) {
            return Some(UiStateAction::SaveAndMainMenu)
        }

        if game_change {
            if let Some(game_state) = &mut state.game_state {
                tick(game_state);
            }
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
