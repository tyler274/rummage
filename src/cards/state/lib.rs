use crate::cards::counters::PermanentCounters;
use bevy::prelude::*;

/// Component for tracking the state of permanents on the battlefield
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct CardState {
    /// Whether the permanent is tapped (turned sideways)
    pub is_tapped: bool,
    /// Whether the permanent has summoning sickness (can't attack/use tap abilities)
    pub has_summoning_sickness: bool,
    /// The turn this permanent entered the battlefield
    pub turn_entered_battlefield: u32,
    /// Whether this card is currently revealed to all players
    pub is_revealed: bool,
    /// Whether this card is currently face-down
    pub is_face_down: bool,
    /// Counters on the permanent
    pub counters: PermanentCounters,
}

impl CardState {
    /// Create a new card state on a specific turn
    #[allow(dead_code)]
    pub fn new(turn_number: u32) -> Self {
        Self {
            is_tapped: false,
            has_summoning_sickness: true,
            turn_entered_battlefield: turn_number,
            is_revealed: false,
            is_face_down: false,
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
    #[allow(dead_code)]
    pub fn update_summoning_sickness(&mut self, current_turn: u32) {
        if self.has_summoning_sickness && current_turn > self.turn_entered_battlefield {
            self.has_summoning_sickness = false;
        }
    }

    /// Reveal a card to all players
    #[allow(dead_code)]
    pub fn reveal(&mut self) {
        self.is_revealed = true;
    }

    /// Hide a previously revealed card
    #[allow(dead_code)]
    pub fn hide(&mut self) {
        self.is_revealed = false;
    }

    /// Turn a card face-down
    #[allow(dead_code)]
    pub fn turn_face_down(&mut self) {
        self.is_face_down = true;
    }

    /// Turn a card face-up
    #[allow(dead_code)]
    pub fn turn_face_up(&mut self) {
        self.is_face_down = false;
    }
}
