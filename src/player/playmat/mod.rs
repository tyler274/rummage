//! Player playmat system for spawning and managing the player's board layout
//! as defined in the playmat documentation.

// Declare modules
pub mod battlefield;
pub mod command;
mod components;
pub mod exile;
pub mod graveyard;
pub mod hand;
pub mod library;
// Make plugin module public
pub mod plugin;
mod resources;
mod systems;
mod zones;

// Re-export necessary items publicly
pub use components::{PlayerPlaymat, PlaymatZone};
// Remove the specific re-export for the plugin as it's now accessible via the public module path
// pub use plugin::PlayerPlaymatPlugin;
// Only export resources/systems actually needed outside this parent module
// pub use resources::{CurrentPhaseLayout, ZoneFocusState};
pub use systems::spawn_player_playmat; // Assuming this is called from outside

// No other code should be in this file.
