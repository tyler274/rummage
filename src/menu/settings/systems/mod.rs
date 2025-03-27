pub mod audio;
pub mod common;
pub mod controls;
pub mod gameplay;
pub mod main;
pub mod video;

pub use audio::*;
pub use common::*;
pub use controls::*;
pub use gameplay::*;
pub use main::*;
pub use video::*;

use crate::menu::components::MenuItem;
use crate::menu::settings::state::SettingsMenuState;
use bevy::prelude::*;

/// Cleanup the settings menu entities
pub fn cleanup_settings_menu(
    mut commands: Commands,
    settings_entities: Query<(Entity, &Name), With<MenuItem>>,
    current_settings_state: Res<State<SettingsMenuState>>,
) {
    info!(
        "Starting settings menu cleanup for state: {:?}",
        current_settings_state.get()
    );

    // Only clean up entities that belong to the current state
    let entities_to_remove: Vec<(Entity, String)> = settings_entities
        .iter()
        .filter(|(_, name)| {
            let name_str = name.to_string();
            match *current_settings_state.get() {
                SettingsMenuState::Video => name_str.contains("Video"),
                SettingsMenuState::Audio => name_str.contains("Audio"),
                SettingsMenuState::Gameplay => name_str.contains("Gameplay"),
                SettingsMenuState::Controls => name_str.contains("Controls"),
                SettingsMenuState::Main => {
                    name_str.contains("Settings")
                        && !name_str.contains("Video")
                        && !name_str.contains("Audio")
                        && !name_str.contains("Gameplay")
                        && !name_str.contains("Controls")
                }
                SettingsMenuState::Disabled => true, // Clean up everything in disabled state
            }
        })
        .map(|(entity, name)| (entity, name.to_string()))
        .collect();

    let num_entities = entities_to_remove.len();
    info!("Found {} entities to remove", num_entities);

    // Remove the entities
    for (entity, name) in entities_to_remove {
        info!("Despawning entity: '{}'", name);
        commands.entity(entity).despawn_recursive();
    }

    info!("Completed cleanup of {} entities", num_entities);
}
