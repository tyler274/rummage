use crate::cards::counters::PermanentCounters;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for entities that are permanents on the battlefield
#[derive(Component, Debug, Clone, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Permanent;

/// Component for tracking the state of permanents on the battlefield
#[derive(Component, Debug, Clone, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PermanentState {
    /// Whether the permanent is tapped (turned sideways)
    pub is_tapped: bool,
    /// Whether the permanent has summoning sickness (can't attack/use tap abilities)
    pub has_summoning_sickness: bool,
    /// The turn this permanent entered the battlefield
    pub turn_entered_battlefield: u32,
    /// Counters on the permanent
    pub counters: PermanentCounters,
}

impl PermanentState {
    /// Create a new permanent state on a specific turn
    pub fn new(turn_number: u32) -> Self {
        Self {
            is_tapped: false,
            has_summoning_sickness: true,
            turn_entered_battlefield: turn_number,
            counters: PermanentCounters::default(),
        }
    }

    /// Tap a permanent. Returns false if already tapped.
    #[allow(dead_code)]
    pub fn tap(&mut self) -> bool {
        if self.is_tapped {
            return false;
        }
        self.is_tapped = true;
        true
    }

    /// Untap a permanent. Returns false if already untapped.
    #[allow(dead_code)]
    pub fn untap(&mut self) -> bool {
        if !self.is_tapped {
            return false;
        }
        self.is_tapped = false;
        true
    }

    /// Check if the permanent can be tapped (not already tapped and no summoning sickness for creatures)
    #[allow(dead_code)]
    pub fn can_tap(&self, is_creature: bool) -> bool {
        !self.is_tapped && (!is_creature || !self.has_summoning_sickness)
    }

    /// Update summoning sickness at the beginning of its controller's turn
    pub fn update_summoning_sickness(&mut self, current_turn: u32) {
        if self.has_summoning_sickness && current_turn > self.turn_entered_battlefield {
            self.has_summoning_sickness = false;
        }
    }
}

/// Component for permanents that have a "doesn't untap during untap step" effect
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct NoUntapEffect {
    /// The source of the effect (e.g., the card that applied this effect)
    pub source: Option<Entity>,
    /// The condition that must be met for the permanent to not untap
    /// If None, the permanent never untaps during untap step
    pub condition: Option<NoUntapCondition>,
}

/// Conditions under which a permanent doesn't untap
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum NoUntapCondition {
    /// The permanent doesn't untap during its controller's next untap step only
    NextUntapStep,
    /// The permanent doesn't untap as long as a specified permanent exists
    WhilePermanentExists(Entity),
    /// The permanent doesn't untap as long as the controller controls another specific permanent
    WhileControlling(Entity),
    /// The permanent doesn't untap as long as the controller has less than X life
    WhileLifeLessThan(i32),
    /// Custom textual description of the condition (for display purposes)
    Custom(String),
}
