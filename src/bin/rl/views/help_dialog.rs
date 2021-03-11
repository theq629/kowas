use crate::state::{UiState, UiStateAction};
use super::{DialogView, DialogChoice};

const TITLE: &'static str = "Help";
const TEXT: &'static str = "This is some helpful help text.";

pub fn make_help_dialog() -> DialogView<UiState, UiStateAction> {
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
