use crate::state::{UiState, UiStateAction};
use super::{DialogView, DialogChoice};

const TITLE: &'static str = "Error loading game";

pub fn make_load_error_dialog<S: ToString>(err_msg: S) -> DialogView<UiState, UiStateAction> {
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
                text: "Delete save file".to_string(),
                key: 'D',
                handler: Box::new(|_state| {
                    Some(UiStateAction::RemoveSave)
                })
            }
        ]
    )
}
