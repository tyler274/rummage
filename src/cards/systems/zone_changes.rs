use bevy::prelude::*;

use crate::cards::CardZone;
use crate::game_engine::zones::{events::ZoneChangeEvent, types::Zone};
use crate::player::playmat::battlefield::BattlefieldZone;
use crate::player::playmat::hand::HandZone;

/// System that processes zone change events and updates card entities,
/// including parenting them to the correct zone entity.
#[allow(unused_variables)]
pub fn process_zone_changes(
    mut commands: Commands,
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    mut card_query: Query<&mut CardZone>,
    hand_zone_query: Query<(Entity, &HandZone)>,
    battlefield_zone_query: Query<(Entity, &BattlefieldZone)>,
) {
    for event in zone_change_events.read() {
        if let Ok(mut card_zone) = card_query.get_mut(event.card) {
            // 1. Update the card's zone component
            card_zone.set_zone(event.destination, Some(event.owner));
            info!(
                "Card {:?} moved to {:?} for player {:?}",
                event.card, event.destination, event.owner
            );

            // 2. Update the card's parent based on the destination zone
            match event.destination {
                Zone::Hand => {
                    if let Some((hand_zone_entity, _)) = hand_zone_query
                        .iter()
                        .find(|(_, hz)| hz.player_id == event.owner)
                    {
                        info!(
                            "Parenting card {:?} to HandZone {:?}",
                            event.card, hand_zone_entity
                        );
                        commands
                            .entity(event.card)
                            .insert(ChildOf(hand_zone_entity));
                    } else {
                        warn!("Could not find HandZone for player {:?}", event.owner);
                    }
                }
                Zone::Battlefield => {
                    if let Some((bf_zone_entity, _)) = battlefield_zone_query
                        .iter()
                        .find(|(_, bf)| bf.player_id == event.owner)
                    {
                        info!(
                            "Parenting card {:?} to BattlefieldZone {:?}",
                            event.card, bf_zone_entity
                        );
                        commands.entity(event.card).insert(ChildOf(bf_zone_entity));
                    } else {
                        warn!(
                            "Could not find BattlefieldZone for player {:?}",
                            event.owner
                        );
                    }
                }
                _ => {
                    info!(
                        "Removing parent for card {:?} entering zone {:?}",
                        event.card, event.destination
                    );
                    commands.entity(event.card).remove::<ChildOf>();
                }
            }

            // TODO: Add/remove other components based on the new zone
            // (e.g., PermanentState for Battlefield)
        }
    }
}
