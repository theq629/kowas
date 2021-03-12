#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    DoNothing,
    MoveN,
    MoveS,
    MoveE,
    MoveW,
    MoveNE,
    MoveNW,
    MoveSE,
    MoveSW,
    Get,
    Shove,
    SwordSlash,
    SwordWhirl,
    GainPower,
    Help,
    Quit
}

impl Key {
    pub fn name(self) -> &'static str {
        match self {
            Key::DoNothing => "do nothing",
            Key::MoveN => "move north",
            Key::MoveS => "move south",
            Key::MoveE => "move east",
            Key::MoveW => "move west",
            Key::MoveNE => "move northeast",
            Key::MoveNW => "move northwest",
            Key::MoveSE => "move southeast",
            Key::MoveSW => "move southwest",
            Key::Get => "get",
            Key::Shove => "shove",
            Key::SwordSlash => "sword slash",
            Key::SwordWhirl => "sword whirl",
            Key::GainPower => "get more powerful",
            Key::Help => "help",
            Key::Quit => "quit",
        }
    }
}
