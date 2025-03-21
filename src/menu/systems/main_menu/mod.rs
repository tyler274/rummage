pub mod background;
pub mod buttons;
pub mod interactions;
pub mod setup;
pub mod states;

// Remove unused imports
// Keep only specific functions for external use
pub use interactions::handle_main_menu_interactions;
pub use setup::setup_main_menu;
