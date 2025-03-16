use bevy::prelude::*;
use bevy_persistent::prelude::*;

use crate::menu::state::GameMenuState;

use super::components::*;
use super::state::SettingsMenuState;
use super::systems::*;

/// Plugin that sets up the settings menu system
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the VolumeSettings resource first
        app.init_resource::<VolumeSettings>();
        
        // Set up persistent volume settings
        match Persistent::<VolumeSettings>::builder()
            .name("volume_settings")
            .format(StorageFormat::Bincode)
            .path("settings/volume.bin")
            .default(VolumeSettings::default())
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
        {
            Ok(persistent_volume) => {
                // Store the persistent volume settings
                app.insert_resource(persistent_volume);
            }
            Err(e) => {
                error!("Failed to initialize persistent volume settings: {:?}", e);
                // No need to fall back as we already initialized VolumeSettings above
            }
        }

        // Register settings states
        app.init_state::<SettingsMenuState>()
            // Register other resources
            .init_resource::<GameplaySettings>()
            .insert_resource(GraphicsQuality::default())
            // Settings state - Main screen
            .add_systems(
                OnEnter(SettingsMenuState::Main), 
                (
                    setup_main_settings,
                    |state: Res<State<SettingsMenuState>>, game_state: Res<State<GameMenuState>>| {
                        info!("ENTERED SettingsMenuState::Main - Current Settings State: {:?}, Game State: {:?}", 
                              state.get(), game_state.get());
                    }
                ).run_if(in_state(GameMenuState::Settings))
            )
            // Settings state - Video settings
            .add_systems(
                OnEnter(SettingsMenuState::Video), 
                setup_video_settings.run_if(in_state(GameMenuState::Settings))
            )
            // Settings state - Audio settings
            .add_systems(
                OnEnter(SettingsMenuState::Audio), 
                setup_audio_settings.run_if(in_state(GameMenuState::Settings))
            )
            // Settings state - Gameplay settings
            .add_systems(
                OnEnter(SettingsMenuState::Gameplay),
                setup_gameplay_settings.run_if(in_state(GameMenuState::Settings)),
            )
            // Settings state - Controls settings
            .add_systems(
                OnEnter(SettingsMenuState::Controls),
                setup_controls_settings.run_if(in_state(GameMenuState::Settings)),
            )
            // Settings interaction system
            .add_systems(
                Update,
                (
                    settings_button_action,
                    volume_slider_interaction
                ).run_if(in_state(GameMenuState::Settings)),
            )
            // Apply volume settings on startup
            .add_systems(Startup, apply_volume_settings)
            // Apply volume changes when leaving the audio settings menu
            .add_systems(OnExit(SettingsMenuState::Audio), save_volume_settings)
            // Cleanup systems for each settings state exit
            .add_systems(OnExit(SettingsMenuState::Video), cleanup_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Audio), cleanup_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Gameplay), cleanup_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Controls), cleanup_settings_menu)
            // Cleanup system for the main settings menu
            .add_systems(OnExit(GameMenuState::Settings), cleanup_settings_menu);
    }
}

/// Apply saved volume settings on startup
fn apply_volume_settings(
    persistent_volume: Option<Res<Persistent<VolumeSettings>>>, 
    mut volume_settings: ResMut<VolumeSettings>,
    mut global_volume: ResMut<bevy::prelude::GlobalVolume>
) {
    if let Some(persistent) = persistent_volume {
        info!("Applying saved volume settings: {:?}", *persistent);
        
        // Update the actual settings resource 
        *volume_settings = (*persistent).clone();
        
        // Apply to global volume
        global_volume.volume = bevy::audio::Volume::new(volume_settings.master);
    } else {
        info!("No persistent volume settings available, using defaults");
        global_volume.volume = bevy::audio::Volume::new(volume_settings.master);
    }
}

/// Save volume settings when leaving the audio settings menu
fn save_volume_settings(
    persistent_volume: Option<ResMut<Persistent<VolumeSettings>>>,
    volume_settings: Res<VolumeSettings>
) {
    if let Some(mut persistent) = persistent_volume {
        info!("Saving volume settings: {:?}", *volume_settings);
        if let Err(e) = persistent.set(volume_settings.clone()) {
            error!("Failed to save volume settings: {:?}", e);
        }
    } else {
        warn!("Cannot save volume settings: persistent storage not available");
    }
}
