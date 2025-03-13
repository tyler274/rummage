pub mod components;
pub mod resources;
pub mod systems;

// Re-export the core components and systems
pub use components::Player;
pub use systems::spawn::spawn_players;
