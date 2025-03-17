pub mod components;
pub mod examples;
pub mod plugin;
pub mod resources;
pub mod systems;

// Include tests module when running tests but not in normal builds
#[cfg(test)]
pub mod tests;

// Re-export key types for convenience
pub use components::{CameraSnapshot, SaveGameSnapshot, SnapshotSettings};
pub use plugin::SnapshotPlugin;
pub use resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};
