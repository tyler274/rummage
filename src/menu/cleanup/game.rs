use crate::{
    camera::components::GameCamera,
    cards::Card,
};
use bevy::prelude::*;

/// Cleans up game entities (cards and game camera)
pub fn cleanup_game(
    mut commands: Commands,
    cards: Query<Entity, With<Card>>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    let card_count = cards.iter().count();
    let camera_count = game_cameras.iter().count();
    info!(
        "Cleaning up {} cards and {} game cameras",
        card_count, camera_count
    );

    // First clean up all cards
    for entity in cards.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Then clean up all game cameras
    for entity in game_cameras.iter() {
        info!("Despawning game camera entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }

    // Verify cleanup
    let remaining_cameras = game_cameras.iter().count();
    if remaining_cameras > 0 {
        warn!(
            "{} game cameras still exist after cleanup!",
            remaining_cameras
        );
    }
}

/// System to clean up temporary elements when leaving the card selection screen
#[allow(dead_code)]
pub fn cleanup_card_selection(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // Implementation pending
} 