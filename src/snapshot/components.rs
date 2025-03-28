use bevy::prelude::*;

/// Marker component for entities that need a snapshot rendered
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct CameraSnapshot {
    /// Whether the snapshot has been taken (to prevent duplicates)
    pub taken: bool,
}

impl CameraSnapshot {
    /// Create a new CameraSnapshot
    pub fn new() -> Self {
        Self { taken: false }
    }

    /// Create a CameraSnapshot that has already been taken
    pub fn taken() -> Self {
        Self { taken: true }
    }
}

/// Component to specify snapshot settings
#[derive(Component, Debug, Clone)]
pub struct SnapshotSettings {
    /// Filename to save the snapshot
    ///
    /// This field is used by snapshot handling systems to determine where to save the image
    pub filename: String,

    /// Whether to include debug information in the snapshot
    pub include_debug: bool,

    /// Optional description to include in the filename
    ///
    /// Used for creating more descriptive snapshot filenames
    pub description: Option<String>,

    /// Whether to save automatically - if false, will only create the RenderTarget
    ///
    /// Currently all snapshots are saved automatically by default
    pub auto_save: bool,
}

impl SnapshotSettings {
    /// Create a new SnapshotSettings with default values
    pub fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            include_debug: true,
            description: None,
            auto_save: true,
        }
    }

    /// Set the filename
    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = filename.into();
        self
    }

    /// Set whether to include debug information
    pub fn with_debug(mut self, include_debug: bool) -> Self {
        self.include_debug = include_debug;
        self
    }

    /// Set a description to include in the filename
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set whether to save automatically
    pub fn with_auto_save(mut self, auto_save: bool) -> Self {
        self.auto_save = auto_save;
        self
    }
}

/// Component to link a snapshot to a specific saved game
#[derive(Component, Debug, Clone)]
pub struct SaveGameSnapshot {
    /// The save slot name this snapshot is associated with
    #[allow(dead_code)]
    pub slot_name: String,

    /// The turn number in the saved game
    #[allow(dead_code)]
    pub turn_number: u32,

    /// Optional timestamp of when the snapshot was taken
    pub timestamp: Option<i64>,

    /// Optional description of the game state
    pub description: Option<String>,
}

impl SaveGameSnapshot {
    /// Create a new SaveGameSnapshot linked to a specific save slot
    pub fn new(slot_name: impl Into<String>, turn_number: u32) -> Self {
        Self {
            slot_name: slot_name.into(),
            turn_number,
            timestamp: None,
            description: None,
        }
    }

    /// Add a description to the snapshot
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the timestamp for the snapshot
    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
}
