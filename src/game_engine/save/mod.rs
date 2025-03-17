// Internal modules
pub mod data;
pub mod events;
pub mod plugin;
pub mod resources;
pub mod systems;

#[cfg(test)]
pub mod tests;

// Re-export public API
pub use events::*;
pub use plugin::SaveLoadPlugin;
pub use resources::*;

// Re-export data types
pub use data::*;

// Re-export specific systems needed elsewhere
pub use systems::setup_save_system;
