pub mod components;
pub mod logo;
pub mod main_menu;
pub mod pause_menu;
pub mod state_management;
pub mod visibility;

pub use components::*;
pub use logo::*;
pub use main_menu::background as main_menu_background;
pub use main_menu::buttons;
pub use main_menu::interactions as main_menu_interactions;
pub use main_menu::setup as main_menu_setup;

pub use pause_menu::buttons as pause_menu_buttons;
pub use pause_menu::input_handler as pause_menu_input;
pub use pause_menu::interactions as pause_menu_interactions;
pub use pause_menu::setup as pause_menu_setup;

pub use state_management::*;
pub use visibility::*;

// Reexport key functionality
pub use logo::setup_main_menu_star;
pub use main_menu::handle_main_menu_interactions;
pub use main_menu::setup_main_menu;
pub use pause_menu::setup::setup_pause_menu;
