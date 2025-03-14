use crate::mana::Mana;
use bevy::prelude::*;

/// Component that represents an activated ability on a card
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ActivatedAbility {
    /// The cost to activate this ability (mana)
    pub mana_cost: Option<Mana>,
    /// The cost to activate this ability (tap)
    pub tap_cost: bool,
    /// Whether this ability requires the card to be sacrificed
    pub sacrifice_cost: bool,
    /// Description of the ability
    pub description: String,
    /// Whether this ability can be activated at instant speed
    pub instant_speed: bool,
    /// Additional costs (discard, exile, etc.)
    pub additional_costs: Vec<String>,
}

/// Component that represents a triggered ability on a card
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct TriggeredAbility {
    /// The trigger condition for this ability
    pub trigger: TriggerCondition,
    /// Description of the ability
    pub description: String,
    /// Whether this ability triggers only once per turn
    pub once_per_turn: bool,
    /// Whether this ability is optional ("may" effects)
    pub optional: bool,
}

/// Different types of trigger conditions for triggered abilities
#[derive(Debug, Clone, Reflect)]
pub enum TriggerCondition {
    /// Triggers when the card enters the battlefield
    EnterTheBattlefield,
    /// Triggers when the card leaves the battlefield
    LeaveTheBattlefield,
    /// Triggers at the beginning of a specific phase
    BeginningOfPhase(Phase),
    /// Triggers when a specific action happens
    OnAction(String),
    /// Triggers when a specific card type enters the battlefield
    WhenTypeEnters(String),
    /// Triggers when a creature attacks
    WhenAttacks,
    /// Triggers when a creature blocks
    WhenBlocks,
    /// Triggers when a creature dies
    WhenCreatureDies,
    /// Triggers when a player casts a specific type of spell
    WhenPlayerCasts(String),
    /// Custom trigger condition (for complex abilities)
    Custom(String),
}

/// Game phases for phase-based triggers
#[derive(Debug, Clone, Reflect)]
pub enum Phase {
    Untap,
    Upkeep,
    Draw,
    MainOne,
    Combat,
    MainTwo,
    End,
    Cleanup,
}

impl ActivatedAbility {
    /// Creates a new activated ability with a mana cost
    #[allow(dead_code)]
    pub fn with_mana_cost(cost: Mana, description: &str) -> Self {
        Self {
            mana_cost: Some(cost),
            tap_cost: false,
            sacrifice_cost: false,
            description: description.to_string(),
            instant_speed: false,
            additional_costs: Vec::new(),
        }
    }

    /// Creates a new tap ability
    #[allow(dead_code)]
    pub fn tap(description: &str) -> Self {
        Self {
            mana_cost: None,
            tap_cost: true,
            sacrifice_cost: false,
            description: description.to_string(),
            instant_speed: false,
            additional_costs: Vec::new(),
        }
    }

    /// Makes this ability activatable at instant speed
    #[allow(dead_code)]
    pub fn at_instant_speed(mut self) -> Self {
        self.instant_speed = true;
        self
    }
}

impl TriggeredAbility {
    /// Creates a new "enters the battlefield" triggered ability
    #[allow(dead_code)]
    pub fn on_etb(description: &str) -> Self {
        Self {
            trigger: TriggerCondition::EnterTheBattlefield,
            description: description.to_string(),
            once_per_turn: false,
            optional: false,
        }
    }

    /// Makes this ability optional ("you may...")
    #[allow(dead_code)]
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Limits this ability to once per turn
    #[allow(dead_code)]
    pub fn once_per_turn(mut self) -> Self {
        self.once_per_turn = true;
        self
    }
}
