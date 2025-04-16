use bevy::prelude::*;

use crate::cards::{Card, CardZone};
use crate::game_engine::zones::{Zone, ZoneManager, ZoneMarker};
use crate::menu::GameMenuState;
use crate::player::components::Player;

// Type alias for the query in register_unzoned_cards
type UnzonedCardQuery<'w, 's> =
    Query<'w, 's, (Entity, &'static CardZone), (With<Card>, Without<ZoneMarker>)>;

// One-time event to connect cards to zones after they're spawned
#[derive(Component)]
pub(super) struct InitializeCardsEvent;

pub(super) fn connect_cards_to_zones(
    mut commands: Commands,
    query: Query<(Entity, &InitializeCardsEvent)>,
    card_query: Query<(Entity, &CardZone)>,
    mut zone_manager: ResMut<ZoneManager>,
    game_state: Res<State<GameMenuState>>,
) {
    // Only run when the game state is InGame
    if *game_state.get() != GameMenuState::InGame {
        return;
    }

    for (entity, _) in query.iter() {
        info!("Connecting cards to zones...");

        let card_count = card_query.iter().count();
        info!("Found {} cards to connect to zones", card_count);

        if card_count == 0 {
            error!(
                "No cards found to connect to zones - cards are not being spawned properly. this error is only important when we are loading a game or in a game"
            );

            // Let's check if the zone manager already has any cards registered
            let mut total_cards = 0;

            // Count cards in hands
            for player_hand in zone_manager.hands.values() {
                total_cards += player_hand.len();
            }

            // Count cards in libraries
            for player_library in zone_manager.libraries.values() {
                total_cards += player_library.len();
            }

            // Count cards in battlefield
            total_cards += zone_manager.battlefield.len();

            // Count cards in graveyards
            for player_graveyard in zone_manager.graveyards.values() {
                total_cards += player_graveyard.len();
            }

            if total_cards > 0 {
                info!(
                    "Found {} cards already registered in the zone manager",
                    total_cards
                );
            } else {
                error!("Zone manager has no cards registered!");
            }
        }

        // Process each card and add it to the appropriate zone in ZoneManager
        for (card_entity, card_zone) in card_query.iter() {
            // First check if this card is already registered to avoid duplicates
            let already_registered = zone_manager.get_card_zone(card_entity).is_some();

            if already_registered {
                info!(
                    "Card {:?} is already registered in the zone manager, skipping",
                    card_entity
                );
                continue;
            }

            match card_zone.zone {
                Zone::Hand => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_hand(owner, card_entity);
                        info!("Added card {:?} to player {:?}'s hand", card_entity, owner);
                    } else {
                        warn!("Card {:?} has no owner, cannot add to hand", card_entity);
                    }
                }
                Zone::Library => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_library(owner, card_entity);
                        info!(
                            "Added card {:?} to player {:?}'s library",
                            card_entity, owner
                        );
                    } else {
                        warn!("Card {:?} has no owner, cannot add to library", card_entity);
                    }
                }
                Zone::Battlefield => {
                    let owner = card_zone.zone_owner.unwrap_or(Entity::PLACEHOLDER);
                    zone_manager.add_to_battlefield(owner, card_entity);
                    info!(
                        "Added card {:?} to battlefield with owner {:?}",
                        card_entity, owner
                    );
                }
                Zone::Graveyard => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_graveyard(owner, card_entity);
                        info!(
                            "Added card {:?} to player {:?}'s graveyard",
                            card_entity, owner
                        );
                    } else {
                        warn!(
                            "Card {:?} has no owner, cannot add to graveyard",
                            card_entity
                        );
                    }
                }
                Zone::Exile | Zone::Stack | Zone::Command => {
                    // These zones are global or handled elsewhere, no owner needed
                }
            }
        }

        // Remove the one-time event
        commands.entity(entity).despawn();
        info!("Card connection complete");
    }
}

pub(super) fn register_unzoned_cards(
    cards: UnzonedCardQuery,
    player_query: Query<(Entity, &Player)>,
    mut zone_manager: ResMut<ZoneManager>,
) {
    let card_count = cards.iter().count();
    if card_count == 0 {
        return;
    }

    info!(
        "Found {} cards not registered in any zone, attempting to register them",
        card_count
    );

    // Create a map of player indices to player entities
    let mut player_map = std::collections::HashMap::new();
    for (entity, player) in player_query.iter() {
        player_map.insert(player.player_index, entity);
    }

    // Register each card to the appropriate player's hand based on position
    for (card_entity, card_zone) in cards.iter() {
        // First check if this card is already registered to avoid duplicates
        let already_registered = zone_manager.get_card_zone(card_entity).is_some();
        if already_registered {
            continue;
        }

        let owner = if let Some(owner) = card_zone.zone_owner {
            owner
        } else if !player_map.is_empty() {
            // Default to first player if no owner is specified
            player_map.get(&0).copied().unwrap_or(Entity::PLACEHOLDER)
        } else {
            warn!(
                "No players found to assign card ownership for card {:?}",
                card_entity
            );
            Entity::PLACEHOLDER
        };

        // Initialize player zones if they don't exist yet
        zone_manager.init_player_zones(owner);

        // Add the card to the player's hand by default
        zone_manager.add_to_hand(owner, card_entity);
        info!(
            "Registered card {:?} to player {:?}'s hand",
            card_entity, owner
        );
    }
}
