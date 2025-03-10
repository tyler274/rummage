use crate::{card::Card, menu::components::*};
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
        commands.entity(entity).despawn();
    }
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
    for entity in cards.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in game_cameras.iter() {
        commands.entity(entity).despawn();
    }
}
