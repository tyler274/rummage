use crate::{camera::components::GameCamera, cards::Card};
use bevy::prelude::*;

/// Cleans up game entities (cards)
/// Note: Game camera is no longer despawned here; visibility is handled by another system.
pub fn cleanup_game(
    mut commands: Commands,
    cards: Query<Entity, With<Card>>,
    game_cameras: Query<Entity, With<GameCamera>>, // Keep query for logging count
) {
    let card_count = cards.iter().count();
    let camera_count = game_cameras.iter().count(); // Log count but don't despawn
    info!(
        "Cleaning up {} cards. Found {} game cameras (will not despawn).",
        card_count, camera_count
    );

    // First clean up all cards
    for entity in cards.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // // Then clean up all game cameras - REMOVED
    // for entity in game_cameras.iter() {
    //     info!("Despawning game camera entity: {:?}", entity);
    //     commands.entity(entity).despawn_recursive();
    // }
}

/// System to clean up temporary elements when leaving the card selection screen
#[allow(dead_code)]
pub fn cleanup_card_selection(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // Implementation pending
}
