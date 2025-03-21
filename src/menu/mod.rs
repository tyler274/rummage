pub mod camera;
pub mod cleanup;
pub mod components;
pub mod input_blocker;
pub mod logo;
pub mod main_menu;
pub mod pause_menu;
pub mod plugin;
pub mod save_load;
pub mod settings;
pub mod stars;
pub mod state;
pub mod state_transitions;
pub mod styles;
mod systems;
pub mod ui;

// Add the missing modules
pub mod credits;
pub mod deck;
pub mod main;

pub use plugin::MenuPlugin;
pub use state::*;
pub use ui::{MenuVisibilityState, PreviousWindowSize};
