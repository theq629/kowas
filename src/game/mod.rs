use std::io::{Read, Write};
use enum_map::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Enum)]
pub enum Graphic {
    Player,
    Floor
}

pub struct GameState {
    pub test_graphic: Graphic
}

pub fn new_game() -> GameState {
    return GameState {
        test_graphic: Graphic::Player
    }
}

impl GameState {
    pub fn save<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        Ok(())
    }

    pub fn load<R: Read>(_reader: &mut R) -> std::io::Result<Self> {
        Ok(new_game())
    }
}
