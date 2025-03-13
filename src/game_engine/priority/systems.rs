use crate::game_engine::stack::GameStack;
use crate::game_engine::state::GameState;
use crate::game_engine::turns::TurnManager;
use crate::player::Player;
use bevy::prelude::*;

use super::events::{NextPhaseEvent, PassPriorityEvent};
use super::resources::PrioritySystem;

/// Main system for managing priority passing and game flow
pub fn priority_system(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut game_state: ResMut<GameState>,
    stack: Res<GameStack>,
    phase: Res<crate::game_engine::Phase>,
    turn_manager: Res<TurnManager>,
) {
    // Skip if we're waiting for decisions
    if !priority.simultaneous_decision_players.is_empty() {
        return;
    }

    // If everyone has passed priority and the stack is empty
    if priority.priority_round_complete() && priority.stack_is_empty {
        // Transition to the next phase
        // Reset priority passing for next phase
        priority.reset_passing_status();

        if !priority.has_processed_phase(*phase, turn_manager.turn_number) {
            priority.mark_phase_processed(*phase, turn_manager.turn_number);
            commands.spawn_empty().insert(NextPhaseEvent);
        }
    }
    // If everyone has passed and there's something on stack, resolve top item
    else if priority.priority_round_complete() && !priority.stack_is_empty {
        // The next pending item will be resolved
        // Priority resets to active player after resolving
        let players: Vec<Entity> = game_state.turn_order.iter().copied().collect();
        let active_player = game_state.active_player;

        priority.reset_after_stack_action(&players, active_player);
    }

    // Auto-pass priority in phases that don't allow player actions
    if !phase.allows_actions() && priority.stack_is_empty {
        commands.spawn_empty().insert(PassPriorityEvent {
            player: priority.priority_player,
        });
    }
}

/// System for handling priority passing events
pub fn priority_passing_system(
    _commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut game_stack: ResMut<GameStack>,
    mut pass_events: EventReader<PassPriorityEvent>,
    _players: Query<Entity, With<Player>>,
    _time: Res<Time>,
) {
    // Process all pending pass priority events
    for event in pass_events.read() {
        // If the player doesn't have priority, ignore the event
        if event.player != priority.priority_player {
            continue;
        }

        // Mark that this player has passed priority
        if let Some(passed) = priority.has_priority_passed.get_mut(&event.player) {
            *passed = true;
        }

        // Pass priority to the next player
        priority.pass_priority();

        // Update stack empty status
        priority.set_stack_empty(game_stack.items.is_empty());
    }
}
