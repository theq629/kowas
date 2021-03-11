use std::collections::HashMap;
use bracket_terminal::prelude::VirtualKeyCode;
use crate::state::{UiState, UiStateAction};
use crate::input::{Key, KeyBindings, input_key_name};
use super::{DialogView, DialogChoice};

const TITLE: &'static str = "Help";

pub fn make_help_dialog(key_bindings: &KeyBindings) -> DialogView<UiState, UiStateAction> {
    let mut bindings: HashMap<Key, Vec<VirtualKeyCode>> = HashMap::new();
    for (ik, ok) in key_bindings.bindings().iter() {
        bindings.entry(*ok).or_insert_with(|| Vec::new()).push(*ik);
    }
    let mut bindings: Vec<_> = bindings
        .iter()
        .map(|(ok, iks)| (ok, iks))
        .collect();
    bindings.sort_by_key(|(ok, _)| ok.clone());
    let text_lines: Vec<_> = bindings.iter()
        .map(|(ok, iks)| {
            let iks_str = iks.iter().map(|ik| input_key_name(*ik)).collect::<Vec<_>>().join(", ");
            format!("{:<20} {}", ok.name(), iks_str)
        })
        .collect();
    let text = text_lines.join("\n");
    let mut view = DialogView::new(
        TITLE.to_string(),
        text,
        Some(0),
        vec![
            DialogChoice {
                text: "Ok".to_string(),
                key: 'O',
                handler: Box::new(|_state| {
                    Some(UiStateAction::Back)
                })
            }
        ]
    );
    view.max_text_width = 10000000;
    view
}
