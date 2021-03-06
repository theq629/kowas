use crate::state::{UiState, UiStateAction};
use super::{DialogView, DialogChoice};

const TITLE: &'static str = "Welcome";
const TEXT: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas faucibus venenatis suscipit. Mauris ultricies sed elit vel congue. Nunc fringilla, risus eget scelerisque fringilla, lorem augue sollicitudin sem, nec laoreet massa lacus ut arcu. Donec euismod ligula posuere, consectetur libero vitae, porta lacus. Nunc eleifend elementum arcu sit amet iaculis. Fusce arcu tortor, rhoncus quis pretium sit amet, tempus vitae justo. Sed aliquet turpis nunc, nec egestas urna aliquam ut. Praesent ultrices, tortor ac iaculis posuere, tortor nibh interdum dui, imperdiet sollicitudin arcu dui vitae ante. Ut fermentum enim eu lacus vestibulum, eu rutrum dolor bibendum. Nulla fringilla erat ut fermentum tempor. In pulvinar dui quam, nec sodales libero tempor ac. Sed condimentum orci non lectus pharetra dictum.";

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
