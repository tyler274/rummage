pub mod audio;
pub mod common;
pub mod controls;
pub mod gameplay;
pub mod main;
pub mod state_transitions;
pub mod video;

use crate::menu::components::MenuItem;
use crate::menu::settings::state::SettingsMenuState;
use bevy::prelude::*;

/// Cleanup the settings menu entities
pub fn cleanup_settings_menu(
    mut commands: Commands,
    settings_entities: Query<(Entity, &Name, Option<&Parent>), With<MenuItem>>,
    current_settings_state: Res<State<SettingsMenuState>>,
) {
    info!(
        "Starting settings menu cleanup for state: {:?}",
        current_settings_state.get()
    );

    // First, collect all entities that match our criteria
    let entities_to_remove: Vec<(Entity, String, bool)> = settings_entities
        .iter()
        .filter(|(_, name, _)| {
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
                SettingsMenuState::Disabled => true,
            }
        })
        .map(|(entity, name, parent)| (entity, name.to_string(), parent.is_none()))
        .collect();

    let num_entities = entities_to_remove.len();
    info!("Found {} entities to remove", num_entities);

    // Only despawn root entities (those without parents)
    // Their children will be despawned automatically due to despawn_recursive
    let mut despawned = 0;
    for (entity, name, is_root) in entities_to_remove {
        if is_root {
            info!("Despawning root settings entity: '{}'", name);
            commands.entity(entity).despawn_recursive();
            despawned += 1;
        }
    }

    info!("Despawned {} root settings entities", despawned);
}
