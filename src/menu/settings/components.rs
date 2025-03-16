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
    /// Show video settings
    VideoSettings,
    /// Show audio settings
    AudioSettings,
    /// Show gameplay settings
    GameplaySettings,
    /// Show controls settings
    ControlsSettings,
    /// Return to previous menu (either main menu or pause menu)
    Back,
    /// Return to main settings screen
    BackToMainSettings,
}

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
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
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
#[derive(Resource, Debug, Clone)]
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
