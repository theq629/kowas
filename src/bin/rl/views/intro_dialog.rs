use crate::state::{UiState, UiStateAction};
use super::{DialogView, DialogChoice};

const TITLE: &'static str = "Welcome";
const TEXT: &'static str = "You must defeat the orc lord.\n\nPress slash for help. Mouse around for tooltips.";

pub fn make_intro_dialog() -> DialogView<UiState, UiStateAction> {
DialogView::new(
        TITLE.to_string(),
        TEXT.to_string(),
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
    )
}
