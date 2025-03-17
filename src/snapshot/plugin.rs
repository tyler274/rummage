use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

use crate::snapshot::resources::{
    SnapshotConfig, SnapshotDebugState, SnapshotDisabled, SnapshotEvent,
};
use crate::snapshot::systems::{
    capture_replay_at_point, check_snapshot_key_input, handle_snapshot_events,
    process_pending_snapshots, snapshot_enabled, take_replay_snapshot, take_save_game_snapshot,
};

// Note: We keep the ScheduleLabel for testing purposes, but don't use it in production code
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet, ScheduleLabel)]
pub struct SnapshotExclusiveSet;

/// Plugin for camera snapshot functionality
pub struct SnapshotPlugin;

impl Plugin for SnapshotPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing SnapshotPlugin");
        app.init_resource::<SnapshotConfig>()
            .init_resource::<SnapshotDebugState>()
            .insert_resource(SnapshotDisabled::enabled())
            .add_event::<SnapshotEvent>();

        #[cfg(feature = "snapshot")]
        {
            info!("Setting up snapshot feature components");

            // Add the snapshot processing system to PostUpdate to ensure it runs after UI systems
            // but avoid conflicts with other systems that might access the same components
            // We only use the non-exclusive version for production code
            app.add_systems(
                PostUpdate,
                process_pending_snapshots.run_if(snapshot_enabled),
            );
            debug!("Added process_pending_snapshots to PostUpdate schedule");

            // Keep the regular systems in Update
            app.add_systems(
                Update,
                (
                    handle_snapshot_events.run_if(snapshot_enabled),
                    check_snapshot_key_input.run_if(snapshot_enabled),
                ),
            );
            debug!("Added regular snapshot systems to Update schedule");

            // Add systems for save/load integration (these use run conditions to check requirements)
            app.add_systems(
                Update,
                (
                    take_save_game_snapshot.run_if(snapshot_enabled),
                    take_replay_snapshot.run_if(snapshot_enabled),
                    capture_replay_at_point.run_if(snapshot_enabled),
                ),
            );
            debug!("Added save/load integration systems to Update schedule");
        }
        info!("SnapshotPlugin initialization complete");
    }
}
