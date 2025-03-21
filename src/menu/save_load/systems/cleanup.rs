use crate::menu::save_load::components::SaveLoadUi;
use bevy::prelude::*;

/// Cleans up the save/load UI when exiting the save/load state
pub fn cleanup_save_load_ui(mut commands: Commands, query: Query<Entity, With<SaveLoadUi>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
