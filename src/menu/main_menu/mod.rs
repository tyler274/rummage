mod cleanup;
pub mod components;
mod plugin;
mod systems;

// Export the plugin for use in the menu system
pub use plugin::MainMenuPlugin;

// Export components needed by other modules
