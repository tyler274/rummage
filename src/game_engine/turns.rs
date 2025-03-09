use crate::game_engine::{BeginningStep, GameState, Phase};
use crate::player::Player;
use bevy::prelude::*;

/// Event triggered at the start of a turn
#[derive(Event)]
pub struct TurnStartEvent {
    pub player: Entity,
    pub turn_number: u32,
}

/// Event triggered at the end of a turn
#[derive(Event)]
pub struct TurnEndEvent {
    pub player: Entity,
    pub turn_number: u32,
}

/// System to handle the start of a new turn
pub fn turn_start_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    phase: Res<Phase>,
    player_query: Query<&Player>,
    mut turn_start_events: EventWriter<TurnStartEvent>,
) {
    // Only trigger at the beginning of the untap step
    if *phase == Phase::Beginning(BeginningStep::Untap) {
        // Emit a turn start event
        turn_start_events.send(TurnStartEvent {
            player: game_state.active_player,
            turn_number: game_state.turn_number,
        });

        // At the start of a turn, perform these actions:
        // - Untap all permanents controlled by the active player
        // - Reset "until end of turn" effects
        // - Reset damage on creatures

        info!(
            "Turn {} started for player {:?}",
            game_state.turn_number, game_state.active_player
        );

        // For Commander specifically:
        // - Check if any Commander tax needs to be applied
        // - Check for Commander damage thresholds (21 damage from same commander)
    }
}

/// System to handle the end of a turn
pub fn turn_end_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    phase: Res<Phase>,
    player_query: Query<&Player>,
    mut turn_end_events: EventWriter<TurnEndEvent>,
) {
    // Only trigger at the end of the cleanup step
    if let Phase::Ending(crate::game_engine::EndingStep::Cleanup) = *phase {
        // Emit a turn end event
        turn_end_events.send(TurnEndEvent {
            player: game_state.active_player,
            turn_number: game_state.turn_number,
        });

        // At the end of a turn, perform these actions:
        // - Remove "until end of turn" effects
        // - Discard down to maximum hand size (normally 7)

        info!(
            "Turn {} ended for player {:?}",
            game_state.turn_number, game_state.active_player
        );
    }
}

/// System to handle untapping permanents at the start of a turn
pub fn untap_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    phase: Res<Phase>,
    // We would need queries for permanents to untap them
) {
    // Only run during the untap step
    if *phase == Phase::Beginning(BeginningStep::Untap) {
        // Untap all permanents controlled by the active player
        // In a full implementation, this would query for all tapped permanents
        // belonging to the active player and untap them (with exceptions for
        // cards with "doesn't untap during untap step" effects)

        info!(
            "Untapping permanents for player {:?}",
            game_state.active_player
        );
    }
}

/// Register the turn-related systems and events
pub fn register_turn_systems(app: &mut App) {
    app.add_event::<TurnStartEvent>()
        .add_event::<TurnEndEvent>()
        .add_systems(Update, (turn_start_system, turn_end_system, untap_system));
}
