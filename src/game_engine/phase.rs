use crate::game_engine::PrioritySystem;
use crate::game_engine::priority::NextPhaseEvent;
use crate::game_engine::state::GameState;
use crate::game_engine::turns::TurnManager;
use crate::player::Player;
use bevy::prelude::*;

/// The main phases of a Magic: The Gathering turn
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum Phase {
    Beginning(BeginningStep),
    Precombat(PrecombatStep),
    Combat(CombatStep),
    Postcombat(PostcombatStep),
    Ending(EndingStep),
}

/// Steps within the Beginning phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeginningStep {
    Untap,
    Upkeep,
    Draw,
}

/// Steps within the Precombat phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecombatStep {
    Main,
}

/// Steps within the Combat phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatStep {
    Beginning,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    End,
}

/// Steps within the Postcombat phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostcombatStep {
    Main,
}

/// Steps within the Ending phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndingStep {
    End,
    Cleanup,
}

impl Phase {
    /// Returns whether this phase/step should automatically pass priority if nothing happens
    pub fn auto_pass_if_empty(&self) -> bool {
        match self {
            Phase::Beginning(BeginningStep::Untap) => true,
            Phase::Beginning(BeginningStep::Draw) => true,
            Phase::Combat(CombatStep::Beginning) => true,
            Phase::Combat(CombatStep::End) => true,
            Phase::Ending(EndingStep::Cleanup) => true,
            _ => false,
        }
    }

    /// Returns whether players can cast spells/activate abilities during this phase/step
    pub fn allows_actions(&self) -> bool {
        match self {
            Phase::Beginning(BeginningStep::Untap) => false,
            Phase::Ending(EndingStep::Cleanup) => false,
            _ => true,
        }
    }

    /// Returns whether sorcery-speed spells can be cast during this phase/step
    /// (assuming it's the active player's main phase and the stack is empty)
    pub fn allows_sorcery_speed(&self) -> bool {
        match self {
            Phase::Precombat(PrecombatStep::Main) => true,
            Phase::Postcombat(PostcombatStep::Main) => true,
            _ => false,
        }
    }

    /// Advances to the next phase/step
    pub fn next(&self) -> Self {
        match self {
            // Beginning phase progression
            Phase::Beginning(BeginningStep::Untap) => Phase::Beginning(BeginningStep::Upkeep),
            Phase::Beginning(BeginningStep::Upkeep) => Phase::Beginning(BeginningStep::Draw),
            Phase::Beginning(BeginningStep::Draw) => Phase::Precombat(PrecombatStep::Main),

            // Precombat phase progression
            Phase::Precombat(PrecombatStep::Main) => Phase::Combat(CombatStep::Beginning),

            // Combat phase progression
            Phase::Combat(CombatStep::Beginning) => Phase::Combat(CombatStep::DeclareAttackers),
            Phase::Combat(CombatStep::DeclareAttackers) => {
                Phase::Combat(CombatStep::DeclareBlockers)
            }
            Phase::Combat(CombatStep::DeclareBlockers) => Phase::Combat(CombatStep::CombatDamage),
            Phase::Combat(CombatStep::CombatDamage) => Phase::Combat(CombatStep::End),
            Phase::Combat(CombatStep::End) => Phase::Postcombat(PostcombatStep::Main),

            // Postcombat phase progression
            Phase::Postcombat(PostcombatStep::Main) => Phase::Ending(EndingStep::End),

            // Ending phase progression
            Phase::Ending(EndingStep::End) => Phase::Ending(EndingStep::Cleanup),
            Phase::Ending(EndingStep::Cleanup) => Phase::Beginning(BeginningStep::Untap),
        }
    }
}

impl Default for Phase {
    fn default() -> Self {
        Phase::Beginning(BeginningStep::Untap)
    }
}

/// System to handle phase transitions
pub fn phase_transition_system(
    mut commands: Commands,
    mut phase: ResMut<Phase>,
    mut turn_manager: ResMut<crate::game_engine::turns::TurnManager>,
    mut game_state: ResMut<crate::game_engine::state::GameState>,
    mut priority_system: ResMut<crate::game_engine::PrioritySystem>,
    mut next_phase_events: EventReader<NextPhaseEvent>,
    player_query: Query<Entity, With<Player>>,
) {
    // Process next phase events specifically triggered by priority system
    let has_next_phase_event = !next_phase_events.is_empty();
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

    // Handle automatic phase transitions and all-players-passed case
    if !has_next_phase_event && priority_system.all_players_passed {
        let current_phase = *phase;

        // Check if we need to advance to the next phase
        if priority_system.stack_is_empty
            && !priority_system.has_processed_phase(current_phase, turn_manager.turn_number)
        {
            advance_phase(
                &mut commands,
                &mut phase,
                &mut turn_manager,
                &mut game_state,
                &mut priority_system,
                &player_query,
            );

            // Mark this phase as processed
            priority_system.mark_phase_processed(current_phase, turn_manager.turn_number);
        }
    }
}

/// Advance to the next phase
fn advance_phase(
    _commands: &mut Commands,
    phase: &mut Phase,
    turn_manager: &mut TurnManager,
    game_state: &mut GameState,
    priority_system: &mut PrioritySystem,
    player_query: &Query<Entity, With<Player>>,
) {
    // Store current phase to detect if we've already processed this transition
    let current_phase = *phase;

    // If this phase has already been processed for the current turn, skip
    if priority_system.has_processed_phase(current_phase, turn_manager.turn_number)
        && priority_system.all_players_passed
    {
        // This specific phase combination has already been fully processed this turn
        return;
    }

    // If all players have passed priority and the stack is empty, move to the next phase
    if priority_system.all_players_passed && priority_system.stack_is_empty {
        let next_phase = phase.next();

        // If we're about to wrap around to a new turn
        let start_new_turn = current_phase == Phase::Ending(EndingStep::Cleanup)
            && next_phase == Phase::Beginning(BeginningStep::Untap);

        // Special case: when moving from Cleanup to a new turn's Untap
        if start_new_turn {
            // Advance to the next player in turn order
            turn_manager.advance_turn();

            // Also update the game state to keep them in sync
            game_state.turn_number = turn_manager.turn_number;
            game_state.active_player = turn_manager.active_player;

            info!(
                "Turn {} begins for player {:?}",
                turn_manager.turn_number, turn_manager.active_player
            );
        }

        // Update the phase
        *phase = next_phase;
        // Keep the TurnManager's phase in sync
        turn_manager.current_phase = *phase;

        info!("Phase changed to {:?}", *phase);

        // Perform phase-specific actions
        match *phase {
            Phase::Beginning(BeginningStep::Untap) => {
                // Untap permanents controlled by active player
                // This will be implemented in a separate untap system
            }
            Phase::Beginning(BeginningStep::Draw) => {
                // Active player draws a card (except on the first turn of the game)
                if turn_manager.turn_number > 1 {
                    // Card draw will be implemented in a separate system
                }
            }
            _ => {}
        }

        // Reset the priority system to prevent immediate phase transitions
        let players: Vec<Entity> = player_query.iter().collect();
        let active_player = turn_manager.active_player;

        // Reset priority system for the new phase
        priority_system.initialize(&players, active_player);
        priority_system.all_players_passed = false;

        // If we're in a phase that doesn't allow actions, immediately mark all players as passed
        if !next_phase.allows_actions() {
            priority_system.reset_passing_status();
            for _ in 0..priority_system.player_order.len() {
                priority_system.pass_priority();
            }
        }

        // Update priority holder in game state
        game_state.priority_holder = priority_system.priority_player;

        // Mark this as a different phase so we know it's a new state
        if start_new_turn {
            // For a new turn, we haven't processed any phases yet
            priority_system.last_processed_phase = None;
            priority_system.last_processed_turn = turn_manager.turn_number;
        }
    }
}
