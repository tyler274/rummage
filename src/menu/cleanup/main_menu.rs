use crate::menu::components::MenuRoot;
use bevy::prelude::*;

/// Cleans up main menu entities by despawning the root node
pub fn cleanup_main_menu(mut commands: Commands, menu_root_query: Query<Entity, With<MenuRoot>>) {
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

    // Log the completion of cleanup
    info!("Main menu cleanup complete");
}
