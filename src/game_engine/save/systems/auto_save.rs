use bevy::prelude::*;

use crate::game_engine::save::events::*;
use crate::game_engine::save::resources::*;
use crate::game_engine::state::GameState;

/// Handles automatic saving of the game state at regular intervals
pub fn handle_auto_save(
    time: Res<Time>,
    mut auto_save_tracker: ResMut<AutoSaveTracker>,
    config: Res<SaveConfig>,
    mut event_writer: EventWriter<SaveGameEvent>,
) {
    // Skip if auto-save is disabled
    if !config.auto_save_enabled {
        return;
    }

    auto_save_tracker.time_since_last_save += time.delta_secs();

    // Check if it's time for an auto-save
    if auto_save_tracker.time_since_last_save >= config.auto_save_interval_seconds {
        info!("Auto-saving game...");

        // Generate a timestamp-based save name
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();

        event_writer.send(SaveGameEvent {
            slot_name: format!("auto_save_{}", now.as_secs()),
            description: Some("Auto save".to_string()),
            with_snapshot: true,
        });

        // Reset the timer
        auto_save_tracker.time_since_last_save = 0.0;
    }
}

/// System to automatically capture game state for history
pub fn auto_capture_history(
    mut event_writer: EventWriter<CaptureHistoryEvent>,
    mut auto_save_tracker: ResMut<AutoSaveTracker>,
    game_state: Res<GameState>,
    config: Res<SaveConfig>,
) {
    // Check if turn has changed
    if auto_save_tracker.time_since_last_save >= config.auto_save_interval_seconds / 2.0
        && game_state.turn_number != auto_save_tracker.last_turn_checkpoint
    {
        // Capture state at the beginning of each turn
        event_writer.send(CaptureHistoryEvent);

        // Update last turn
        auto_save_tracker.last_turn_checkpoint = game_state.turn_number;
    }
}
