use bevy::prelude::*;

/// Marker component for entities that need a snapshot rendered
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct CameraSnapshot {
    /// Whether the snapshot has been taken (to prevent duplicates)
    pub taken: bool,
}

/// Component to specify snapshot settings
#[derive(Component, Debug, Clone)]
pub struct SnapshotSettings {
    /// Filename to save the snapshot
    ///
    /// This field is used by snapshot handling systems to determine where to save the image
    #[allow(dead_code)]
    pub filename: String,

    /// Whether to include debug information in the snapshot
    pub include_debug: bool,

    /// Optional description to include in the filename
    ///
    /// Used for creating more descriptive snapshot filenames
    #[allow(dead_code)]
    pub description: Option<String>,

    /// Whether to save automatically - if false, will only create the RenderTarget
    ///
    /// Currently all snapshots are saved automatically by default
    #[allow(dead_code)]
    pub auto_save: bool,
}
