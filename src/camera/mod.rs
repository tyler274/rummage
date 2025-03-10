mod components;
mod config;
mod state;
mod systems;

pub use components::*;
pub use config::*;
pub use state::*;
pub use systems::*;

/// Camera management for the game's 2D view.
///
/// This module provides functionality for:
/// - Camera setup and configuration
/// - Camera movement (WASD/Arrow keys and middle mouse drag)
/// - Mouse wheel zoom with smooth interpolation
/// - Viewport management
/// - Coordinate space transformations
///
/// # Important Note for Bevy 0.15.x Compatibility
/// As of Bevy 0.15.x, Camera2dBundle and other *Bundle types are deprecated.
/// Instead, spawn camera entities with individual components:
/// ```ignore
/// // OLD (deprecated):
/// commands.spawn(Camera2dBundle::default());
///
/// // NEW (correct):
/// commands.spawn((
///     Camera2d::default(),
///     Camera::default(),
///     Transform::default(),
///     GlobalTransform::default(),
///     Visibility::default(),
///     InheritedVisibility::default(),
///     ViewVisibility::default(),
/// ));
/// ```
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use rummage::camera::{setup_camera, camera_movement, CameraConfig};
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .insert_resource(CameraConfig::default())
///         .add_systems(Startup, setup_camera)
///         .add_systems(Update, camera_movement)
///         .run();
/// }
/// ``` 