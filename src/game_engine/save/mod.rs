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
#[allow(unused)]
pub use resources::*;

// Re-export data types
#[allow(unused)]
pub use data::*;
