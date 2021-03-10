use std::cmp::{max, min};
use bracket_terminal::prelude::*;
use hecs::Entity;
use sevendrl_2021::log_err::result_error;
use sevendrl_2021::bracket_views::{Input, View};
use sevendrl_2021::game::{GameState, act, visual_tick};
use sevendrl_2021::game::graphics::Graphic;
use sevendrl_2021::game::liquids::Liquid;
use sevendrl_2021::game::components::{Position, Renderable, Health, Power};
use sevendrl_2021::game::actions::Action;
use sevendrl_2021::game::directions::Direction;
use crate::input::{Key, InputImpl};
use crate::state::{UiState, UiStateAction};
use crate::graphics::GraphicLookup;

enum InputMode {
    Move,
    Shove,
    Slash
}

pub struct GameView {
    bg_col: RGB,
    input_mode: InputMode
}

impl GameView {
    pub fn new() -> Self {
        GameView {
            bg_col: RGB::named(BLACK),
            input_mode: InputMode::Move
        }
    }
}

impl GameView {
    fn draw_map(&mut self, view_centre: Point, game_state: &GameState, graphics: &GraphicLookup, ctx: &mut BTerm) {
        let terrain = &game_state.terrain;
        let liquids = &game_state.liquids;

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
                let ter = terrain[pos];
                let graphic = match liquids[pos] {
                    None => graphics[ter.graphic()].clone(),
                    Some(Liquid::Gore) => graphics[Graphic::Gore].clone(),
                    Some(Liquid::Blood) => {
                        let mut g = graphics[ter.graphic()].clone();
                        g.colour = RGB::named(RED);
                        g
                    }
                };
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

        for (_, (pos, renderable)) in game_state.particles_world.query::<(&Position, &Renderable)>().iter() {
            let pos = pos.0;
            let graphic = &graphics[renderable.0];
            if pos.x >= world_min.x && pos.x < world_max.x && pos.y >= world_min.y && pos.y < world_max.y {
                let screen_pos = offset + pos;
                ctx.set(screen_pos.x, screen_pos.y, graphic.colour, self.bg_col, graphic.glyph);
            }
        }
    }

    fn draw_ui(&mut self, game_state: &GameState, ctx: &mut BTerm) {
        let bg = RGB::named(LIGHTGREY);

        let (dim_x, dim_y) = ctx.get_char_size();

        ctx.fill_region(Rect::with_size(0, dim_y - 1, dim_x, 1), to_cp437(' '), RGB::named(BLACK), bg);

        match self.input_mode {
            InputMode::Move => {
                if let Some(player) = game_state.player {
                    let power = game_state.world.get::<Power>(player).unwrap();
                    ctx.print_color(0, dim_y - 1, RGB::named(BLACK), bg, format!("POWER {}", power.0));
                    let health = game_state.world.get::<Health>(player).unwrap();
                    ctx.print_color(9, dim_y - 1, RGB::named(BLACK), bg, format!("HEALTH {}/{}", health.value, health.max));
                }
            },
            InputMode::Shove => {
                ctx.print_color_centered(dim_y - 1, RGB::named(BLACK), bg, "select direction to shove");
            },
            InputMode::Slash => {
                ctx.print_color_centered(dim_y - 1, RGB::named(BLACK), bg, "select direction to slash");
            }
        }
    }

    fn handle_move_input(&mut self, player: Entity, game_state: &mut GameState, input: &InputImpl) {
        fn move_or_attack(player: Entity, dir: Direction, game_state: &mut GameState) {
            let res = act(player, Action::MeleeAttack(dir), game_state).or_else(|_| {
                act(player, Action::Move(dir), game_state)
            });
            result_error(res);
        }

        if input.is_pressed(Key::MoveN) {
            move_or_attack(player, Direction::N, game_state);
        }
        if input.is_pressed(Key::MoveS) {
            move_or_attack(player, Direction::S, game_state);
        }
        if input.is_pressed(Key::MoveE) {
            move_or_attack(player, Direction::E, game_state);
        }
        if input.is_pressed(Key::MoveW) {
            move_or_attack(player, Direction::W, game_state);
        }
        if input.is_pressed(Key::MoveNE) {
            move_or_attack(player, Direction::NE, game_state);
        }
        if input.is_pressed(Key::MoveNW) {
            move_or_attack(player, Direction::NW, game_state);
        }
        if input.is_pressed(Key::MoveSE) {
            move_or_attack(player, Direction::SE, game_state);
        }
        if input.is_pressed(Key::MoveSW) {
            move_or_attack(player, Direction::SW, game_state);
        }
    }

    fn handle_shove_input(&mut self, player: Entity, game_state: &mut GameState, input: &InputImpl) {
        fn do_shove(view: &mut GameView, player: Entity, dir: Direction, game_state: &mut GameState) {
            view.input_mode = InputMode::Move;
            result_error(act(player, Action::Shove(dir), game_state))
        }

        if input.is_pressed(Key::MoveN) {
            do_shove(self, player, Direction::N, game_state);
        }
        if input.is_pressed(Key::MoveS) {
            do_shove(self, player, Direction::S, game_state);
        }
        if input.is_pressed(Key::MoveE) {
            do_shove(self, player, Direction::E, game_state);
        }
        if input.is_pressed(Key::MoveW) {
            do_shove(self, player, Direction::W, game_state);
        }
        if input.is_pressed(Key::MoveNE) {
            do_shove(self, player, Direction::NE, game_state);
        }
        if input.is_pressed(Key::MoveNW) {
            do_shove(self, player, Direction::NW, game_state);
        }
        if input.is_pressed(Key::MoveSE) {
            do_shove(self, player, Direction::SE, game_state);
        }
        if input.is_pressed(Key::MoveSW) {
            do_shove(self, player, Direction::SW, game_state);
        }
    }

    fn handle_slash_input(&mut self, player: Entity, game_state: &mut GameState, input: &InputImpl) {
        fn do_slash(view: &mut GameView, player: Entity, dir: Direction, game_state: &mut GameState) {
            view.input_mode = InputMode::Move;
            result_error(act(player, Action::Slash(dir), game_state))
        }

        if input.is_pressed(Key::MoveN) {
            do_slash(self, player, Direction::N, game_state);
        }
        if input.is_pressed(Key::MoveS) {
            do_slash(self, player, Direction::S, game_state);
        }
        if input.is_pressed(Key::MoveE) {
            do_slash(self, player, Direction::E, game_state);
        }
        if input.is_pressed(Key::MoveW) {
            do_slash(self, player, Direction::W, game_state);
        }
        if input.is_pressed(Key::MoveNE) {
            do_slash(self, player, Direction::NE, game_state);
        }
        if input.is_pressed(Key::MoveNW) {
            do_slash(self, player, Direction::NW, game_state);
        }
        if input.is_pressed(Key::MoveSE) {
            do_slash(self, player, Direction::SE, game_state);
        }
        if input.is_pressed(Key::MoveSW) {
            do_slash(self, player, Direction::SW, game_state);
        }
    }

    fn handle_action_input(&mut self, player: Entity, game_state: &mut GameState, input: &InputImpl) {
        if input.is_pressed(Key::DoNothing) {
            result_error(act(player, Action::DoNothing, game_state));
        }

        match self.input_mode {
            InputMode::Move => {
                self.handle_move_input(player, game_state, input)
            },
            InputMode::Shove => {
                self.handle_shove_input(player, game_state, input)
            },
            InputMode::Slash => {
                self.handle_slash_input(player, game_state, input)
            }
        }

        if input.is_pressed(Key::Slash) {
            self.input_mode = InputMode::Slash;
        }
        if input.is_pressed(Key::Shove) {
            self.input_mode = InputMode::Shove;
        }

        if input.is_pressed(Key::Get) {
            result_error(act(player, Action::Get, game_state));
        }

        if input.is_pressed(Key::GainPower) {
            result_error(act(player, Action::GainPower, game_state));
        }
    }

    fn handle_input(&mut self, state: &mut UiState, input: &InputImpl) -> Option<UiStateAction> {
        if let Some(game_state) = &mut state.game_state {
            if let Some(player) = game_state.player {
                self.handle_action_input(player, game_state, input);
            }
        }

        if input.is_pressed(Key::Quit) {
            return Some(UiStateAction::SaveAndMainMenu)
        }

        None
    }
}

impl View<UiState, Key, InputImpl, UiStateAction> for GameView {
    fn tick(&mut self, state: &mut UiState, input: &InputImpl, ctx: &mut BTerm) -> Option<UiStateAction> {
        ctx.cls();
        if let Some(game_state) = &mut state.game_state {
            visual_tick(game_state);
            if let Some(player) = game_state.player {
                let view_centre = game_state.world.get::<Position>(player)
                    .map(|p| p.0)
                    .unwrap_or(Point::new(0, 0));
                self.draw_map(view_centre, game_state, &state.graphics, ctx);
                self.draw_ui(game_state, ctx);
            }
        }
        self.handle_input(state, input)
    }
}
