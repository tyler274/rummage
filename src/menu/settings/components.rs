use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for settings menu entities
#[derive(Component)]
pub struct SettingsMenuItem;

/// Marker component for main settings screen
#[derive(Component)]
pub struct MainSettingsScreen;

/// Marker component for video settings screen
#[derive(Component)]
pub struct VideoSettingsScreen;

/// Marker component for audio settings screen
#[derive(Component)]
pub struct AudioSettingsScreen;

/// Marker component for gameplay settings screen
#[derive(Component)]
pub struct GameplaySettingsScreen;

/// Marker component for controls settings screen
#[derive(Component)]
pub struct ControlsSettingsScreen;

/// Settings button actions for navigating between settings screens
#[derive(Component, Clone, Copy, Debug)]
pub enum SettingsButtonAction {
    /// Navigate to video settings
    NavigateToVideo,
    /// Navigate to audio settings
    NavigateToAudio,
    /// Navigate to gameplay settings
    NavigateToGameplay,
    /// Navigate to controls settings
    NavigateToControls,
    /// Navigate to main settings
    NavigateToMain,
    /// Exit settings menu
    ExitSettings,
}

/// Component to associate a button with a specific GraphicsQuality
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct QualityButton(pub GraphicsQuality);

/// Volume settings resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct VolumeSettings {
    /// Master volume level (0.0 - 1.0)
    pub master: f32,
    /// Music volume level (0.0 - 1.0)
    pub music: f32,
    /// Sound effects volume level (0.0 - 1.0)
    pub sfx: f32,
}

impl Default for VolumeSettings {
    fn default() -> Self {
        Self {
            master: 0.7,
            music: 0.5,
            sfx: 0.5,
        }
    }
}

/// Graphics quality settings
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum GraphicsQuality {
    /// Low quality graphics for performance
    Low,
    /// Medium quality graphics balancing performance and visuals
    Medium,
    /// High quality graphics for best visuals
    High,
}

impl Default for GraphicsQuality {
    fn default() -> Self {
        Self::Medium
    }
}

/// Gameplay settings resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameplaySettings {
    /// Enable auto-pass priority when no valid actions
    pub auto_pass: bool,
    /// Show card tooltips on hover
    pub show_tooltips: bool,
    /// Animation speed multiplier
    pub animation_speed: f32,
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            auto_pass: true,
            show_tooltips: true,
            animation_speed: 1.0,
        }
    }
}

/// Combined settings that will be saved to TOML
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RummageSettings {
    /// Volume settings
    pub volume: VolumeSettings,
    /// Graphics settings
    pub graphics: GraphicsQuality,
    /// Gameplay settings
    pub gameplay: GameplaySettings,
}

/* impl Default for RummageSettings {
    fn default() -> Self {
        Self {
            volume: VolumeSettings::default(),
            graphics: GraphicsQuality::default(),
            gameplay: GameplaySettings::default(),
        }
    }
} */
