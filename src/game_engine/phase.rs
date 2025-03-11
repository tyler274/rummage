use crate::game_engine::state::GameState;
use crate::game_engine::turns::TurnManager;
use crate::menu::state::GameMenuState;
use crate::player::Player;
use bevy::prelude::*;

/// The main phases of a Magic: The Gathering turn
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Represents any step in any phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Beginning(BeginningStep),
    Precombat(PrecombatStep),
    Combat(CombatStep),
    Postcombat(PostcombatStep),
    Ending(EndingStep),
}

/// Resource to track the current phase and step
#[derive(Resource, Debug)]
pub struct PhaseState {
    pub current_phase: Phase,
    pub current_step: Option<Step>,
}

impl Default for PhaseState {
    fn default() -> Self {
        let initial_phase = Phase::Beginning(BeginningStep::Untap);
        Self {
            current_phase: initial_phase,
            current_step: Some(Step::Beginning(BeginningStep::Untap)),
        }
    }
}

/// Event fired when a phase transition occurs
#[derive(Event)]
pub struct PhaseTransitionEvent {
    pub new_phase: Phase,
    pub new_step: Option<Step>,
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

/// System to handle phase transitions
pub fn phase_transition_system(
    _commands: Commands,
    mut phase_state: ResMut<PhaseState>,
    mut events: EventReader<PhaseTransitionEvent>,
    _player_query: Query<Entity, With<Player>>,
) {
    for event in events.read() {
        phase_state.current_phase = event.new_phase;
        phase_state.current_step = event.new_step;
        info!(
            "Phase changed to {:?}, step {:?}",
            phase_state.current_phase, phase_state.current_step
        );
    }
}

/// System to handle the beginning of a turn
pub fn turn_begin_system(_phase_state: ResMut<PhaseState>, _turn_manager: ResMut<TurnManager>) {
    // Implementation will be added later
}
