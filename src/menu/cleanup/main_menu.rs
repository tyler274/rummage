use crate::menu::{cleanup::MainMenuMusic, components::MenuItem, input_blocker::InputBlocker};
use bevy::prelude::*;

/// Cleans up main menu entities
pub fn cleanup_main_menu(
    mut commands: Commands,
    menu_items: Query<(Entity, Option<&Name>), With<MenuItem>>,
    main_menu_music: Query<Entity, With<MainMenuMusic>>,
    input_blockers: Query<Entity, With<InputBlocker>>,
) {
    let count = menu_items.iter().count();

    // Only clean up if there are items to clean up
    if count > 0 {
        info!("Cleaning up {} main menu items", count);
        for (entity, name) in menu_items.iter() {
            if let Some(name_comp) = name {
                info!("Despawning main menu entity: {:?} ({})", entity, name_comp);
            } else {
                info!("Despawning unnamed main menu entity: {:?}", entity);
            }
            commands.entity(entity).despawn_recursive();
        }
    } else {
        info!("No main menu items found to clean up");
    }

    // Clean up the main menu music
    let music_count = main_menu_music.iter().count();
    if music_count > 0 {
        debug!("Stopping main menu music");
        for entity in main_menu_music.iter() {
            info!("Despawning main menu music entity: {:?}", entity);
            commands.entity(entity).despawn();
        }
    }

    // Explicitly clean up any input blockers that might remain
    let blocker_count = input_blockers.iter().count();
    if blocker_count > 0 {
        info!("Cleaning up {} input blockers", blocker_count);
        for entity in input_blockers.iter() {
            info!("Despawning input blocker: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }

    // Log the completion of cleanup
    info!("Main menu cleanup complete");
}
