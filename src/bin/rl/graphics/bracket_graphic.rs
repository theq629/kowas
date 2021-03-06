use bracket_terminal::prelude::*;
use enum_map::EnumMap;
use sevendrl_2021::game::graphics::Graphic;

pub type GraphicLookup = EnumMap<Graphic, BracketGraphic>;

pub struct BracketGraphic {
    pub glyph: FontCharType,
    pub colour: RGB
}
