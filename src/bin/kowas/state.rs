use bracket_terminal::prelude::*;
use bracket_terminal::prelude::GameState as BracketGameState;
use kowas::easy_storage::{FileMode, File, get_storage};
use kowas::easy_quitter::{Quitter, get_quitter};
use kowas::bracket_views::{View, ViewStack};
use kowas::game::{GameState, new_game};
use crate::graphics::GraphicLookup;
use crate::input::{Key, KeyBindings, InputImpl, make_default_key_bindings};
use super::views::{GameView, make_main_menu, make_intro_dialog, make_save_error_dialog, make_load_error_dialog, make_help_dialog, make_win_dialog, make_lose_dialog};
use super::branding::FILENAME;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UiStateAction {
    Back,
    NewGame,
    ContinueGame,
    Quit,
    SaveAndMainMenu,
    EndGameAndMainMenu,
    WinGameAndMainMenu,
    LoseGameAndMainMenu,
    ShowHelp,
    RemoveSave
}

pub struct UiState {
    save_file: Option<Box<dyn File>>,
    quitter: Option<Box<dyn Quitter>>,
    pub ui_can_quit: bool,
    pub ui_save_storage_avail: bool,
    pub show_intro_dialog: bool,
    pub graphics: GraphicLookup,
    pub game_state: Option<GameState>
}

impl UiState {
    pub fn new_game(&mut self) {
        self.game_state = Some(new_game());
    }

    fn save_game(&mut self) -> std::io::Result<()> {
        if let Some(file) = &mut self.save_file {
            if let Some(game_state) = &mut self.game_state {
                let mut writer = file.writer()?;
                game_state.save(&mut writer)?;
                writer.flush()?;
            }
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "storage not available"))
        }
    }

    fn load_game(&mut self) -> std::io::Result<()> {
        if let Some(file) = &self.save_file {
            let mut reader = file.reader()?;
            self.game_state = Some(GameState::load(&mut reader)?);
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "storage not available"))
        }
    }

    fn close_game(&mut self) {
        self.game_state = None;
    }

    fn remove_save(&mut self) {
        if let Some(file) = &mut self.save_file {
            let _ = file.remove();
        }
    }
}

pub struct BracketState {
    pub state: UiState,
    pub key_bindings: KeyBindings,
    pub views: ViewStack<UiState, Key, InputImpl, UiStateAction>
}

impl BracketState {
    pub fn new(graphics: GraphicLookup) -> Self {
        let save_file = get_storage(FILENAME.to_string()).map(|mut storage| {
            storage.data().file("savegame".to_string(), FileMode::Binary)
        });
        let quitter = get_quitter();
        let mut state = UiState {
            graphics: graphics,
            ui_can_quit: quitter.is_some(),
            ui_save_storage_avail: save_file.is_some(),
            show_intro_dialog: true,
            save_file: save_file,
            quitter: quitter,
            game_state: None
        };
        {
            state.show_intro_dialog = true;
            #[cfg(debug_assertions)]
            {
                state.show_intro_dialog = false;
            }
        }
        let mut new = BracketState {
            state: state,
            key_bindings: make_default_key_bindings(),
            views: ViewStack::new()
        };
        new.show_main_menu();
        new
    }

    fn show_main_menu(&mut self) {
        self.views.push(make_main_menu(
            self.state.ui_can_quit,
            self.state.ui_save_storage_avail,
            self.state.save_file.as_ref().map_or(false, |f| f.exists() )
        ));
    }

    fn handle_action(&mut self, action: UiStateAction) {
        match action {
            UiStateAction::Back => {
                self.views.pop();
            },
            UiStateAction::Quit => {
                if let Some(quitter) = &self.state.quitter {
                    quitter.quit()
                }
            },
            UiStateAction::SaveAndMainMenu => {
                let save_result =
                    if self.state.ui_save_storage_avail {
                        self.state.save_game()
                    } else {
                        Ok(())
                    };
                if let Err(e) = save_result {
                    self.views.push(make_save_error_dialog(e));
                } else {
                    self.state.close_game();
                    self.views.clear();
                    self.show_main_menu();
                }
            },
            UiStateAction::EndGameAndMainMenu => {
                self.state.remove_save();
                self.state.close_game();
                self.views.clear();
                self.show_main_menu();
            },
            UiStateAction::WinGameAndMainMenu => {
                self.state.remove_save();
                self.state.close_game();
                self.views.clear();
                self.show_main_menu();
                self.views.push(make_win_dialog());
            },
            UiStateAction::LoseGameAndMainMenu => {
                self.state.remove_save();
                self.state.close_game();
                self.views.clear();
                self.show_main_menu();
                self.views.push(make_lose_dialog());
            },
            UiStateAction::ShowHelp => {
                self.views.push(make_help_dialog(&self.key_bindings));
            },
            UiStateAction::NewGame => {
                self.state.new_game();
                self.views.clear();
                self.views.push(GameView::new(&self.key_bindings));
                if self.state.show_intro_dialog {
                    self.views.push(make_intro_dialog());
                }
            },
            UiStateAction::ContinueGame => {
                match self.state.load_game() {
                    Ok(()) => {
                        self.views.clear();
                        self.views.push(GameView::new(&self.key_bindings));
                    },
                    Err(e) => {
                        self.views.push(make_load_error_dialog(e));
                    }
                }
            },
            UiStateAction::RemoveSave => {
                self.state.remove_save();
                self.views.clear();
                self.show_main_menu();
            }
        }
    }
}

impl BracketGameState for BracketState {
    fn tick(&mut self, ctx: &mut BTerm) {
        let input = InputImpl::new(ctx, &self.key_bindings);
        let action = self.views.tick(&mut self.state, &input, ctx);
        if let Some(action) = action {
            self.handle_action(action);
        }
    }
}
