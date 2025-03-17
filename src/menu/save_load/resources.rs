use bevy::prelude::*;

/// State for the save/load UI
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SaveLoadUiState {
    /// UI is hidden
    #[default]
    Hidden,
    /// Showing save game dialog
    SaveGame,
    /// Showing load game dialog
    LoadGame,
}

/// Resource to track the current state of the save/load UI
#[derive(Resource, Default)]
pub struct SaveLoadUiContext {
    /// Whether the save/load UI was opened from the pause menu
    pub from_pause_menu: bool,
    /// The last entered save slot name
    pub last_save_slot: Option<String>,
    /// The current selected save slot
    pub selected_slot: Option<String>,
}
