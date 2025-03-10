use crate::{
    card::Card,
    menu::{components::*, logo::StarOfDavid},
};
use bevy::prelude::*;

/// Cleans up main menu entities
pub fn cleanup_main_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    let count = menu_items.iter().count();
    info!("Cleaning up {} main menu items", count);
    for entity in menu_items.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Cleans up pause menu entities
pub fn cleanup_pause_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    let count = menu_items.iter().count();
    info!("Cleaning up {} pause menu items", count);
    for entity in menu_items.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Cleans up menu camera entities
pub fn cleanup_menu_camera(mut commands: Commands, menu_cameras: Query<Entity, With<MenuCamera>>) {
    let count = menu_cameras.iter().count();
    info!("Cleaning up {} menu cameras", count);
    for entity in menu_cameras.iter() {
        info!("Despawning menu camera entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}

/// Cleans up Star of David entities
pub fn cleanup_star_of_david(mut commands: Commands, stars: Query<Entity, With<StarOfDavid>>) {
    let count = stars.iter().count();
    info!("Cleaning up {} Star of David entities", count);
    for entity in stars.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Cleans up Star of David entities more thoroughly
pub fn cleanup_star_of_david_thoroughly(
    mut commands: Commands,
    stars_query: Query<Entity, With<StarOfDavid>>,
    children_query: Query<&Children>,
    star_components: Query<(Entity, &StarOfDavid), Without<Children>>,
) {
    let count = stars_query.iter().count();
    info!("Cleaning up {} Star of David entities thoroughly", count);

    // First, remove all children entities recursively
    for entity in stars_query.iter() {
        if let Ok(children) = children_query.get(entity) {
            for &child in children.iter() {
                commands.entity(child).despawn_recursive();
            }
        }
        commands.entity(entity).despawn_recursive();
    }

    // Also clean up any orphaned StarOfDavid components that might not have children
    for (entity, _) in star_components.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Let commands execute immediately - no need for a dummy entity
}

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
