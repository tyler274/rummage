use bevy::prelude::*;

use crate::cards::{Card, CardEntity, CardName, CardOwner, CardZone};
use crate::game_engine::zones::Zone;

/// Bundle of components needed for a card entity
#[allow(dead_code)]
pub struct CardEntityBundle {
    /// The card data
    pub card: Card,
    /// Marker component
    pub card_entity: CardEntity,
    /// The card's current zone
    pub card_zone: CardZone,
    /// The card's owner
    pub card_owner: CardOwner,
    /// Name component for easy querying
    pub name: CardName,
}

/// Spawn a new card entity in a specified zone
#[allow(dead_code)]
pub fn spawn_card(
    commands: &mut Commands,
    card: Card,
    owner: Entity,
    zone: Zone,
    zone_owner: Option<Entity>,
) -> Entity {
    // Extract name for convenience
    let name = card.name.clone();

    commands
        .spawn((
            card,
            CardEntity,
            CardZone::new(zone, zone_owner),
            CardOwner::new(owner),
            name,
        ))
        .id()
}
