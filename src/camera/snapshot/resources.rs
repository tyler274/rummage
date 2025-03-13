use bevy::prelude::*;

/// Resource to configure snapshot settings globally
#[derive(Resource, Debug, Clone)]
pub struct SnapshotConfig {
    /// Directory to save snapshots to (default: workspace root)
    ///
    /// This path will be used for saving snapshots when screenshot functionality is fully implemented
    #[allow(dead_code)]
    pub output_dir: String,
    /// Prefix for snapshot filenames
    pub filename_prefix: String,
    /// Whether to include timestamp in filenames
    pub include_timestamp: bool,
    /// Resolution of snapshots (default: 1920x1080)
    ///
    /// This will be used when screenshot functionality is fully implemented to specify custom resolutions
    #[allow(dead_code)]
    pub resolution: Vec2,
    /// Whether to enable automatic snapshots on particular events
    ///
    /// This will be used to trigger snapshots on specific game events without manual intervention
    #[allow(dead_code)]
    pub auto_snapshot_enabled: bool,
    /// Whether to capture debug visualization in snapshots
    pub include_debug_by_default: bool,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            output_dir: ".".to_string(),
            filename_prefix: "rummage_snapshot".to_string(),
            include_timestamp: true,
            resolution: Vec2::new(1920.0, 1080.0),
            auto_snapshot_enabled: false,
            include_debug_by_default: true,
        }
    }
}

/// Event to trigger a camera snapshot
#[derive(Event, Debug, Clone)]
pub struct SnapshotEvent {
    /// Optional camera entity to use (if None, use the first GameCamera)
    pub camera_entity: Option<Entity>,
    /// Optional filename (if None, use the default naming scheme)
    pub filename: Option<String>,
    /// Optional description to add to the filename
    pub description: Option<String>,
    /// Whether to include debug visualization
    pub include_debug: Option<bool>,
}
