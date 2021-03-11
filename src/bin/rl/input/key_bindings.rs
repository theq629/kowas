use std::collections::HashMap;
use bracket_terminal::prelude::*;
use sevendrl_2021::bracket_views::Input;
use super::keys::Key;

pub struct KeyBindings {
    bindings: HashMap<VirtualKeyCode, Vec<Key>>,
    empty_keys: Vec<Key>
}

impl KeyBindings {
    pub fn new() -> Self {
        KeyBindings {
            bindings: HashMap::new(),
            empty_keys: Vec::new()
        }
    }

    pub fn get(&self, input_key: VirtualKeyCode) -> &Vec<Key> {
        if let Some(keys) = self.bindings.get(&input_key) {
            keys
        } else {
            &self.empty_keys
        }
    }

    pub fn bind(&mut self, input_key: VirtualKeyCode, key: Key) {
        if !self.bindings.contains_key(&input_key) {
            self.bindings.insert(input_key, vec![key]);
        } else {
            self.bindings.get_mut(&input_key).unwrap().push(key);
        }
    }

    pub fn bindings(&self) -> Vec<(VirtualKeyCode, Key)> {
        return self.bindings.iter()
            .map(|(ik, oks)| oks.iter().map(move |ok| (ik.clone(), ok.clone())))
            .flatten()
            .collect();
    }
}

pub struct InputImpl {
    mouse_pos: Point,
    keys: Vec<Key>
}

impl InputImpl {
    pub fn new(ctx: &BTerm, key_bindings: &KeyBindings) -> Self {
        let keys =
            if let Some(input_key) = ctx.key {
                key_bindings.get(input_key).clone()
            } else {
                Vec::new()
            };
        InputImpl {
            mouse_pos: Point::from_tuple(ctx.mouse_pos.clone()),
            keys: keys
        }
    }
}

impl Input<Key> for InputImpl {
    fn get_mouse_pos(&self) -> Point {
        self.mouse_pos
    }

    fn is_pressed(&self, key: Key) -> bool {
        self.keys.contains(&key)
    }
}
