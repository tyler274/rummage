pub mod background;
pub mod buttons;
pub mod interactions;
pub mod setup;
pub mod states;

pub use background::*;
pub use buttons::*;
pub use states::*;

// Reexport key functions for easier access
pub use background::update_background;
pub use interactions::handle_main_menu_interactions;
pub use setup::setup_main_menu;
