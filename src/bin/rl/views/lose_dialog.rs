use crate::state::{UiState, UiStateAction};
use super::{DialogView, DialogChoice};

const TITLE: &'static str = "You lost";
const TEXT: &'static str = "You have been defeated.";

pub fn make_lose_dialog() -> DialogView<UiState, UiStateAction> {
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
