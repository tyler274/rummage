use crate::card::{Card, NoUntapCondition, NoUntapEffect, PermanentState};
use crate::game_engine::phase::types::{BeginningStep, EndingStep, Phase};
use crate::game_engine::turns::{
    PermanentController, TurnEndEvent, TurnEventTracker, TurnManager, TurnStartEvent,
};
use crate::player::Player;
use bevy::prelude::*;

/// System to handle the start of a new turn
pub fn handle_turn_start(
    _commands: Commands,
    phase: Res<Phase>,
    _player_query: Query<&Player>,
    mut turn_start_events: EventWriter<TurnStartEvent>,
    turn_manager: Res<TurnManager>,
    mut event_tracker: Local<TurnEventTracker>,
) {
    // Only trigger at the beginning of the untap step
    if *phase == Phase::Beginning(BeginningStep::Untap) {
        // Check if we've already processed this turn
        if event_tracker.turn_start_processed
            && event_tracker.last_processed_turn == turn_manager.turn_number
        {
            return;
        }

        // Create a turn start event
        turn_start_events.send(TurnStartEvent {
            player: turn_manager.active_player,
            turn_number: turn_manager.turn_number,
        });

        // Mark as processed
        event_tracker.turn_start_processed = true;
        event_tracker.last_processed_turn = turn_manager.turn_number;

        info!(
            "Turn {} started for player {:?}",
            turn_manager.turn_number, turn_manager.active_player
        );
    } else {
        // If we're not in the untap step, reset the tracker for the next turn
        event_tracker.turn_start_processed = false;
    }
}

/// System to handle the end of a turn
pub fn handle_turn_end(
    _commands: Commands,
    phase: Res<Phase>,
    _player_query: Query<&Player>,
    mut turn_end_events: EventWriter<TurnEndEvent>,
    turn_manager: Res<TurnManager>,
    mut event_tracker: Local<TurnEventTracker>,
) {
    // Only trigger at the beginning of the end step
    if *phase == Phase::Ending(EndingStep::End) {
        // Check if we've already processed the end of this turn
        if event_tracker.turn_end_processed
            && event_tracker.last_processed_turn == turn_manager.turn_number
        {
            return;
        }

        // Create a turn end event
        turn_end_events.send(TurnEndEvent {
            player: turn_manager.active_player,
            turn_number: turn_manager.turn_number,
        });

        // Mark as processed
        event_tracker.turn_end_processed = true;
        event_tracker.last_processed_turn = turn_manager.turn_number;

        info!(
            "Turn {} ended for player {:?}",
            turn_manager.turn_number, turn_manager.active_player
        );
    } else {
        // If we're not in the end step, reset the tracker for the next turn
        event_tracker.turn_end_processed = false;
    }
}

/// System that handles untapping permanents during the untap step
/// This system considers special effects that prevent untapping, like NoUntapEffect
pub fn handle_untap_step(
    mut card_query: Query<
        (
            Entity,
            &mut PermanentState,
            Option<&NoUntapEffect>,
            Option<&PermanentController>,
        ),
        With<Card>,
    >,
    turn_manager: Res<TurnManager>,
    phase: Res<Phase>,
    player_query: Query<&Player>,
    mut event_tracker: Local<TurnEventTracker>,
) {
    // Only process during untap step
    if *phase != Phase::Beginning(BeginningStep::Untap) {
        // Reset the untap step tracker if we're not in the untap step
        event_tracker.untap_step_processed = false;
        return;
    }

    // Check if we've already processed the untap step for this turn
    if event_tracker.untap_step_processed
        && event_tracker.last_processed_turn == turn_manager.turn_number
    {
        return;
    }

    let active_player_entity = turn_manager.get_active_player();
    info!(
        "Processing untap step for player: {:?}",
        active_player_entity
    );

    // Get the current turn number
    let current_turn = turn_manager.turn_number;

    for (entity, mut permanent_state, no_untap_effect, controller) in card_query.iter_mut() {
        // Update summoning sickness regardless of untap restrictions
        permanent_state.update_summoning_sickness(current_turn);

        // Only untap permanents controlled by the active player
        if controller.map_or(false, |c| c.player != active_player_entity) {
            continue;
        }

        // Check for "doesn't untap" effects
        let mut should_untap = true;

        // Check if this permanent is affected by a NoUntapEffect
        if let Some(no_untap) = no_untap_effect {
            // Check the condition that would prevent untapping
            if let Some(condition) = &no_untap.condition {
                // Evaluate the condition based on its type
                match condition {
                    NoUntapCondition::NextUntapStep => {
                        // Card doesn't untap for this turn only
                        should_untap = false;
                        info!("Permanent {:?} doesn't untap due to NextUntapStep", entity);
                    }
                    NoUntapCondition::WhilePermanentExists(e) => {
                        // Card doesn't untap while specific entity exists
                        // We would check if the entity still exists in battlefield
                        should_untap = false;
                        info!("Permanent {:?} doesn't untap while {:?} exists", entity, e);
                    }
                    NoUntapCondition::WhileControlling(e) => {
                        // Card doesn't untap if controller controls another specific entity
                        // We would check if controller has the other permanent
                        should_untap = false;
                        info!(
                            "Permanent {:?} doesn't untap while controlling {:?}",
                            entity, e
                        );
                    }
                    NoUntapCondition::WhileLifeLessThan(life_threshold) => {
                        // Card doesn't untap if controller's life is less than threshold
                        should_untap = false;
                        info!(
                            "Permanent {:?} doesn't untap while life below {}",
                            entity, life_threshold
                        );
                    }
                    NoUntapCondition::Custom(reason) => {
                        // Card doesn't untap for a custom reason
                        should_untap = false;
                        info!("Permanent {:?} doesn't untap due to: {}", entity, reason);
                    }
                }
            } else {
                // No specific condition means it never untaps during untap step
                should_untap = false;
                info!(
                    "Permanent {:?} doesn't untap due to unconditional effect",
                    entity
                );
            }
        }

        // Untap the permanent if no restrictions prevented it
        if should_untap {
            permanent_state.is_tapped = false;
            info!("Untapped permanent: {:?}", entity);
        }
    }

    // Mark the untap step as processed for this turn
    event_tracker.untap_step_processed = true;
    event_tracker.last_processed_turn = turn_manager.turn_number;
}
