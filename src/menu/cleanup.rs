use crate::{card::Card, menu::components::*};
use bevy::prelude::*;

/// Cleans up main menu entities
pub fn cleanup_main_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for entity in menu_items.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Cleans up pause menu entities
pub fn cleanup_pause_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for entity in menu_items.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Cleans up menu camera entities
pub fn cleanup_menu_camera(mut commands: Commands, menu_cameras: Query<Entity, With<MenuCamera>>) {
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
    for entity in cards.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in game_cameras.iter() {
        commands.entity(entity).despawn();
    }
}
