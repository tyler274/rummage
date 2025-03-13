use bevy::prelude::*;

/// Event for passing priority
#[derive(Event)]
pub struct PassPriorityEvent {
    /// The player passing priority
    pub player: Entity,
}

/// Event for resolving stack items
#[derive(Event)]
pub struct ResolveStackItemEvent {
    /// The stack item to resolve
    #[allow(dead_code)]
    pub item: Entity,
}

/// Event for phase transitions
#[derive(Event)]
pub struct NextPhaseEvent;

/// Reasons a spell or ability can be countered
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CounterReason {
    /// Explicitly countered by a spell or ability
    #[allow(dead_code)]
    CounterSpell,
    /// Countered due to invalid targets on resolution
    InvalidTargets,
    /// Countered due to rules (e.g., illegal targets)
    #[allow(dead_code)]
    RulesBased,
}

/// Event when a spell or ability is countered
#[derive(Event)]
pub struct EffectCounteredEvent {
    /// The item that was countered
    #[allow(dead_code)]
    pub item: Entity,
    /// The reason it was countered
    #[allow(dead_code)]
    pub reason: CounterReason,
}
