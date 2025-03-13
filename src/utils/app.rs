use bevy::app::AppExit;
use bevy::prelude::*;

/// Handles application exit events
pub fn handle_exit(mut exit_events: EventReader<AppExit>) {
    for _exit_event in exit_events.read() {
        info!("Received exit event, cleaning up...");
    }
}
