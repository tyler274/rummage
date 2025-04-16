use crate::menu::{components::MenuItem, decorations::MenuDecorativeElement};
use bevy::prelude::*;

/// Cleans up pause menu entities
pub fn cleanup_pause_menu(
    mut commands: Commands,
    menu_items: Query<Entity, With<MenuItem>>,
    decorative_elements: Query<Entity, With<MenuDecorativeElement>>,
) {
    let item_count = menu_items.iter().count();
    if item_count > 0 {
        info!("Cleaning up {} pause menu items", item_count);
        for entity in menu_items.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Despawn decorative elements as well
    let element_count = decorative_elements.iter().count();
    if element_count > 0 {
        info!(
            "Cleaning up {} pause menu decorative elements",
            element_count
        );
        for entity in decorative_elements.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
