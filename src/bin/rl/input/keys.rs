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
    Heal,
    ThrowOff,
    SwordSlash,
    SwordWhirl,
    SwordFlurry,
    GetALotOfEnergy,
    GetALotOfHealth,
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
            Key::ThrowOff => "throw off",
            Key::Heal => "heal",
            Key::SwordSlash => "sword slash",
            Key::SwordWhirl => "sword whirl",
            Key::SwordFlurry => "sword flurry",
            Key::GetALotOfEnergy => "get a lot of energy",
            Key::GetALotOfHealth => "get a lot of health",
            Key::GainPower => "get more powerful",
            Key::Help => "help",
            Key::Quit => "quit",
        }
    }

    pub fn needs_energy(self) -> Option<i32> {
        match self {
            Key::Shove => Some(Action::Shove(Direction::N).needs_energy()),
            Key::ThrowOff => Some(Action::ThrowOff.needs_energy()),
            Key::Heal => Some(Action::Heal.needs_energy()),
            Key::SwordSlash => Some(Action::SwordSlash(Direction::N).needs_energy()),
            Key::SwordWhirl => Some(Action::SwordWhirl.needs_energy()),
            Key::SwordFlurry => Some(Action::SwordFlurry(Direction::N).needs_energy()),
            _ => None
        }
    }
}
