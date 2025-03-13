use bevy::prelude::*;

/// Resource to configure snapshot settings globally
#[derive(Resource, Debug, Clone)]
pub struct SnapshotConfig {
    /// Directory to save snapshots to (default: workspace root)
    ///
    /// This path will be used for saving snapshots when screenshot functionality is fully implemented
    pub output_dir: String,
    /// Prefix for snapshot filenames
    pub filename_prefix: String,
    /// Whether to include timestamp in filenames
    pub include_timestamp: bool,
    /// Resolution of snapshots (default: 1920x1080)
    ///
    /// This will be used when screenshot functionality is fully implemented to specify custom resolutions
    pub resolution: Vec2,
    /// Whether to enable automatic snapshots on particular events
    ///
    /// This will be used to trigger snapshots on specific game events without manual intervention
    pub auto_snapshot_enabled: bool,
    /// Whether to capture debug visualization in snapshots
    pub include_debug_by_default: bool,
}

impl SnapshotConfig {
    /// Create a new SnapshotConfig with custom defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the output directory for snapshots
    pub fn with_output_dir(mut self, dir: impl Into<String>) -> Self {
        self.output_dir = dir.into();
        self
    }

    /// Set the filename prefix for snapshots
    pub fn with_filename_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.filename_prefix = prefix.into();
        self
    }

    /// Set whether to include timestamps in filenames
    pub fn with_timestamp(mut self, include: bool) -> Self {
        self.include_timestamp = include;
        self
    }

    /// Set the resolution for snapshots
    pub fn with_resolution(mut self, width: f32, height: f32) -> Self {
        self.resolution = Vec2::new(width, height);
        self
    }

    /// Set whether to enable automatic snapshots on particular events
    pub fn with_auto_snapshot(mut self, enabled: bool) -> Self {
        self.auto_snapshot_enabled = enabled;
        self
    }

    /// Set whether to include debug visualization in snapshots by default
    pub fn with_debug_visualization(mut self, include: bool) -> Self {
        self.include_debug_by_default = include;
        self
    }
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

impl SnapshotEvent {
    /// Create a new SnapshotEvent
    pub fn new() -> Self {
        Self {
            camera_entity: None,
            filename: None,
            description: None,
            include_debug: None,
        }
    }

    /// Set the camera entity to use for the snapshot
    pub fn with_camera(mut self, entity: Entity) -> Self {
        self.camera_entity = Some(entity);
        self
    }

    /// Set the filename for the snapshot
    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    /// Set a description to include in the filename
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set whether to include debug visualization
    pub fn with_debug(mut self, include_debug: bool) -> Self {
        self.include_debug = Some(include_debug);
        self
    }
}

/// Resource to globally disable snapshot functionality
/// This is useful for debugging when snapshot functionality might be causing panics
#[derive(Resource, Debug, Clone, Copy)]
pub struct SnapshotDisabled(pub bool);

impl SnapshotDisabled {
    /// Create a new SnapshotDisabled resource
    pub fn new(disabled: bool) -> Self {
        Self(disabled)
    }

    /// Create a resource with snapshots enabled
    pub fn enabled() -> Self {
        Self(false)
    }

    /// Create a resource with snapshots disabled
    pub fn disabled() -> Self {
        Self(true)
    }

    /// Check if snapshots are enabled
    pub fn is_enabled(&self) -> bool {
        !self.0
    }

    /// Check if snapshots are disabled
    pub fn is_disabled(&self) -> bool {
        self.0
    }
}

impl Default for SnapshotDisabled {
    fn default() -> Self {
        // Default to enabled (not disabled)
        Self(false)
    }
}

/// Resource to track snapshot system debug state to prevent log spam
#[derive(Resource, Default, Debug)]
pub struct SnapshotDebugState {
    /// Last reported event count in handle_snapshot_events
    pub last_event_count: usize,
    /// Last reported pending snapshot count in process_pending_snapshots
    pub last_pending_count: usize,
    /// Whether the last snapshot system run had any activity
    pub had_activity: bool,
}

impl SnapshotDebugState {
    /// Check if event processing state has changed
    pub fn has_events_changed(&mut self, current_event_count: usize) -> bool {
        let changed = self.last_event_count != current_event_count;
        self.last_event_count = current_event_count;
        self.had_activity = self.had_activity || changed;
        changed
    }

    /// Check if pending snapshot processing state has changed
    pub fn has_pending_changed(&mut self, current_pending_count: usize) -> bool {
        let changed = self.last_pending_count != current_pending_count;
        self.last_pending_count = current_pending_count;
        self.had_activity = self.had_activity || changed;
        changed
    }

    /// Reset the activity flag at the end of a frame
    #[allow(dead_code)]
    pub fn reset_activity(&mut self) -> bool {
        let had_activity = self.had_activity;
        self.had_activity = false;
        had_activity
    }
}
