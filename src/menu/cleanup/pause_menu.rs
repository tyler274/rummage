use crate::menu::{
    components::MenuItem, decorations::MenuDecorativeElement, star_of_david::StarOfDavid,
};
use bevy::prelude::*;

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
