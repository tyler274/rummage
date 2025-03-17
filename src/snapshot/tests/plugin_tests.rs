use crate::game_engine::save::SaveGameEvent;
use crate::game_engine::save::StepReplayEvent;
use crate::snapshot::plugin::SnapshotPlugin;
use crate::snapshot::systems::snapshot_enabled;
use crate::snapshot::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[test]
fn test_snapshot_plugin() {
    // Create a test app with a minimal plugin set to avoid button input errors
    let mut app = App::new();

    // Add necessary events and resources for the snapshot systems
    app.add_event::<StepReplayEvent>();
    app.add_event::<SaveGameEvent>();
    app.init_resource::<ButtonInput<KeyCode>>();
    app.init_resource::<Time>();

    // Add the SnapshotPlugin with custom configuration
    let plugin = SnapshotPlugin::new()
        .with_snapshots_enabled(true)
        .with_config(
            SnapshotConfig::new()
                .with_output_dir("test_output")
                .with_debug_visualization(true),
        );

    // Add the plugin directly
    app.add_plugins(plugin);

    // Check that the expected resources exist
    assert!(app.world().contains_resource::<SnapshotConfig>());
    assert!(app.world().contains_resource::<SnapshotDisabled>());
    assert!(app.world().contains_resource::<Events<SnapshotEvent>>());

    // Test with snapshots enabled (default)
    let result = Arc::new(Mutex::new(false));
    let result_clone = result.clone();

    app.add_systems(Update, move |res: Res<SnapshotDisabled>| {
        *result_clone.lock().unwrap() = snapshot_enabled(res);
    });
    app.update();
    assert!(
        *result.lock().unwrap(),
        "Snapshots should be enabled by default"
    );

    // Test with snapshots disabled
    app.insert_resource(SnapshotDisabled::disabled());
    app.update();
    assert!(
        !*result.lock().unwrap(),
        "Snapshots should be disabled after setting SnapshotDisabled::disabled()"
    );

    // Test creating a plugin with snapshots initially disabled
    let mut app2 = App::new();

    // Add necessary events and resources for the snapshot systems
    app2.add_event::<StepReplayEvent>();
    app2.add_event::<SaveGameEvent>();
    app2.init_resource::<ButtonInput<KeyCode>>();
    app2.init_resource::<Time>();

    let disabled_plugin = SnapshotPlugin::new().with_snapshots_enabled(false);

    // Add the plugin directly
    app2.add_plugins(disabled_plugin);

    // Verify snapshots are disabled
    let disabled = app2.world().resource::<SnapshotDisabled>();
    assert!(
        disabled.is_disabled(),
        "Snapshots should be initially disabled"
    );
}
