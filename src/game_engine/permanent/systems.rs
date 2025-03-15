use bevy::prelude::*;

use crate::game_engine::turns::TurnManager;

use super::PermanentState;

/// System to update permanent state at the beginning of controller's turn
pub fn update_permanent_state(
    turn_manager: Res<TurnManager>,
    mut permanent_query: Query<&mut PermanentState>,
) {
    // This is a placeholder for now - we'll implement specific logic
    // for updating permanents each turn (removing summoning sickness, etc.)
    for mut state in permanent_query.iter_mut() {
        state.update_summoning_sickness(turn_manager.turn_number);
    }
}
