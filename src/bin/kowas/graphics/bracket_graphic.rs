use bracket_terminal::prelude::*;
use enum_map::EnumMap;
use kowas::game::graphics::Graphic;

pub type GraphicLookup = EnumMap<Graphic, BracketGraphic>;

#[derive(Clone)]
pub struct BracketGraphic {
    pub glyph: FontCharType,
    pub colour: RGB
}
