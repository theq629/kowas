use bracket_terminal::prelude::*;
use crate::state::{UiState, UiStateAction};
use super::{FancyLogoMenuView, FancyLogoMenuChoice};
use crate::branding::TITLE;

bracket_terminal::embedded_resource!(LOGO_IMAGE, "../../../../resources/logo.xp");

pub fn make_main_menu(
    can_quit: bool,
    save_storage_avail: bool,
    save_exists: bool
) -> FancyLogoMenuView<UiState, UiStateAction> {
    bracket_terminal::link_resource!(LOGO_IMAGE, "../../../../resources/logo.xp");
    let logo = XpFile::from_resource("../../../../resources/logo.xp").ok();

    let mut choices = Vec::new();
    choices.push(FancyLogoMenuChoice {
        text: "new game".to_string(),
        key: 'n',
        enabled: true,
        handler: Box::new(|_| {
            Some(UiStateAction::NewGame)
        })
    });
    if save_storage_avail {
        choices.push(FancyLogoMenuChoice {
            text: "continue game".to_string(),
            key: 'c',
            enabled: save_exists,
            handler: Box::new(|_| {
                Some(UiStateAction::ContinueGame)
            })
        });
    }
    if can_quit {
        choices.push(FancyLogoMenuChoice {
            text: "quit".to_string(),
            key: 'q',
            enabled: true,
            handler: Box::new(|_| {
                Some(UiStateAction::Quit)
            })
        });
    }

    let mut warnings = Vec::new();
    if !save_storage_avail {
        warnings.push("storage not available, games cannot be saved".to_string());
    }

    FancyLogoMenuView::new(TITLE.to_string().to_uppercase(), logo, choices, warnings)
}
