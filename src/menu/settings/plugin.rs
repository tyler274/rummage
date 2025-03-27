use crate::menu::{
    components::NeedsMainMenuSetup,
    settings::state::SettingsMenuState,
    state::{GameMenuState, StateTransitionContext},
};
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_persistent::prelude::*;

use super::components::*;
use super::systems::*;
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
                    crate::menu::camera::setup::setup_menu_camera,
                    |mut commands: Commands| {
                        // Add an input blocker to prevent clicks going through to other UI elements
                        commands.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            crate::menu::components::MenuItem,
                            crate::menu::settings::components::SettingsMenuItem,
                            crate::menu::input_blocker::InputBlocker,
                            Visibility::Visible,
                            InheritedVisibility::VISIBLE,
                            Name::new("Settings Menu Input Blocker"),
                        ));
                    },
                    setup_main_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>,
                     current_game_state: Res<State<GameMenuState>>| {
                        // Force the game state to remain in Settings when entering main settings
                        if *current_game_state.get() != GameMenuState::Settings {
                            info!("Fixing GameMenuState to Settings for main settings");
                        }
                        game_state.set(GameMenuState::Settings);
                        info!("ENTERED SettingsMenuState::Main - Current Settings State: Main, Game State: Settings");
                    },
                ),
            )
            // Settings state - Video settings
            .add_systems(
                OnEnter(SettingsMenuState::Video),
                (
                    setup_video_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>| {
                        game_state.set(GameMenuState::Settings);
                    },
                ),
            )
            // Settings state - Audio settings
            .add_systems(
                OnEnter(SettingsMenuState::Audio),
                (
                    setup_audio_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>| {
                        game_state.set(GameMenuState::Settings);
                    },
                ),
            )
            // Settings state - Gameplay settings
            .add_systems(
                OnEnter(SettingsMenuState::Gameplay),
                (
                    setup_gameplay_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>| {
                        game_state.set(GameMenuState::Settings);
                    },
                ),
            )
            // Settings state - Controls settings
            .add_systems(
                OnEnter(SettingsMenuState::Controls),
                (
                    setup_controls_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>| {
                        game_state.set(GameMenuState::Settings);
                    },
                ),
            )
            // Settings interaction system
            .add_systems(
                Update,
                (settings_button_action, volume_slider_interaction),
            )
            // Apply settings on startup
            .add_systems(Startup, apply_settings)
            // Save settings when exiting any settings menu
            .add_systems(OnExit(SettingsMenuState::Audio), save_settings)
            .add_systems(OnExit(SettingsMenuState::Video), save_settings)
            .add_systems(OnExit(SettingsMenuState::Gameplay), save_settings)
            // Cleanup systems for each settings state
            .add_systems(
                OnExit(SettingsMenuState::Main),
                cleanup_settings_menu.run_if(not(in_state(GameMenuState::Settings))),
            )
            .add_systems(
                OnExit(SettingsMenuState::Video),
                cleanup_settings_menu.run_if(not(in_state(GameMenuState::Settings))),
            )
            .add_systems(
                OnExit(SettingsMenuState::Audio),
                cleanup_settings_menu.run_if(not(in_state(GameMenuState::Settings))),
            )
            .add_systems(
                OnExit(SettingsMenuState::Gameplay),
                cleanup_settings_menu.run_if(not(in_state(GameMenuState::Settings))),
            )
            .add_systems(
                OnExit(SettingsMenuState::Controls),
                cleanup_settings_menu.run_if(not(in_state(GameMenuState::Settings))),
            )
            // Cleanup system for the main settings menu
            .add_systems(OnExit(GameMenuState::Settings), settings_cleanup);
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

/// Cleanup settings menu entities and handle state transitions
fn settings_cleanup(
    mut commands: Commands,
    settings_entities: Query<(Entity, &Name), With<MenuItem>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    current_state: Res<State<GameMenuState>>,
    current_settings_state: Res<State<SettingsMenuState>>,
) {
    info!(
        "Starting settings cleanup with current state: {:?}",
        current_settings_state.get()
    );

    // First, collect all entities to remove to avoid any ordering issues
    let entities_to_remove: Vec<(Entity, String)> = settings_entities
        .iter()
        .filter(|(_, name)| {
            let name_str = name.to_string();
            name_str.contains("Settings")
                || name_str.contains("Option")
                || name_str.contains("Slider")
                || name_str.contains("Checkbox")
                || name_str.contains("settings")
                || name_str.contains("Input Blocker")
        })
        .map(|(entity, name)| (entity, name.to_string()))
        .collect();

    let num_entities = entities_to_remove.len();
    info!("Found {} settings entities to remove", num_entities);

    // Log what we're about to remove and remove them
    for (entity, name) in entities_to_remove {
        info!("Despawning settings entity: '{}'", name);
        commands.entity(entity).despawn_recursive();
    }

    info!("Despawned {} settings menu entities", num_entities);

    // Only handle state transitions if we're in the Disabled state
    if *current_settings_state.get() == SettingsMenuState::Disabled {
        // Return to the origin state after settings
        if let Some(origin) = context.settings_origin {
            info!("Returning to origin state: {:?}", origin);
            next_state.set(origin);
            if origin == GameMenuState::MainMenu {
                // Mark that the main menu needs setup when returning from settings
                info!("Setting NeedsMainMenuSetup flag to true");
                commands.insert_resource(NeedsMainMenuSetup(true));
            }
            context.settings_origin = None;
        }
    }
}
