use crate::state::{UiState, UiStateAction};
use super::{DialogView, DialogChoice};

const TITLE: &'static str = "You won";
const TEXT: &'static str = "You have defeated the orc lord.";

pub fn make_win_dialog() -> DialogView<UiState, UiStateAction> {
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
