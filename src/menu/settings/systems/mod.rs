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

/// Cleanup the settings menu entities (Reverted Signature)
pub fn cleanup_settings_menu(
    mut commands: Commands,
    settings_entities: Query<Entity, With<SettingsMenuItem>>, // Query only for Entity with the general marker
    // Re-add the State resource
    current_settings_state: Res<State<SettingsMenuState>>,
    context: Res<StateTransitionContext>,
    // Remove the exited state argument
    // exited_state: SettingsMenuState,
) {
    info!(
        "Running generic cleanup_settings_menu for state: {:?} (returning: {})",
        current_settings_state.get(),
        context.returning_from_settings
    );

    // This function might not be needed anymore if we use specific despawn systems.
    // For now, let's just despawn ALL SettingsMenuItem entities if returning.
    if context.returning_from_settings
        || *current_settings_state.get() == SettingsMenuState::Disabled
    {
        info!("Cleaning up all settings menu entities via generic cleanup");
        let mut despawned = 0;
        for entity in settings_entities.iter() {
            // Attempt to despawn - might need adjustment based on hierarchy
            // Consider despawning only root entities if this causes issues.
            commands.entity(entity).despawn_recursive();
            despawned += 1;
        }
        info!("Despawned {} entities via generic cleanup", despawned);
    }
    // Removed the logic that tried to filter based on the (incorrect) current state.
}

// Define a generic despawn function based on Bevy examples
/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    info!(
        "Despawning screen entities with component: {}",
        std::any::type_name::<T>()
    );
    let mut count = 0;
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
        count += 1;
    }
    info!("Despawned {} entities", count);
}
