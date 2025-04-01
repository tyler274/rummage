use crate::menu::{
    components::NeedsMainMenuSetup,
    settings::state::SettingsMenuState,
    state::{GameMenuState, StateTransitionContext},
};
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_persistent::prelude::*;

use super::components::*;
use super::state::*;
use super::systems::{
    audio::{setup_audio_settings, volume_slider_interaction},
    cleanup_settings_menu,
    controls::setup_controls_settings,
    gameplay::setup_gameplay_settings,
    main::{handle_settings_back_input, settings_button_action, setup_main_settings},
    video::setup_video_settings,
};
use crate::game_engine::state::GameState;
use crate::menu::components::MenuItem;

/// Plugin that sets up the settings menu system
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        info!("Building SettingsPlugin...");

        // Initialize all settings resources first
        app.init_resource::<VolumeSettings>()
            .init_resource::<GameplaySettings>()
            .init_resource::<CurrentGraphicsQuality>()
            .init_resource::<RummageSettings>();

        info!("Settings resources initialized");

        // Set up persistent settings using TOML
        match Persistent::<RummageSettings>::builder()
            .name("rummage_settings")
            .format(StorageFormat::Toml)
            .path("settings/settings.toml")
            .default(RummageSettings::default())
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
        {
            Ok(persistent_settings) => {
                // Store the persistent settings
                app.insert_resource(persistent_settings);
            }
            Err(e) => {
                error!("Failed to initialize persistent settings: {:?}", e);
                // No need to fall back as we already initialized default resources above
            }
        }

        // Register settings states
        app.init_state::<SettingsMenuState>()
            // Settings state - Main settings
            .add_systems(
                OnEnter(SettingsMenuState::Main),
                (
                    setup_main_settings,
                    /* // REMOVED: System that incorrectly forced GameMenuState to Settings
                    |mut game_state: ResMut<NextState<GameMenuState>>,
                     current_state: Res<State<GameMenuState>>| {
                        if *current_state.get() != GameMenuState::Settings {
                            game_state.set(GameMenuState::Settings);
                            info!(
                                "ENTERED SettingsMenuState::Main - Set GameMenuState to Settings"
                            );
                        }
                    }, */
                    // Only set up menu camera if we're not already in Settings state
                    crate::menu::camera::setup::setup_menu_camera.run_if(
                        |state: Res<State<GameMenuState>>| *state.get() != GameMenuState::Settings,
                    ),
                ),
            )
            // Settings state - Video settings
            .add_systems(OnEnter(SettingsMenuState::Video), setup_video_settings)
            // Settings state - Audio settings
            .add_systems(OnEnter(SettingsMenuState::Audio), setup_audio_settings)
            // Settings state - Gameplay settings
            .add_systems(
                OnEnter(SettingsMenuState::Gameplay),
                setup_gameplay_settings,
            )
            // Settings state - Controls settings
            .add_systems(
                OnEnter(SettingsMenuState::Controls),
                setup_controls_settings,
            )
            // Settings interaction system
            .add_systems(
                Update,
                (
                    settings_button_action,
                    volume_slider_interaction,
                    handle_settings_back_input.run_if(in_state(GameMenuState::Settings)),
                ),
            )
            // Apply settings on startup
            .add_systems(Startup, apply_settings)
            // Save settings when exiting any settings menu
            .add_systems(
                OnExit(SettingsMenuState::Audio),
                (
                    save_settings,
                    cleanup_settings_menu.run_if(in_state(SettingsMenuState::Disabled)),
                ),
            )
            .add_systems(
                OnExit(SettingsMenuState::Video),
                (
                    save_settings,
                    cleanup_settings_menu.run_if(in_state(SettingsMenuState::Disabled)),
                ),
            )
            .add_systems(
                OnExit(SettingsMenuState::Gameplay),
                (
                    save_settings,
                    cleanup_settings_menu.run_if(in_state(SettingsMenuState::Disabled)),
                ),
            )
            .add_systems(
                OnExit(SettingsMenuState::Controls),
                cleanup_settings_menu.run_if(in_state(SettingsMenuState::Disabled)),
            )
            .add_systems(
                OnExit(SettingsMenuState::Main),
                cleanup_settings_menu.run_if(in_state(SettingsMenuState::Disabled)),
            )
            // REMOVED: Cleanup system for the main settings menu (now handled by OnExit(SettingsMenuState::*))
            /* .add_systems(OnExit(GameMenuState::Settings), settings_cleanup) */;
    }
}

#[derive(Resource, Clone)]
pub struct CurrentGraphicsQuality {
    pub quality: GraphicsQuality,
}

impl Default for CurrentGraphicsQuality {
    fn default() -> Self {
        Self {
            quality: GraphicsQuality::Medium,
        }
    }
}

/// Apply saved settings on startup
pub fn apply_settings(
    persistent_settings: Option<Res<Persistent<RummageSettings>>>,
    mut volume_settings: ResMut<VolumeSettings>,
    mut gameplay_settings: ResMut<GameplaySettings>,
    mut graphics_quality: ResMut<CurrentGraphicsQuality>,
    mut global_volume: ResMut<bevy::prelude::GlobalVolume>,
) {
    if let Some(settings) = persistent_settings {
        // Apply volume settings
        volume_settings.master = settings.volume.master;
        volume_settings.music = settings.volume.music;
        volume_settings.sfx = settings.volume.sfx;

        // Apply gameplay settings
        gameplay_settings.auto_pass = settings.gameplay.auto_pass;
        gameplay_settings.show_tooltips = settings.gameplay.show_tooltips;
        gameplay_settings.animation_speed = settings.gameplay.animation_speed;

        // Apply graphics quality
        graphics_quality.quality = settings.graphics.clone();

        // Apply global volume
        global_volume.volume = Volume::new(volume_settings.master);

        info!("Applied settings from persistent storage");
    } else {
        info!("No persistent settings found, using defaults");
    }
}

/// Save current settings to persistent storage
fn save_settings(
    persistent_settings: Option<ResMut<Persistent<RummageSettings>>>,
    volume_settings: Res<VolumeSettings>,
    gameplay_settings: Res<GameplaySettings>,
    graphics_quality: Res<CurrentGraphicsQuality>,
) {
    if let Some(mut settings) = persistent_settings {
        // Update settings with current values
        settings.volume = volume_settings.clone();
        settings.gameplay = gameplay_settings.clone();
        settings.graphics = graphics_quality.quality.clone();

        info!("Saving settings: {:?}", settings.as_ref());
    } else {
        warn!("No persistent settings resource found, settings not saved");
    }
}
