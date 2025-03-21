use crate::{
    camera::components::GameCamera,
    cards::Card,
    menu::{
        components::{MenuCamera, MenuDecorativeElement, MenuItem},
        input_blocker::InputBlocker,
        logo::StarOfDavid,
    },
};
use bevy::prelude::*;

/// Component to mark the main menu music entity
#[derive(Component)]
pub struct MainMenuMusic;

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

/// Cleans up pause menu entities
pub fn cleanup_pause_menu(
    mut commands: Commands,
    menu_items: Query<Entity, With<MenuItem>>,
    decorative_elements: Query<(Entity, Option<&StarOfDavid>), With<MenuDecorativeElement>>,
) {
    let count = menu_items.iter().count();
    info!("Cleaning up {} pause menu items", count);
    for entity in menu_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Hide decorative elements (including Stars of David) rather than despawning them
    // This allows us to reuse them when returning to main menu
    let element_count = decorative_elements.iter().count();
    if element_count > 0 {
        info!(
            "Setting {} decorative elements to Hidden visibility",
            element_count
        );
        for (entity, star) in decorative_elements.iter() {
            // Log if it's a Star of David
            if star.is_some() {
                info!("Setting Star of David {:?} to Hidden", entity);
            } else {
                debug!("Setting decorative element {:?} to Hidden", entity);
            }
            // Just change visibility instead of despawning
            commands.entity(entity).insert(Visibility::Hidden);
        }
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
///
/// This is a utility function for cleaning up star of david entities when they are no longer needed.
/// The more thorough version (cleanup_star_of_david_thoroughly) is currently used instead.
///
/// TODO: Evaluate if this simplified version should be used in certain scenarios for performance reasons
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

/// System to clean up temporary elements when leaving the card selection screen
#[allow(dead_code)]
pub fn cleanup_card_selection(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // ... existing code ...
}
