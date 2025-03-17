use bevy::prelude::*;

/// Marker component for save/load UI
#[derive(Component)]
pub struct SaveLoadUi;

/// Marker component for save game panel
#[derive(Component)]
pub struct SaveGamePanel;

/// Marker component for load game panel
#[derive(Component)]
pub struct LoadGamePanel;

/// Marker component for save/load slot button
#[derive(Component)]
pub struct SaveSlotButton {
    /// The name of the save slot
    pub slot_name: String,
}

/// Button actions specific to save/load UI
#[derive(Component, Clone, Debug)]
pub enum SaveLoadButtonAction {
    /// Save to a specific slot
    SaveToSlot(String),
    /// Load from a specific slot
    LoadFromSlot(String),
    /// Create a new save slot
    CreateSaveSlot,
    /// Cancel and close the save/load dialog
    Cancel,
}
