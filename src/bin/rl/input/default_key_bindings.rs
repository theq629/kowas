use bracket_terminal::prelude::VirtualKeyCode;
use crate::input::{Key, KeyBindings};

pub fn make_default_key_bindings() -> KeyBindings {
    let mut bindings = KeyBindings::new();
    bindings.bind(VirtualKeyCode::Q, Key::Quit);
    bindings
}
