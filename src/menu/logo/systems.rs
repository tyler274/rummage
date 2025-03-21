use crate::menu::logo::components::StarOfDavid;
use bevy::prelude::*;

/// Setup the star of david for the main menu
pub fn setup_main_menu_star(_commands: Commands) {
    debug!("Setting up main menu star");
    // Implementation pending
}

/// Setup the star of david for the pause menu
pub fn setup_pause_star(_commands: Commands) {
    debug!("Setting up pause menu star");
    // Implementation pending
}

/// Cleanup the star of david thoroughly
pub fn cleanup_star_of_david_thoroughly(
    mut commands: Commands,
    stars: Query<Entity, With<StarOfDavid>>,
) {
    for entity in stars.iter() {
        commands.entity(entity).despawn_recursive();
    }
    debug!("Star of David cleaned up");
}
