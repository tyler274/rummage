use bevy::prelude::*;

use crate::menu::state::GameMenuState;

use super::components::*;
use super::state::SettingsMenuState;
use super::systems::*;

/// Plugin that sets up the settings menu system
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // Register settings states
        app.init_state::<SettingsMenuState>()
            // Register default resources
            .init_resource::<VolumeSettings>()
            .init_resource::<GameplaySettings>()
            .insert_resource(GraphicsQuality::default())
            
            // Settings state - Main screen
            .add_systems(OnEnter(SettingsMenuState::Main), setup_main_settings)
            
            // Settings state - Video settings
            .add_systems(OnEnter(SettingsMenuState::Video), setup_video_settings)
            
            // Settings state - Audio settings
            .add_systems(OnEnter(SettingsMenuState::Audio), setup_audio_settings)
            
            // Settings state - Gameplay settings
            .add_systems(OnEnter(SettingsMenuState::Gameplay), setup_gameplay_settings)
            
            // Settings state - Controls settings
            .add_systems(OnEnter(SettingsMenuState::Controls), setup_controls_settings)
            
            // Settings interaction system
            .add_systems(
                Update,
                settings_button_action.run_if(in_state(GameMenuState::Settings)),
            )
            
            // Cleanup systems for each settings state exit
            .add_systems(OnExit(SettingsMenuState::Video), cleanup_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Audio), cleanup_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Gameplay), cleanup_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Controls), cleanup_settings_menu)
            
            // Cleanup system for the main settings menu
            .add_systems(OnExit(GameMenuState::Settings), cleanup_settings_menu);
    }
}
