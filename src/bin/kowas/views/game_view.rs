use std::cmp::{max, min};
use std::collections::HashMap;
use bracket_terminal::prelude::*;
use kowas::log_err::result_error;
use kowas::bracket_views::{Input, View};
use kowas::game::{GameState, GameStatus, act_player, visual_tick};
use kowas::game::graphics::Graphic;
use kowas::game::liquids::Liquid;
use kowas::game::components::{Position, Renderable, Health, Energy, MaxHealthEstimate, MaxEnergyEstimate};
use kowas::game::actions::Action;
use kowas::game::directions::Direction;
use crate::input::{Key, KeyBindings, InputImpl, input_key_name};
use crate::state::{UiState, UiStateAction};
use crate::graphics::GraphicLookup;
use super::cell_info::cell_info;

enum InputMode {
    Move,
    Shove,
    SwordSlash,
    SwordFlurry
}

pub struct GameView {
    bg_col: RGB,
    input_mode: InputMode,
    action_bindings_info: Vec<(String, String)>,
    action_bindings_info_width: u32
}

impl GameView {
    pub fn new(key_bindings: &KeyBindings) -> Self {
        let mut bindings: HashMap<Key, Vec<VirtualKeyCode>> = HashMap::new();
        for (ik, ok) in key_bindings.bindings().iter() {
            bindings.entry(*ok).or_insert_with(|| Vec::new()).push(*ik);
        }
        let mut bindings: Vec<_> = bindings
            .iter()
            .filter(|(ok, _)| ok.is_special_action())
            .map(|(ok, iks)| (ok, iks))
            .collect();
        bindings.sort_by_key(|(ok, _)| ok.clone());
        let action_bindings_info: Vec<_> = bindings.iter()
            .map(|(ok, iks)| {
                let iks: Vec<_> = iks.iter().map(|ik| input_key_name(*ik).to_lowercase()).collect();
                (ok.name().to_string(), iks.join(","))
            })
            .collect();

        let mut width = 0 as u32;
        for (ok, iks) in action_bindings_info.iter() {
            width += 1 + iks.len() as u32 + 1 + ok.len() as u32;
        }

        GameView {
            bg_col: RGB::named(BLACK),
            input_mode: InputMode::Move,
            action_bindings_info: action_bindings_info,
            action_bindings_info_width: width
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

    fn draw_stats_ui(&mut self, game_state: &GameState, ctx: &mut BTerm) {
        let bg = RGB::named(LIGHTGREY);
        let bg_low = RGB::named(YELLOW);
        let bg_critical = RGB::named(RED);

        let (dim_x, dim_y) = ctx.get_char_size();
        let row = (dim_y - 2) as i32;

        ctx.fill_region(Rect::with_size(0, row, dim_x as i32, 1), to_cp437(' '), RGB::named(BLACK), bg);

        let num_width = 4;
        let margin = 4;
        let health_width = 6 + num_width;
        let energy_width = 6 + num_width;
        let spacing = (dim_x - health_width - energy_width - margin) / 2;
        let health_start = spacing;
        let energy_start = spacing + health_width + margin;

        let choose_bg = |value: i32, max: i32| {
            if value <= max / 3 { bg_critical }
            else if value <= 2 * max / 3 { bg_low }
            else { bg }
        };

        match self.input_mode {
            InputMode::Move => {
                if let Some(player) = game_state.player {
                    let health = game_state.world.get::<Health>(player).unwrap();
                    let health_max_estimate = game_state.world.get::<MaxHealthEstimate>(player).unwrap().estimate.estimate;
                    let health_bg = choose_bg(health.value, health_max_estimate);
                    ctx.print_color(health_start, row, RGB::named(BLACK), health_bg, format!(" HEALTH {} ", health.value));
                    let energy = game_state.world.get::<Energy>(player).unwrap();
                    let energy_max_estimate = game_state.world.get::<MaxEnergyEstimate>(player).unwrap().estimate.estimate;
                    let energy_bg = choose_bg(energy.value, energy_max_estimate);
                    ctx.print_color(energy_start, row, RGB::named(BLACK), energy_bg, format!("ENERGY {} ", energy.value));
                }
            },
            InputMode::Shove => {
                ctx.print_color_centered(row, RGB::named(BLACK), bg, "select direction to shove");
            },
            InputMode::SwordSlash => {
                ctx.print_color_centered(row, RGB::named(BLACK), bg, "select direction to slash");
            },
            InputMode::SwordFlurry => {
                ctx.print_color_centered(row, RGB::named(BLACK), bg, "select direction to flurry");
            }
        }
    }

    fn draw_keys_ui(&mut self, _game_state: &GameState, ctx: &mut BTerm) {
        let bg = RGB::named(LIGHTGREY);

        let (dim_x, dim_y) = ctx.get_char_size();
        let row = (dim_y - 1) as i32;

        let mut x = (dim_x - self.action_bindings_info_width) / 2;
        for (ok, iks) in self.action_bindings_info.iter() {
            x += 1;
            ctx.print_color(x, row, RGB::named(BLACK), bg, iks);
            x += iks.len() as u32 + 1;
            ctx.print_color(x, row, RGB::named(DARKGREY), bg, ok);
            x += ok.len() as u32;
        }
    }

    fn draw_tooltips(&self, view_centre: Point, game_state: &GameState, ctx: &mut BTerm) {
        let (screen_width, screen_height) = ctx.get_char_size();

        let bg = RGB::from_u8(64, 64, 64);
        let fg = RGB::from_u8(192, 192, 192);
        let arrow_fg = RGB::from_u8(255, 255, 255);

        let (mouse_x, mouse_y) = ctx.mouse_pos();
        if mouse_y >= (screen_height - 8) as i32 {
            return;
        }

        let world_mouse_pos = self.screen_to_world_point(Point::new(mouse_x, mouse_y), view_centre, game_state, ctx);
        if world_mouse_pos.is_none() {
            return;
        }
        let world_mouse_pos = world_mouse_pos.unwrap();

        let tooltip = cell_info(world_mouse_pos, game_state);

        if !tooltip.is_empty() {
            let mut width: i32 = 0;
            for s in tooltip.iter() {
                if width < s.len() as i32 {
                    width = s.len() as i32;
                }
            }
            width += 3;

            let arrow_pos: Point;
            let arrow: String;
            let box_x: i32;
            let text_x: i32;
            if mouse_x > (screen_width / 2) as i32 {
                arrow_pos = Point::new(mouse_x - 2, mouse_y);
                arrow = "->".to_string();
                box_x = mouse_x - width;
                text_x = mouse_x - width;
            } else {
                arrow_pos = Point::new(mouse_x + 1, mouse_y);
                arrow = "<-".to_string();
                box_x = mouse_x + 1;
                text_x = mouse_x + 4;
            }
            ctx.fill_region(Rect::with_size(box_x, mouse_y, width - 1, (tooltip.len() - 1) as i32), to_cp437(' '), fg, bg);
            let mut y = mouse_y;
            for s in tooltip.iter() {
                ctx.print_color(text_x, y, fg, bg, s);
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, arrow_fg, bg, &arrow);
        }
    }

    fn screen_to_world_point(&self, point: Point, view_centre: Point, game_state: &GameState, ctx: &mut BTerm) -> Option<Point> {
        let (screen_width, screen_height) = ctx.get_char_size();
        let world_min = Point::new(
            max(0, view_centre.x - (screen_width / 2) as i32),
            max(0, view_centre.y - (screen_height / 2) as i32)
        );
        let screen_min = Point::new(
            max(0, (screen_width / 2) as i32 - view_centre.x),
            max(0, (screen_height / 2) as i32 - view_centre.y)
        );
        let world_pos = point + world_min - screen_min;
        if world_pos.x >= 0
            && world_pos.y >= 0
            && world_pos.x < game_state.terrain.dim.x
            && world_pos.y < game_state.terrain.dim.y {
            Some(world_pos)
        } else {
            None
        }
    }

    fn handle_move_input(&mut self, game_state: &mut GameState, input: &InputImpl) {
        handle_directional_action_input(input, |dir| {
            if let Some(dir) = dir {
                let res = act_player(Action::MeleeAttack(dir), game_state).or_else(|_| {
                    act_player(Action::Move(dir), game_state)
                });
                result_error(res);
            }
        });
    }

    fn handle_shove_input(&mut self, game_state: &mut GameState, input: &InputImpl) {
        handle_directional_action_input(input, |dir| {
            self.input_mode = InputMode::Move;
            if let Some(dir) = dir {
                result_error(act_player(Action::Shove(dir), game_state))
            }
        });
    }

    fn handle_slash_input(&mut self, game_state: &mut GameState, input: &InputImpl) {
        handle_directional_action_input(input, |dir| {
            self.input_mode = InputMode::Move;
            if let Some(dir) = dir {
                result_error(act_player(Action::SwordSlash(dir), game_state))
            }
        });
    }

    fn handle_flurry_input(&mut self, game_state: &mut GameState, input: &InputImpl) {
        handle_directional_action_input(input, |dir| {
            self.input_mode = InputMode::Move;
            if let Some(dir) = dir {
                result_error(act_player(Action::SwordFlurry(dir), game_state))
            }
        });
    }

    fn handle_action_input(&mut self, game_state: &mut GameState, input: &InputImpl) {
        if input.is_pressed(Key::DoNothing) {
            result_error(act_player(Action::DoNothing, game_state));
        }

        match self.input_mode {
            InputMode::Move => {
                self.handle_move_input(game_state, input)
            },
            InputMode::Shove => {
                self.handle_shove_input(game_state, input)
            },
            InputMode::SwordSlash => {
                self.handle_slash_input(game_state, input)
            },
            InputMode::SwordFlurry => {
                self.handle_flurry_input(game_state, input)
            }
        }

        if input.is_pressed(Key::SwordSlash) {
            self.input_mode = InputMode::SwordSlash;
        }
        if input.is_pressed(Key::SwordWhirl) {
            result_error(act_player(Action::SwordWhirl, game_state));
        }
        if input.is_pressed(Key::SwordFlurry) {
            self.input_mode = InputMode::SwordFlurry;
        }
        if input.is_pressed(Key::Shove) {
            self.input_mode = InputMode::Shove;
        }
        if input.is_pressed(Key::ThrowOff) {
            result_error(act_player(Action::ThrowOff, game_state));
        }
        if input.is_pressed(Key::Heal) {
            result_error(act_player(Action::Heal, game_state));
        }

        if input.is_pressed(Key::GetALotOfEnergy) {
            result_error(act_player(Action::GetALotOfEnergy, game_state));
        }
        if input.is_pressed(Key::GetALotOfHealth) {
            result_error(act_player(Action::GetALotOfHealth, game_state));
        }
        if input.is_pressed(Key::GainPower) {
            result_error(act_player(Action::GainPower, game_state));
        }
    }

    fn handle_input(&mut self, state: &mut UiState, input: &InputImpl) -> Option<UiStateAction> {
        if let Some(game_state) = &mut state.game_state {
            self.handle_action_input(game_state, input);
        }

        if input.is_pressed(Key::Help) {
            return Some(UiStateAction::ShowHelp)
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
            match game_state.status {
                GameStatus::Won => {
                    return Some(UiStateAction::WinGameAndMainMenu);
                },
                GameStatus::Lost => {
                    return Some(UiStateAction::LoseGameAndMainMenu);
                },
                _ => {}
            }
            visual_tick(game_state);
            if let Some(player) = game_state.player {
                let view_centre = game_state.world.get::<Position>(player)
                    .map(|p| p.0)
                    .unwrap_or(Point::new(0, 0));
                self.draw_map(view_centre, game_state, &state.graphics, ctx);
                self.draw_stats_ui(game_state, ctx);
                self.draw_keys_ui(game_state, ctx);
                self.draw_tooltips(view_centre, game_state, ctx);
            }
        }
        self.handle_input(state, input)
    }
}

fn handle_directional_action_input<F>(input: &InputImpl, mut action: F)
    where F: FnMut(Option<Direction>) -> ()
{
    if input.is_pressed(Key::MoveN) {
        action(Some(Direction::N))
    }
    if input.is_pressed(Key::MoveS) {
        action(Some(Direction::S))
    }
    if input.is_pressed(Key::MoveE) {
        action(Some(Direction::E))
    }
    if input.is_pressed(Key::MoveW) {
        action(Some(Direction::W))
    }
    if input.is_pressed(Key::MoveNE) {
        action(Some(Direction::NE))
    }
    if input.is_pressed(Key::MoveNW) {
        action(Some(Direction::NW))
    }
    if input.is_pressed(Key::MoveSE) {
        action(Some(Direction::SE))
    }
    if input.is_pressed(Key::MoveSW) {
        action(Some(Direction::SW))
    }
    if input.is_pressed(Key::Cancel) {
        action(None)
    }
}
