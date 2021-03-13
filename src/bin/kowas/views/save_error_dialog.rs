use crate::state::{UiState, UiStateAction};
use super::dialog::{DialogView, DialogChoice};

const TITLE: &'static str = "Error saving game";

pub fn make_save_error_dialog<S: ToString>(err_msg: S) -> DialogView<UiState, UiStateAction> {
    DialogView::new(
        TITLE.to_string(),
        err_msg.to_string(),
        Some(0),
        vec![
            DialogChoice {
                text: "Ok".to_string(),
                key: 'O',
                handler: Box::new(|_state| {
                    Some(UiStateAction::Back)
                })
            },
            DialogChoice {
                text: "Quit without saving".to_string(),
                key: 'Q',
                handler: Box::new(|_state| {
                    Some(UiStateAction::EndGameAndMainMenu)
                })
            }
        ]
    )
}
