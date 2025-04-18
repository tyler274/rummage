//! Player related components, systems, and resources.

pub mod components;
pub mod playmat;
pub mod resources;
pub mod systems;
// pub mod zone; // This seems unused or defined elsewhere

use bevy::prelude::*;

// Import and re-export common player components and systems
pub use components::Player;
pub use playmat::plugin::PlayerPlaymatPlugin;
pub use resources::PlayerConfig;
pub use systems::debug::{PlayerPositionTracker, debug_draw_player_positions};

/// Plugin for player-related functionality
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .init_resource::<PlayerPositionTracker>()
            .add_systems(FixedUpdate, debug_draw_player_positions)
            .add_plugins(PlayerPlaymatPlugin);
    }
}
