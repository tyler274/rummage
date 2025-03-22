use bevy::prelude::*;

use super::resources::*;
use super::systems::*;

/// Plugin that adds save/load UI functionality to the game
pub struct SaveLoadUiPlugin;

impl Plugin for SaveLoadUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SaveLoadUiState>()
            .init_resource::<SaveLoadUiContext>()
            .init_resource::<SaveExists>()
            // Setup UI when entering the appropriate SaveLoadUiState
            .add_systems(OnEnter(SaveLoadUiState::SaveGame), setup_save_dialog)
            .add_systems(OnEnter(SaveLoadUiState::LoadGame), setup_load_dialog)
            // Clean up UI when exiting the SaveLoadUiState
            .add_systems(OnExit(SaveLoadUiState::SaveGame), cleanup_save_load_ui)
            .add_systems(OnExit(SaveLoadUiState::LoadGame), cleanup_save_load_ui)
            // Button interaction system
            .add_systems(
                Update,
                handle_save_load_buttons.run_if(|state: Res<State<SaveLoadUiState>>| {
                    *state.get() != SaveLoadUiState::Hidden
                }),
            );

        info!("Save/Load UI plugin registered with SaveExists resource");
    }
}
