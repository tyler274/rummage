use bevy::prelude::*;

/// Settings menu states for navigating between different settings screens
#[derive(States, Resource, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SettingsMenuState {
    /// Main settings menu
    Main,
    /// Video settings submenu
    Video,
    /// Audio settings submenu
    Audio,
    /// Gameplay settings submenu
    Gameplay,
    /// Controls settings submenu
    Controls,
    /// Disabled state - no UI is shown
    #[default]
    Disabled,
}

impl SettingsMenuState {
    /// Get a user-friendly name for the settings state
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Main => "Main Settings",
            Self::Video => "Video Settings",
            Self::Audio => "Audio Settings",
            Self::Gameplay => "Gameplay Settings",
            Self::Controls => "Controls Settings",
            Self::Disabled => "Settings Disabled",
        }
    }
}
