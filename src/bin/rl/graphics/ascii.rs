use enum_map::{EnumMap, enum_map};
use bracket_terminal::prelude::*;
use sevendrl_2021::game::graphics::Graphic;
use super::BracketGraphic;

pub fn make_ascii() -> EnumMap<Graphic, BracketGraphic> {
    enum_map! {
        Graphic::Player => BracketGraphic {
            glyph: to_cp437('@'),
            colour: RGB::named(BLUE)
        },
        Graphic::Floor => BracketGraphic {
            glyph: to_cp437('.'),
            colour: RGB::named(DARKGREY)
        },
        Graphic::FloorUnderRoof => BracketGraphic {
            glyph: to_cp437('_'),
            colour: RGB::named(LIGHTGREY)
        },
        Graphic::Wall => BracketGraphic {
            glyph: to_cp437('#'),
            colour: RGB::named(GREY)
        },
        Graphic::Gore => BracketGraphic {
            glyph: to_cp437('%'),
            colour: RGB::named(RED)
        },
        Graphic::Goblin => BracketGraphic {
            glyph: to_cp437('g'),
            colour: RGB::named(GREY)
        },
        Graphic::Orc => BracketGraphic {
            glyph: to_cp437('o'),
            colour: RGB::named(GREY)
        },
        Graphic::DamageEffect => BracketGraphic {
            glyph: to_cp437('!'),
            colour: RGB::named(RED)
        }
    }
}
