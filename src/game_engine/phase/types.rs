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

/// Steps within the Precombat Main phase
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

/// Steps within the Postcombat Main phase
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
    /// Determine if the phase or step should auto-pass priority if the stack is empty
    #[allow(dead_code)]
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

    /// Determine if the phase allows player actions
    pub fn allows_actions(&self) -> bool {
        match self {
            Phase::Beginning(BeginningStep::Untap) => false,
            Phase::Beginning(BeginningStep::Draw) => false,
            Phase::Ending(EndingStep::Cleanup) => false,
            _ => true,
        }
    }

    /// Determine if the phase allows sorcery-speed actions
    pub fn allows_sorcery_speed(&self) -> bool {
        match self {
            Phase::Precombat(PrecombatStep::Main) => true,
            Phase::Postcombat(PostcombatStep::Main) => true,
            _ => false,
        }
    }

    /// Get the next phase or step in the sequence
    pub fn next(&self) -> Self {
        match self {
            Phase::Beginning(BeginningStep::Untap) => Phase::Beginning(BeginningStep::Upkeep),
            Phase::Beginning(BeginningStep::Upkeep) => Phase::Beginning(BeginningStep::Draw),
            Phase::Beginning(BeginningStep::Draw) => Phase::Precombat(PrecombatStep::Main),
            Phase::Precombat(PrecombatStep::Main) => Phase::Combat(CombatStep::Beginning),
            Phase::Combat(CombatStep::Beginning) => Phase::Combat(CombatStep::DeclareAttackers),
            Phase::Combat(CombatStep::DeclareAttackers) => {
                Phase::Combat(CombatStep::DeclareBlockers)
            }
            Phase::Combat(CombatStep::DeclareBlockers) => Phase::Combat(CombatStep::CombatDamage),
            Phase::Combat(CombatStep::CombatDamage) => Phase::Combat(CombatStep::End),
            Phase::Combat(CombatStep::End) => Phase::Postcombat(PostcombatStep::Main),
            Phase::Postcombat(PostcombatStep::Main) => Phase::Ending(EndingStep::End),
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
