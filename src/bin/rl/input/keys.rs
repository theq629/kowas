use sevendrl_2021::game::directions::Direction;
use sevendrl_2021::game::actions::Action;

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
    SwordFlurry,
    GetALotOfEnergy,
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
            Key::SwordFlurry => "sword flurry",
            Key::GetALotOfEnergy => "get a lot of energy",
            Key::GainPower => "get more powerful",
            Key::Help => "help",
            Key::Quit => "quit",
        }
    }

    pub fn needs_energy(self) -> Option<i32> {
        match self {
            Key::SwordSlash => Some(Action::SwordSlash(Direction::N).needs_energy()),
            Key::SwordWhirl => Some(Action::SwordWhirl.needs_energy()),
            Key::SwordFlurry => Some(Action::SwordFlurry(Direction::N).needs_energy()),
            _ => None
        }
    }
}
