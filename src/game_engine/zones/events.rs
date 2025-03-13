use super::types::Zone;
use bevy::prelude::*;

/// Event fired when a card changes zones
#[derive(Event, Debug)]
pub struct ZoneChangeEvent {
    /// The card that changed zones
    pub card: Entity,
    /// The player who owns the card
    pub owner: Entity,
    /// The source zone
    pub source: Zone,
    /// The destination zone
    pub destination: Zone,
    /// Whether the card was visible in the source zone
    /// TODO: Implement visibility tracking for zone changes
    #[allow(dead_code)]
    pub was_visible: bool,
    /// Whether the card is visible in the destination zone
    /// TODO: Implement visibility rules for different zones
    #[allow(dead_code)]
    pub is_visible: bool,
}

/// Event fired when a permanent enters the battlefield
#[derive(Event)]
pub struct EntersBattlefieldEvent {
    /// The permanent that entered the battlefield
    pub permanent: Entity,
    /// The owner of the permanent
    pub owner: Entity,
    /// Whether the permanent entered tapped
    pub enters_tapped: bool,
}
