use bevy::prelude::*;

use crate::snapshot::resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};
use crate::snapshot::systems::{
    check_snapshot_key_input, handle_snapshot_events, process_pending_snapshots_exclusive,
    snapshot_enabled,
};

/// Plugin for camera snapshot functionality
pub struct SnapshotPlugin;

impl Plugin for SnapshotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnapshotConfig>()
            .insert_resource(SnapshotDisabled::enabled())
            .add_event::<SnapshotEvent>();

        #[cfg(feature = "snapshot")]
        app.add_systems(
            Update,
            (
                handle_snapshot_events.run_if(snapshot_enabled),
                process_pending_snapshots_exclusive.run_if(snapshot_enabled),
                check_snapshot_key_input.run_if(snapshot_enabled),
            ),
        );
    }
}
