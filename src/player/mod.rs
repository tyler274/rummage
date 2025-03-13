//! Player module for managing player entities and states in the game

pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

// Re-export the core components and systems
pub use components::Player;
pub use resources::PlayerConfig;
pub use systems::{debug_draw_player_positions, spawn_players};

/// Plugin for player-related functionality
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_systems(Update, debug_draw_player_positions);
    }
}
