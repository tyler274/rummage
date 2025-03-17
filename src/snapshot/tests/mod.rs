mod components_tests;
mod fixtures;
mod integration_tests;
mod plugin_tests;
mod resources_tests;
mod systems_tests;

// This test ensures that the re-exported items from the snapshot module are used
#[cfg(test)]
mod usage_tests {
    use crate::snapshot::{
        CameraSnapshot, SnapshotConfig, SnapshotDisabled, SnapshotEvent, SnapshotPlugin,
        SnapshotSettings,
    };
    use bevy::prelude::*;

    #[test]
    fn test_reexported_items() {
        // Use all re-exported items to prevent "unused imports" warnings

        // Use CameraSnapshot
        let snapshot = CameraSnapshot::new();
        assert!(!snapshot.taken);

        let taken = CameraSnapshot::taken();
        assert!(taken.taken);

        // Use SnapshotSettings
        let settings = SnapshotSettings::new("test.png").with_filename("new_name.png");
        assert_eq!(settings.filename, "new_name.png");

        // Use SnapshotConfig
        let config = SnapshotConfig::new()
            .with_output_dir("test_dir")
            .with_filename_prefix("prefix")
            .with_timestamp(false)
            .with_resolution(800.0, 600.0)
            .with_auto_snapshot(true)
            .with_debug_visualization(false);
        assert_eq!(config.output_dir, "test_dir");
        assert_eq!(config.resolution, Vec2::new(800.0, 600.0));
        assert_eq!(config.auto_snapshot_enabled, true);

        // Use SnapshotDisabled
        let disabled = SnapshotDisabled::new(true);
        assert!(disabled.is_disabled());

        // Use SnapshotEvent
        let event = SnapshotEvent::new().with_filename("event.png");
        assert_eq!(event.filename, Some("event.png".to_string()));

        // Use SnapshotPlugin
        let _plugin = SnapshotPlugin::default();
    }
}
