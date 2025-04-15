mod cleanup;
pub mod components;
// pub mod interactions; // Removed - likely in systems
pub mod plugin;
// pub mod setup; // Removed - likely in systems
pub mod systems;

// Export the plugin for use in the menu system
pub use plugin::MainMenuPlugin;

// Export components needed by other modules
pub use cleanup::{/* cleanup_main_menu, */ cleanup_main_menu_music_on_settings_enter};
