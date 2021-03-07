#[derive(Clone, Copy, PartialEq, Eq)]
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
    Quit
}
