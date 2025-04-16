use bevy::prelude::*;

use crate::camera::components::GameCamera;
use crate::deck::PlayerDeck;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use crate::player::systems::spawn::cards;
use crate::player::systems::spawn::table::TableLayout;

/// Marker component to trigger visual hand spawning for a player
#[derive(Component)]
pub(super) struct SpawnVisualHand {
    pub(super) player_entity: Entity,
    pub(super) deck: PlayerDeck,
    pub(super) position: Vec3, // Store position needed for context
}

pub(super) fn spawn_player_visual_hands(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    player_query: Query<&Player>,
    player_config: Res<PlayerConfig>,
    marker_query: Query<(Entity, &SpawnVisualHand)>,
) {
    if marker_query.is_empty() {
        return; // No hands to spawn
    }

    let config = player_config.clone();
    let table = TableLayout::new(config.player_count, config.player_card_distance);

    // Check the query directly
    if game_cameras.is_empty() {
        error!("No game camera found, cannot spawn visual cards.");
        // Consider despawning markers anyway or retrying?
        for (marker_entity, _) in marker_query.iter() {
            commands.entity(marker_entity).despawn();
        }
        return;
    }

    for (marker_entity, marker) in marker_query.iter() {
        info!("Spawning visual hand for player {:?}", marker.player_entity);

        let mut deck_copy = marker.deck.deck.clone(); // Clone deck from marker
        let display_cards = deck_copy.draw_multiple(7); // Draw from the cloned deck

        if display_cards.is_empty() {
            warn!(
                "Deck for player {:?} was empty, cannot spawn hand.",
                marker.player_entity
            );
            commands.entity(marker_entity).despawn(); // Remove marker
            continue;
        }

        // Get Player component to access player_index
        if let Ok(player) = player_query.get(marker.player_entity) {
            let player_index = player.player_index;

            // Remove context creation, call spawn_visual_cards directly
            cards::spawn_visual_cards(
                &mut commands,
                &config.card_size,
                config.card_spacing_multiplier,
                marker.position, // Use stored position
                player_index,
                marker.player_entity,
                &table,
                Some(&asset_server).map(|v| &**v), // Convert Option<&Res<AssetServer>> to Option<&AssetServer>
                display_cards,
            );
        } else {
            warn!(
                "Could not find Player component for entity {:?}, skipping hand spawn.",
                marker.player_entity
            );
        }

        // Despawn the marker entity once processed
        commands.entity(marker_entity).despawn();
    }
} 