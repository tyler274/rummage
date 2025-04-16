use bevy::prelude::*;

use crate::cards::Card;
use crate::game_engine::zones::{Zone, ZoneManager};
use crate::menu::GameMenuState;
use crate::player::components::Player;

pub(super) fn check_card_status(
    cards: Query<(Entity, &Transform, &Visibility), With<Card>>,
    player_query: Query<(Entity, &Player)>,
    game_camera_query: Query<Entity, With<crate::camera::components::GameCamera>>,
    zone_manager: Res<ZoneManager>,
    mut has_run: Local<bool>,
    game_state: Res<State<GameMenuState>>,
) {
    // Only run when the game state is InGame
    if *game_state.get() != GameMenuState::InGame {
        return;
    }

    // Only run once
    if *has_run {
        return;
    }

    // Wait a few frames before checking
    static mut FRAME_COUNT: u32 = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT < 30 {
            return;
        }
    }

    *has_run = true;

    // Count cards in all zones
    let mut zone_manager_card_count = 0;

    // Count cards in hands
    for player_hand in zone_manager.hands.values() {
        zone_manager_card_count += player_hand.len();
    }

    // Count cards in libraries
    for player_library in zone_manager.libraries.values() {
        zone_manager_card_count += player_library.len();
    }

    // Count cards in battlefield
    zone_manager_card_count += zone_manager.battlefield.len();

    // Count cards in graveyards
    for player_graveyard in zone_manager.graveyards.values() {
        zone_manager_card_count += player_graveyard.len();
    }

    info!("Zone Manager contains {} cards", zone_manager_card_count);

    if cards.is_empty() {
        error!(
            "No cards found! Cards are not being spawned properly. this error is only important when we are loading a game or in a game"
        );

        // Don't return early, continue with diagnostics
    } else {
        info!("Found {} cards in the world", cards.iter().count());
    }

    // Check if cards are registered in the zone manager
    let mut cards_with_zones = 0;
    let mut cards_without_zones = 0;

    for (entity, transform, visibility) in cards.iter() {
        let is_visible = match visibility {
            Visibility::Visible => "visible",
            Visibility::Hidden => "hidden",
            Visibility::Inherited => "inherited",
        };

        // Check if the card is registered in any zone
        let zone = zone_manager.get_card_zone(entity);

        if let Some(zone) = zone {
            cards_with_zones += 1;
            info!(
                "Card {:?} at position {:?} is {} and in zone {:?}",
                entity, transform.translation, is_visible, zone
            );
        } else {
            cards_without_zones += 1;
            warn!(
                "Card {:?} at position {:?} is {} but not registered in any zone!",
                entity, transform.translation, is_visible
            );
        }
    }

    info!(
        "Zone registration status: {} cards in zones, {} cards without zones",
        cards_with_zones, cards_without_zones
    );

    // Check player entities
    if player_query.is_empty() {
        error!("No player entities found!");
    } else {
        info!("Found {} player entities", player_query.iter().count());
        for (entity, player) in player_query.iter() {
            info!(
                "Player {:?} with name '{}' at index {}",
                entity, player.name, player.player_index
            );

            // Check cards in player's hand
            if let Some(hand) = zone_manager.get_player_zone(entity, Zone::Hand) {
                info!("Player {} has {} cards in hand", player.name, hand.len());
            } else {
                warn!("Player {} has no hand zone registered!", player.name);
            }
        }
    }

    // Check game camera
    if game_camera_query.is_empty() {
        error!("No game camera found!");
    } else {
        // Changed from single() to safely handle multiple cameras
        let camera_count = game_camera_query.iter().count();
        if camera_count > 1 {
            warn!(
                "Multiple game cameras found ({}), this may cause rendering issues",
                camera_count
            );
            info!(
                "Game camera entities: {:?}",
                game_camera_query.iter().collect::<Vec<_>>()
            );
        } else {
            info!(
                "Game camera entity: {:?}",
                game_camera_query.iter().next().unwrap()
            );
        }
    }

    // Print some cards for debugging
    for (i, (entity, transform, visibility)) in cards.iter().enumerate().take(3) {
        let is_visible = match visibility {
            Visibility::Visible => "visible",
            Visibility::Hidden => "hidden",
            Visibility::Inherited => "inherited",
        };

        info!(
            "Card {}: Entity {:?} at position {:?} is {}",
            i, entity, transform.translation, is_visible
        );
    }
}
