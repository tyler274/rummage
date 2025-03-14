use bevy::prelude::*;

/// Settings menu states for navigating between different settings screens
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SettingsMenuState {
    /// Main settings screen with categories
    Main,
    /// Video settings screen
    Video,
    /// Audio settings screen
    Audio,
    /// Gameplay settings screen
    Gameplay,
    /// Controls settings screen
    Controls,
    /// Disabled state - no UI is shown
    #[default]
    Disabled,
}
