mod dialog;
mod fancy_logo_menu;
mod main_menu;
mod game_view;
mod intro_dialog;
mod save_error_dialog;
mod load_error_dialog;
mod cell_info;

pub use dialog::{DialogView, DialogChoice};
pub use fancy_logo_menu::{FancyLogoMenuView, FancyLogoMenuChoice};
pub use main_menu::make_main_menu;
pub use game_view::GameView;
pub use intro_dialog::make_intro_dialog;
pub use save_error_dialog::make_save_error_dialog;
pub use load_error_dialog::make_load_error_dialog;
