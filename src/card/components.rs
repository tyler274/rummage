use super::counters::PermanentCounters;
use bevy::prelude::*;

/// Component for tracking the state of permanents on the battlefield
#[derive(Component, Debug, Clone, Default)]
pub struct PermanentState {
    /// Whether the permanent is tapped (turned sideways)
    pub is_tapped: bool,
    /// Whether the permanent has summoning sickness (can't attack/use tap abilities)
    pub has_summoning_sickness: bool,
    /// The turn this permanent entered the battlefield
    pub turn_entered_battlefield: u32,
    /// Counters on the permanent
    #[allow(dead_code)]
    pub counters: PermanentCounters,
}

/// Component for permanents that have a "doesn't untap during untap step" effect
#[derive(Component, Debug, Clone)]
pub struct NoUntapEffect {
    /// The source of the effect (e.g., the card that applied this effect)
    #[allow(dead_code)]
    pub source: Option<Entity>,
    /// The condition that must be met for the permanent to not untap
    /// If None, the permanent never untaps during untap step
    pub condition: Option<NoUntapCondition>,
}

/// Conditions under which a permanent doesn't untap
#[derive(Debug, Clone)]
pub enum NoUntapCondition {
    /// The permanent doesn't untap during its controller's next untap step only
    #[allow(dead_code)]
    NextUntapStep,
    /// The permanent doesn't untap as long as a specified permanent exists
    #[allow(dead_code)]
    WhilePermanentExists(Entity),
    /// The permanent doesn't untap as long as the controller controls another specific permanent
    #[allow(dead_code)]
    WhileControlling(Entity),
    /// The permanent doesn't untap as long as the controller has less than X life
    #[allow(dead_code)]
    WhileLifeLessThan(i32),
    /// Custom textual description of the condition (for display purposes)
    #[allow(dead_code)]
    Custom(String),
}

#[derive(Component)]
pub struct Draggable {
    pub dragging: bool,
    pub drag_offset: Vec2,
    pub z_index: f32,
}

impl PermanentState {
    #[allow(dead_code)]
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
