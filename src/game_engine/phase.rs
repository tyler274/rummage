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
    commands: Commands,
    mut game_state: ResMut<GameState>,
    mut phase: ResMut<Phase>,
    mut turn_manager: ResMut<TurnManager>,
    priority_system: Res<crate::game_engine::PrioritySystem>,
    stack: Res<crate::game_engine::GameStack>,
    player_query: Query<Entity, With<Player>>,
) {
    // If all players have passed priority and the stack is empty, move to the next phase
    if priority_system.all_players_passed && stack.is_empty() {
        let next_phase = phase.next();

        // Special case: when moving from Cleanup to a new turn's Untap
        if *phase == Phase::Ending(EndingStep::Cleanup)
            && next_phase == Phase::Beginning(BeginningStep::Untap)
        {
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

        // Reset priority for the new phase
        if next_phase.allows_actions() {
            // Give priority to the active player
            game_state.priority_holder = turn_manager.active_player;
        }
    }
}
