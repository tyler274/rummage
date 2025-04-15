use bevy::audio::AudioSink;
use bevy::prelude::*;

use super::components::{MainMenuBackground, MainMenuButton, MainMenuContainer, MainMenuItem};
use crate::menu::main_menu::components::MainMenuMusic;

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

/// System to specifically pause main menu music when entering settings
pub fn pause_main_menu_music_on_settings_enter(
    music_query: Query<&AudioSink, With<MainMenuMusic>>,
) {
    if let Ok(sink) = music_query.get_single() {
        info!("Pausing main menu music upon entering settings menu.");
        sink.pause();
    } else if music_query.iter().count() > 1 {
        warn!(
            "Multiple MainMenuMusic sinks found when entering settings! Pausing the first one found."
        );
        // Attempt to pause the first one found, though this indicates an issue elsewhere.
        if let Some(sink) = music_query.iter().next() {
            sink.pause();
        }
    }
}
