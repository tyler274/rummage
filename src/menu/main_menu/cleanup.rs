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
