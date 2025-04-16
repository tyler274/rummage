use crate::menu::components::MenuRoot;
use crate::menu::main_menu::components::{MainMenuBackground, MainMenuMusic};
use bevy::prelude::*;

/// Cleans up main menu entities including root, background, and music
pub fn cleanup_main_menu(
    mut commands: Commands,
    menu_root_query: Query<Entity, With<MenuRoot>>,
    background_query: Query<Entity, With<MainMenuBackground>>,
    music_query: Query<Entity, With<MainMenuMusic>>,
) {
    // Despawn the root entity recursively
    if let Ok(root_entity) = menu_root_query.get_single() {
        info!("Despawning main menu root entity: {:?}", root_entity);
        commands.entity(root_entity).despawn_recursive();
    } else if menu_root_query.iter().count() > 1 {
        warn!("Multiple MenuRoot entities found during cleanup! Despawning all.");
        for root_entity in menu_root_query.iter() {
            info!(
                "Despawning additional main menu root entity: {:?}",
                root_entity
            );
            commands.entity(root_entity).despawn_recursive();
        }
    } else {
        info!("No MenuRoot entity found to clean up.");
    }

    // Despawn background entities
    for entity in background_query.iter() {
        info!("Despawning main menu background entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }

    // Despawn music entities
    for entity in music_query.iter() {
        info!("Despawning main menu music entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }

    // Log the completion of cleanup
    info!("Main menu cleanup complete");
}
