use bracket_terminal::prelude::VirtualKeyCode;
use crate::input::{Key, KeyBindings};

fn make_vi_direction_bindings(bindings: &mut KeyBindings) {
    bindings.bind(VirtualKeyCode::H, Key::MoveW);
    bindings.bind(VirtualKeyCode::L, Key::MoveE);
    bindings.bind(VirtualKeyCode::K, Key::MoveN);
    bindings.bind(VirtualKeyCode::J, Key::MoveS);
    bindings.bind(VirtualKeyCode::Y, Key::MoveNW);
    bindings.bind(VirtualKeyCode::U, Key::MoveNE);
    bindings.bind(VirtualKeyCode::B, Key::MoveSW);
    bindings.bind(VirtualKeyCode::N, Key::MoveSE);
}

fn make_arrow_direction_bindings(bindings: &mut KeyBindings) {
    bindings.bind(VirtualKeyCode::Left, Key::MoveW);
    bindings.bind(VirtualKeyCode::Right, Key::MoveE);
    bindings.bind(VirtualKeyCode::Up, Key::MoveN);
    bindings.bind(VirtualKeyCode::Down, Key::MoveS);
}

fn make_numpad_direction_bindings(bindings: &mut KeyBindings) {
    bindings.bind(VirtualKeyCode::Numpad4, Key::MoveW);
    bindings.bind(VirtualKeyCode::Numpad6, Key::MoveE);
    bindings.bind(VirtualKeyCode::Numpad8, Key::MoveN);
    bindings.bind(VirtualKeyCode::Numpad2, Key::MoveS);
    bindings.bind(VirtualKeyCode::Numpad9, Key::MoveNW);
    bindings.bind(VirtualKeyCode::Numpad7, Key::MoveNE);
    bindings.bind(VirtualKeyCode::Numpad1, Key::MoveSW);
    bindings.bind(VirtualKeyCode::Numpad3, Key::MoveSE);
}

pub fn make_default_key_bindings() -> KeyBindings {
    let mut bindings = KeyBindings::new();

    make_vi_direction_bindings(&mut bindings);
    make_arrow_direction_bindings(&mut bindings);
    make_numpad_direction_bindings(&mut bindings);

    bindings.bind(VirtualKeyCode::S, Key::Shove);
    bindings.bind(VirtualKeyCode::A, Key::ThrowOff);
    bindings.bind(VirtualKeyCode::D, Key::Heal);
    bindings.bind(VirtualKeyCode::Z, Key::SwordSlash);
    bindings.bind(VirtualKeyCode::X, Key::SwordFlurry);
    bindings.bind(VirtualKeyCode::C, Key::SwordWhirl);
    bindings.bind(VirtualKeyCode::G, Key::Get);
    bindings.bind(VirtualKeyCode::Comma, Key::Get);


    #[cfg(debug_assertions)]
    {
        bindings.bind(VirtualKeyCode::E, Key::GetALotOfEnergy);
        bindings.bind(VirtualKeyCode::R, Key::GetALotOfHealth);
        bindings.bind(VirtualKeyCode::P, Key::GainPower);
    }

    bindings.bind(VirtualKeyCode::Space, Key::DoNothing);
    bindings.bind(VirtualKeyCode::Period, Key::DoNothing);

    bindings.bind(VirtualKeyCode::Slash, Key::Help);
    bindings.bind(VirtualKeyCode::Q, Key::Quit);

    bindings
}
