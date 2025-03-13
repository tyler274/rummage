pub mod components;
pub mod examples;
pub mod plugin;
pub mod resources;
pub mod systems;

// Include tests module when running tests but not in normal builds
#[cfg(test)]
pub mod tests;

/// Snapshot functionality provides a way to capture camera views for debugging,
/// testing, or creating screenshots.
///
/// # Example
/// ```rust,ignore
/// use bevy::prelude::*;
/// use rummage::snapshot::{CameraSnapshot, SnapshotSettings, SnapshotConfig, SnapshotDisabled, SnapshotEvent};
///
/// fn setup_snapshot_camera(mut commands: Commands) {
///     // Create a camera with snapshot capability
///     commands.spawn((
///         Camera2d,
///         CameraSnapshot::new(),
///         SnapshotSettings::new("camera_view.png")
///             .with_debug(true)
///             .with_description("Debug view")
///     ));
/// }
///
/// fn trigger_snapshot(mut events: EventWriter<SnapshotEvent>) {
///     // Trigger a snapshot with custom settings
///     events.send(SnapshotEvent::new()
///         .with_filename("custom_snapshot.png")
///         .with_description("Custom snapshot"));
/// }
/// ```
// Re-export key types for convenience
pub use components::{CameraSnapshot, SnapshotSettings};
pub use plugin::SnapshotPlugin;
pub use resources::{SnapshotConfig, SnapshotDebugState, SnapshotDisabled, SnapshotEvent};
