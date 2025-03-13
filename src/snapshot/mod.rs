pub mod components;
pub mod resources;
pub mod systems;
pub mod plugin;

// Re-export key types for convenience
pub use components::{CameraSnapshot, SnapshotSettings};
pub use resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};
pub use plugin::SnapshotPlugin; 