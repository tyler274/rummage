use bevy::prelude::*;

use super::components::{
    MainMenuBackground, MainMenuButton, MainMenuContainer, MainMenuItem, MainMenuMusic,
};

/// System to clean up main menu entities when transitioning from main menu
pub fn cleanup_main_menu(
    mut commands: Commands,
    menu_items: Query<Entity, With<MainMenuItem>>,
    backgrounds: Query<Entity, With<MainMenuBackground>>,
    containers: Query<Entity, With<MainMenuContainer>>,
    buttons: Query<Entity, With<MainMenuButton>>,
    music: Query<Entity, With<MainMenuMusic>>,
) {
    // Clean up all menu items
    for entity in menu_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Clean up all menu backgrounds
    for entity in backgrounds.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Clean up all menu containers
    for entity in containers.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Clean up all menu buttons
    for entity in buttons.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Clean up menu music
    for entity in music.iter() {
        commands.entity(entity).despawn_recursive();
    }

    info!("Main menu cleanup completed");
}

/// System to specifically despawn main menu music when entering settings
pub fn cleanup_main_menu_music_on_settings_enter(
    mut commands: Commands,
    music_query: Query<Entity, With<MainMenuMusic>>,
) {
    if let Ok(entity) = music_query.get_single() {
        info!("Despawning main menu music upon entering settings menu.");
        commands.entity(entity).despawn_recursive();
    } else if music_query.iter().count() > 1 {
        warn!("Multiple MainMenuMusic entities found when entering settings! Despawning all.");
        for entity in music_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
