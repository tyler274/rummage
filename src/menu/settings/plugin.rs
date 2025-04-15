use crate::menu::{settings::state::SettingsMenuState, state::GameMenuState};
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_persistent::prelude::*;

use super::components::*;
use super::systems::{
    audio::{
        VolumeUpdateRequests, apply_volume_updates, setup_audio_settings, volume_slider_interaction,
    },
    cleanup_settings_menu,
    controls::setup_controls_settings,
    gameplay::setup_gameplay_settings,
    main::{handle_settings_back_input, settings_button_action, setup_main_settings},
    video::setup_video_settings,
};

/// Plugin that sets up the settings menu system
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        info!("Building SettingsPlugin...");

        // Initialize all settings resources first
        app.init_resource::<VolumeSettings>()
            .init_resource::<GameplaySettings>()
            .init_resource::<CurrentGraphicsQuality>()
            .init_resource::<RummageSettings>()
            .init_resource::<VolumeUpdateRequests>();

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
                    apply_volume_updates,
                ),
            )
            // Add handle_settings_back_input separately with its condition
            .add_systems(
                Update,
                handle_settings_back_input.run_if(in_state(GameMenuState::Settings)),
            )
            // Apply settings on startup
            .add_systems(Startup, apply_settings)
            // Save settings and cleanup when exiting any settings menu
            // Run cleanup in a fixed order to prevent race conditions
            .add_systems(
                OnExit(SettingsMenuState::Audio),
                (save_settings, cleanup_settings_menu).chain(),
            )
            .add_systems(
                OnExit(SettingsMenuState::Video),
                (save_settings, cleanup_settings_menu).chain(),
            )
            .add_systems(
                OnExit(SettingsMenuState::Gameplay),
                (save_settings, cleanup_settings_menu).chain(),
            )
            .add_systems(OnExit(SettingsMenuState::Controls), cleanup_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Main), cleanup_settings_menu)
            // Add cleanup for Disabled state to ensure complete cleanup
            // This should run after any other cleanup systems
            .add_systems(
                OnExit(SettingsMenuState::Disabled),
                cleanup_settings_menu.after(save_settings),
            );
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
fn apply_settings(
    mut volume_settings: ResMut<VolumeSettings>,
    mut gameplay_settings: ResMut<GameplaySettings>,
    mut graphics_quality: ResMut<CurrentGraphicsQuality>,
    persistent_settings: Res<Persistent<RummageSettings>>,
) {
    info!("Applying saved settings");

    // Apply volume settings
    volume_settings.master = persistent_settings.volume.master;
    volume_settings.music = persistent_settings.volume.music;
    volume_settings.sfx = persistent_settings.volume.sfx;

    // Apply gameplay settings
    gameplay_settings.auto_pass = persistent_settings.gameplay.auto_pass;
    gameplay_settings.show_tooltips = persistent_settings.gameplay.show_tooltips;

    // Apply graphics settings
    graphics_quality.quality = persistent_settings.graphics;

    info!("Settings applied successfully");
}

/// Save current settings to persistent storage
fn save_settings(
    volume_settings: Res<VolumeSettings>,
    gameplay_settings: Res<GameplaySettings>,
    graphics_quality: Res<CurrentGraphicsQuality>,
    mut persistent_settings: ResMut<Persistent<RummageSettings>>,
) {
    info!("Saving current settings");

    // Save volume settings
    persistent_settings.volume.master = volume_settings.master;
    persistent_settings.volume.music = volume_settings.music;
    persistent_settings.volume.sfx = volume_settings.sfx;

    // Save gameplay settings
    persistent_settings.gameplay.auto_pass = gameplay_settings.auto_pass;
    persistent_settings.gameplay.show_tooltips = gameplay_settings.show_tooltips;

    // Save graphics settings
    persistent_settings.graphics = graphics_quality.quality;

    // Persist changes to disk
    if let Err(e) = persistent_settings.persist() {
        error!("Failed to save settings: {:?}", e);
    } else {
        info!("Settings saved successfully");
    }
}
