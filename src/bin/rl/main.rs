use bracket_terminal::prelude::*;

mod branding;
mod graphics;
mod input;
mod state;
mod views;

use state::BracketState;

const WIDTH: i32 = 80;
const HEIGHT: i32 = 40;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title(branding::TITLE)
        .with_font("terminal8x8.png", 8u32, 8u32)
        .with_simple_console_no_bg(WIDTH as u32, HEIGHT as u32, "terminal8x8.png")
        .with_simple_console_no_bg(WIDTH as u32, HEIGHT as u32, "terminal8x8.png")
        .build()?;
    let graphics = graphics::make_ascii();

    let state = BracketState::new(graphics);
    main_loop(context, state)
}
