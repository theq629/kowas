use bracket_terminal::prelude::VirtualKeyCode;
use crate::input::{Key, KeyBindings};

pub fn make_default_key_bindings() -> KeyBindings {
    let mut bindings = KeyBindings::new();

    bindings.bind(VirtualKeyCode::H, Key::MoveLeft);
    bindings.bind(VirtualKeyCode::L, Key::MoveRight);

    bindings.bind(VirtualKeyCode::Left, Key::MoveLeft);
    bindings.bind(VirtualKeyCode::Right, Key::MoveRight);

    bindings.bind(VirtualKeyCode::Numpad4, Key::MoveLeft);
    bindings.bind(VirtualKeyCode::Numpad6, Key::MoveRight);

    bindings.bind(VirtualKeyCode::G, Key::Get);
    bindings.bind(VirtualKeyCode::Comma, Key::Get);

    bindings.bind(VirtualKeyCode::Z, Key::DoNothing);
    bindings.bind(VirtualKeyCode::Period, Key::DoNothing);

    bindings.bind(VirtualKeyCode::Q, Key::Quit);

    bindings
}
