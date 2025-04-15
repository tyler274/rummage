pub mod audio;
pub mod common;
pub mod controls;
pub mod gameplay;
pub mod main;
pub mod state_transitions;
pub mod video;

use crate::menu::settings::components::SettingsMenuItem;
use crate::menu::settings::state::SettingsMenuState;
use crate::menu::state::StateTransitionContext;
use bevy::prelude::*;

/// Cleanup the settings menu entities based on the state being exited
pub fn cleanup_settings_menu(
    mut commands: Commands,
    settings_entities: Query<(Entity, &Name, Option<&Parent>), With<SettingsMenuItem>>,
    context: Res<StateTransitionContext>,
    exited_state: SettingsMenuState,
) {
    info!(
        "Starting settings menu cleanup for exited state: {:?}",
        exited_state
    );

    // If we're not returning from settings, only clean up the specific submenu
    if !context.returning_from_settings && exited_state != SettingsMenuState::Disabled {
        // First, collect all entities that match our criteria for the EXITED state
        let entities_to_remove: Vec<(Entity, String, bool)> = settings_entities
            .iter()
            .filter(|(_, name, _)| {
                let name_str = name.to_string();
                // Use the EXITED state here
                match exited_state {
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
                    SettingsMenuState::Disabled => false, // Should not happen in this branch
                }
            })
            .map(|(entity, name, parent)| (entity, name.to_string(), parent.is_none()))
            .collect();

        let num_entities = entities_to_remove.len();
        info!(
            "Found {} entities to remove for exited state {:?}",
            num_entities, exited_state
        );

        // Only despawn root entities (those without parents)
        let mut despawned = 0;
        for (entity, name, is_root) in entities_to_remove {
            if is_root {
                debug!("Despawning root entity: {}", name);
                commands.entity(entity).despawn_recursive();
                despawned += 1;
            }
        }
        info!(
            "Despawned {} root entities for exited state {:?}",
            despawned, exited_state
        );
    } else if context.returning_from_settings || exited_state == SettingsMenuState::Disabled {
        // When returning to main menu/pause menu or exiting Disabled, clean up everything
        info!(
            "Cleaning up all settings menu entities (exited: {:?}, returning: {})",
            exited_state, context.returning_from_settings
        );
        let mut despawned = 0;
        for (entity, name, parent) in settings_entities.iter() {
            if parent.is_none() {
                debug!("Despawning root entity: {}", name);
                commands.entity(entity).despawn_recursive();
                despawned += 1;
            }
        }
        info!("Despawned {} total root entities", despawned);
    }
}
