use super::components::EliminationReason;
use crate::game_engine::zones::Zone;
use bevy::prelude::*;

/// Event that represents combat damage being dealt
#[derive(Event, Clone)]
pub struct CombatDamageEvent {
    /// The source of the damage (usually a creature)
    pub source: Entity,
    /// The target of the damage (player or creature)
    pub target: Entity,
    /// The amount of damage dealt
    pub damage: u32,
    /// Whether this is combat damage (vs. direct damage from spells/abilities)
    pub is_combat_damage: bool,
    /// Whether the source is a commander (for commander damage tracking)
    pub source_is_commander: bool,
}

/// Event that triggers when a player needs to decide if their commander
/// should go to the command zone instead of another zone
#[derive(Event)]
pub struct CommanderZoneChoiceEvent {
    /// The commander card entity
    pub commander: Entity,
    /// The owner of the commander
    pub owner: Entity,
    /// The zone the commander is currently in
    pub current_zone: Zone,
    /// Whether the commander can go to the command zone
    pub can_go_to_command_zone: bool,
}

/// Event that triggers when a player is eliminated from the game
#[derive(Event)]
pub struct PlayerEliminatedEvent {
    /// The player that was eliminated
    #[allow(dead_code)]
    pub player: Entity,
    /// The reason the player was eliminated
    #[allow(dead_code)]
    pub reason: EliminationReason,
}
