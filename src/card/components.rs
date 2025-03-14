use super::counters::PermanentCounters;
use super::details::CardDetails;
use super::keywords::KeywordAbilities;
use super::types::CardTypes;
use crate::mana::Mana;
use bevy::prelude::*;

/// Component for storing a card's name
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CardName {
    /// The name of the card
    pub name: String,
}

impl CardName {
    /// Create a new CardName from a string
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Get the card name as a string reference
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Check if the card name contains the given substring
    pub fn contains(&self, text: &str) -> bool {
        self.name.contains(text)
    }
}

/// Component for storing a card's mana cost
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CardCost {
    /// The mana cost of the card
    pub cost: Mana,
}

/// Component for storing a card's type information
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct CardTypeInfo {
    /// The types of the card (Creature, Instant, etc.)
    pub types: CardTypes,
}

/// Component for storing a card's rules text
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CardRulesText {
    /// The rules text of the card
    pub rules_text: String,
}

/// Component for storing a card's keyword abilities
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CardKeywords {
    /// The keyword abilities the card has
    pub keywords: KeywordAbilities,
}

/// Component for storing a card's details (creature stats, land types, etc.)
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CardDetailsComponent {
    /// The details of the card
    pub details: CardDetails,
}

/// Component for tracking the state of permanents on the battlefield
/// @deprecated Use CardState from the state module instead
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
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

/// Component for permanents that have a "doesn't untap during untap step" effect
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct NoUntapEffect {
    /// The source of the effect (e.g., the card that applied this effect)
    pub source: Option<Entity>,
    /// The condition that must be met for the permanent to not untap
    /// If None, the permanent never untaps during untap step
    pub condition: Option<NoUntapCondition>,
}

/// Conditions under which a permanent doesn't untap
#[derive(Debug, Clone, Reflect)]
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

/// Component for any entity that can be dragged by the player
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Draggable {
    pub dragging: bool,
    pub drag_offset: Vec2,
    pub z_index: f32,
}

impl Default for Draggable {
    fn default() -> Self {
        Self {
            dragging: false,
            drag_offset: Vec2::ZERO,
            z_index: 0.0,
        }
    }
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
