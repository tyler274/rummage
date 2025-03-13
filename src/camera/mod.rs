/// Camera module provides camera management systems and components for the game.
///
/// Important note for new contributors:
/// Bevy 0.15 has consolidated many camera types into the Camera component.
/// Instead, spawn camera entities with individual components:
pub mod components;
pub mod config;
pub mod state;
pub mod systems;
// mod tests; // Will be added when tests are implemented

// snapshot module has been moved to its own top-level module at src/snapshot

use bevy::prelude::*;

use crate::camera::config::CameraConfig;
use crate::camera::systems::{
    camera_movement, debug_draw_card_positions, handle_window_resize, set_initial_zoom,
    setup_camera,
};
#[cfg(feature = "snapshot")]
use crate::snapshot::SnapshotPlugin;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraConfig>();

        #[cfg(feature = "snapshot")]
        app.add_plugins(SnapshotPlugin);

        app.add_systems(Startup, setup_camera)
            .add_systems(PostStartup, set_initial_zoom)
            .add_systems(
                Update,
                (
                    handle_window_resize,
                    camera_movement,
                    debug_draw_card_positions,
                ),
            );
    }
}

// Re-export key items for convenience
pub use state::CameraPanState;
