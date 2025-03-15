use bevy::prelude::*;

use crate::cards::CardZone;
use crate::game_engine::zones::events::ZoneChangeEvent;

/// System that processes zone change events and updates card entities
#[allow(unused_variables)]
pub fn process_zone_changes(
    commands: Commands,
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    mut card_zones: Query<&mut CardZone>,
) {
    for event in zone_change_events.read() {
        if let Ok(mut card_zone) = card_zones.get_mut(event.card) {
            // Update the card's zone component
            card_zone.set_zone(event.destination, Some(event.owner));

            // Here you would add/remove components based on the new zone
            // For example, adding PermanentState when a card enters the battlefield
        }
    }
}
