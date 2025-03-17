use crate::mana::Color;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Component that marks a card as a Commander
#[derive(Component, Debug, Clone)]
pub struct Commander {
    /// The original owner of this commander
    pub owner: Entity,

    /// How many times this commander has been cast from the command zone
    #[allow(dead_code)]
    pub cast_count: u32,

    /// Tracks commander damage dealt to each player
    pub damage_dealt: Vec<(Entity, u32)>,

    /// Commander's color identity (for deck validation)
    #[allow(dead_code)]
    pub color_identity: HashSet<Color>,

    /// Commander-specific flags
    #[allow(dead_code)]
    pub is_partner: bool,
    #[allow(dead_code)]
    pub is_background: bool,

    /// Track if commander has dealt combat damage this turn
    pub dealt_combat_damage_this_turn: HashSet<Entity>,
}

impl Default for Commander {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            cast_count: 0,
            damage_dealt: Vec::new(),
            color_identity: HashSet::new(),
            is_partner: false,
            is_background: false,
            dealt_combat_damage_this_turn: HashSet::new(),
        }
    }
}

/// Enum indicating where a commander is currently located
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommanderZoneLocation {
    CommandZone,
    Battlefield,
    Graveyard,
    Exile,
    Hand,
    Library,
    Stack,
}

/// Reason why a player was eliminated from the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EliminationReason {
    /// Player lost due to having 0 or less life
    LifeLoss,
    /// Player lost due to trying to draw from an empty library
    #[allow(dead_code)]
    EmptyLibrary,
    /// Player lost due to receiving 21+ commander damage from a single commander
    CommanderDamage(Entity), // The commander that dealt the lethal damage
    /// Player conceded
    #[allow(dead_code)]
    Concede,
    /// Player lost due to a specific card effect
    #[allow(dead_code)]
    CardEffect(Entity), // The card that caused the elimination
}
