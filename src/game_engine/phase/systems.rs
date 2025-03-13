use crate::game_engine::priority::NextPhaseEvent;
use crate::game_engine::priority::PrioritySystem;
use crate::game_engine::state::GameState;
use crate::game_engine::turns::TurnManager;
use crate::player::Player;
use bevy::prelude::*;

use super::types::{BeginningStep, CombatStep, EndingStep, Phase, PostcombatStep, PrecombatStep};

/// System for handling phase transitions
pub fn phase_transition_system(
    mut commands: Commands,
    mut phase: ResMut<Phase>,
    mut turn_manager: ResMut<TurnManager>,
    mut game_state: ResMut<GameState>,
    mut priority_system: ResMut<PrioritySystem>,
    mut next_phase_events: EventReader<NextPhaseEvent>,
    player_query: Query<Entity, With<Player>>,
) {
    for _ in next_phase_events.read() {
        advance_phase(
            &mut commands,
            &mut phase,
            &mut turn_manager,
            &mut game_state,
            &mut priority_system,
            &player_query,
        );
    }
}

/// Helper function to advance to the next phase
fn advance_phase(
    _commands: &mut Commands,
    phase: &mut Phase,
    turn_manager: &mut TurnManager,
    game_state: &mut GameState,
    priority_system: &mut PrioritySystem,
    player_query: &Query<Entity, With<Player>>,
) {
    // Store the old phase for reference
    let old_phase = *phase;

    // Advance to the next phase
    *phase = phase.next();

    // Handle phase-specific logic
    match *phase {
        // Beginning of a new turn
        Phase::Beginning(BeginningStep::Untap) => {
            // This means we're at the start of a new turn (reached by advancing from cleanup)
            if matches!(old_phase, Phase::Ending(EndingStep::Cleanup)) {
                // Advance to the next player's turn
                turn_manager.advance_turn();

                // Update active player
                game_state.active_player = turn_manager.active_player;

                // Reset per-turn state tracking
                game_state.reset_turn_tracking();

                // Reset priority to the new active player
                let players: Vec<Entity> = player_query.iter().collect();
                priority_system.initialize(&players, game_state.active_player);

                info!(
                    "Turn {}: Player {:?}'s turn",
                    turn_manager.turn_number, game_state.active_player
                );
            }
        }
        Phase::Precombat(PrecombatStep::Main) => {
            // First main phase begins - reset main phase tracking
            game_state.main_phase_action_taken = false;
        }
        Phase::Combat(CombatStep::Beginning) => {
            // Beginning of combat phase
        }
        Phase::Postcombat(PostcombatStep::Main) => {
            // Second main phase begins - reset main phase tracking
            game_state.main_phase_action_taken = false;
        }
        Phase::Ending(EndingStep::End) => {
            // End step - trigger "at end of turn" effects
        }
        Phase::Ending(EndingStep::Cleanup) => {
            // Cleanup step - discard to hand size, remove damage, etc.
            // This is typically the last step before a new turn
        }
        _ => {}
    }

    // Log the phase transition
    match *phase {
        Phase::Beginning(step) => {
            let step_name = match step {
                BeginningStep::Untap => "Untap",
                BeginningStep::Upkeep => "Upkeep",
                BeginningStep::Draw => "Draw",
            };
            info!(
                "Phase: Beginning ({}) - Turn {}",
                step_name, turn_manager.turn_number
            );
        }
        Phase::Precombat(_) => {
            info!("Phase: Precombat Main - Turn {}", turn_manager.turn_number);
        }
        Phase::Combat(step) => {
            let step_name = match step {
                CombatStep::Beginning => "Beginning",
                CombatStep::DeclareAttackers => "Declare Attackers",
                CombatStep::DeclareBlockers => "Declare Blockers",
                CombatStep::CombatDamage => "Combat Damage",
                CombatStep::End => "End",
            };
            info!(
                "Phase: Combat ({}) - Turn {}",
                step_name, turn_manager.turn_number
            );
        }
        Phase::Postcombat(_) => {
            info!("Phase: Postcombat Main - Turn {}", turn_manager.turn_number);
        }
        Phase::Ending(step) => {
            let step_name = match step {
                EndingStep::End => "End",
                EndingStep::Cleanup => "Cleanup",
            };
            info!(
                "Phase: Ending ({}) - Turn {}",
                step_name, turn_manager.turn_number
            );
        }
    }

    // Reset priority for the new phase
    let players: Vec<Entity> = player_query.iter().collect();
    priority_system.reset_passing_status();
    priority_system.reset_after_stack_action(&players, game_state.active_player);
}
