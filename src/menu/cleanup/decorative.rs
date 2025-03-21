use crate::menu::{components::MenuCamera, logo::StarOfDavid};
use bevy::prelude::*;

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
///
/// This is a utility function for cleaning up star of david entities when they are no longer needed.
/// The more thorough version (cleanup_star_of_david_thoroughly) is currently used instead.
#[allow(dead_code)]
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
    // Clean up complete entities with children
    let count = stars_query.iter().count();
    info!("Cleaning up {} Star of David entities thoroughly", count);

    // First, find all StarOfDavid entities that have children
    for entity in &stars_query {
        if let Ok(children) = children_query.get(entity) {
            info!(
                "Found StarOfDavid entity {:?} with {} children",
                entity,
                children.len()
            );

            // Despawn the entity and all of its children
            commands.entity(entity).despawn_recursive();
        } else {
            // No children, just despawn the entity itself
            commands.entity(entity).despawn();
        }
    }

    // Look for any detached StarOfDavid components
    for (entity, _) in star_components.iter() {
        info!(
            "Found detached StarOfDavid component on entity {:?}",
            entity
        );
        commands.entity(entity).despawn_recursive();
    }
}
