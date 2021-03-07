use enum_map::{EnumMap, enum_map};
use bracket_terminal::prelude::*;
use sevendrl_2021::game::graphics::Graphic;
use super::BracketGraphic;

pub fn make_ascii() -> EnumMap<Graphic, BracketGraphic> {
    enum_map! {
        Graphic::Body => BracketGraphic {
            glyph: to_cp437('@'),
            colour: RGB::named(BLUE)
        },
        Graphic::Empty => BracketGraphic {
            glyph: to_cp437('.'),
            colour: RGB::named(LIGHT_GREY)
        },
        Graphic::Floor => BracketGraphic {
            glyph: to_cp437('#'),
            colour: RGB::named(GREY)
        }
    }
}
