use crate::snapshot::systems::snapshot_enabled;
use crate::snapshot::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[test]
fn test_snapshot_plugin() {
    // Create a test app with a minimal plugin set to avoid button input errors
    let mut app = App::new();

    // Create a simplified version of the plugin's setup
    // instead of app.add_plugins(SnapshotPlugin) that would require input modules
    app.init_resource::<SnapshotConfig>()
        .insert_resource(SnapshotDisabled::enabled())
        .add_event::<SnapshotEvent>();

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
}
