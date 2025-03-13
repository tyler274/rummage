use crate::mana::Mana;
use bevy::prelude::*;

/// Different types of game actions a player can take
#[derive(Debug, Clone, Event)]
pub enum GameAction {
    /// Play a land
    PlayLand { player: Entity, land_card: Entity },
    /// Cast a spell
    CastSpell {
        player: Entity,
        spell_card: Entity,
        targets: Vec<Entity>,
        mana_payment: Mana,
    },
    /// Activate an ability
    ActivateAbility {
        player: Entity,
        source: Entity,
        ability_index: usize,
        targets: Vec<Entity>,
        mana_payment: Mana,
    },
    /// Pass priority
    PassPriority { player: Entity },
}
