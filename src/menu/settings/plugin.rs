use crate::menu::state::StateTransitionContext;
use bevy::prelude::*;
use bevy_persistent::prelude::*;

use crate::menu::components::MenuItem;
use crate::menu::state::GameMenuState;

use super::components::*;
use super::state::SettingsMenuState;
use super::systems::*;

/// Plugin that sets up the settings menu system
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        info!("Building SettingsPlugin...");
        
        // Initialize all settings resources first
        app.init_resource::<VolumeSettings>()
            .init_resource::<GameplaySettings>()
            .init_resource::<GraphicsQuality>()
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
            // Settings state - Main screen
            .add_systems(
                OnEnter(SettingsMenuState::Main),
                (
                    // Ensure we set up the menu camera before spawning UI elements
                    crate::menu::camera::setup::setup_menu_camera,
                    setup_main_settings,
                    |state: Res<State<SettingsMenuState>>, game_state: Res<State<GameMenuState>>, mut next_game_state: ResMut<NextState<GameMenuState>>| {
                        info!("ENTERED SettingsMenuState::Main - Current Settings State: {:?}, Game State: {:?}", 
                              state.get(), game_state.get());
                        // Ensure we're in the Settings game state
                        if *game_state.get() != GameMenuState::Settings {
                            info!("Fixing GameMenuState to Settings for main settings");
                            next_game_state.set(GameMenuState::Settings);
                        }
                    }
                )
            )
            // Settings state - Video settings
            .add_systems(
                OnEnter(SettingsMenuState::Video),
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
                            Name::new("Video Settings Input Blocker")
                        ));
                    },
                    setup_video_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>, current_game_state: Res<State<GameMenuState>>| {
                        // Force the game state to remain in Settings when entering Video submenu
                        if *current_game_state.get() != GameMenuState::Settings {
                            info!("Fixing GameMenuState to Settings for video settings");
                        }
                        game_state.set(GameMenuState::Settings);
                        info!("Entering Video settings - ensuring GameMenuState is Settings");
                    }
                ).run_if(in_state(GameMenuState::Settings))
            )
            // Settings state - Audio settings
            .add_systems(
                OnEnter(SettingsMenuState::Audio),
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
                            Name::new("Audio Settings Input Blocker")
                        ));
                    },
                    setup_audio_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>, current_game_state: Res<State<GameMenuState>>| {
                        // Force the game state to remain in Settings when entering Audio submenu
                        if *current_game_state.get() != GameMenuState::Settings {
                            info!("Fixing GameMenuState to Settings for audio settings");
                        }
                        game_state.set(GameMenuState::Settings);
                        info!("Entering Audio settings - ensuring GameMenuState is Settings");
                    }
                ).run_if(in_state(GameMenuState::Settings))
            )
            // Settings state - Gameplay settings
            .add_systems(
                OnEnter(SettingsMenuState::Gameplay),
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
                            Name::new("Gameplay Settings Input Blocker")
                        ));
                    },
                    setup_gameplay_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>, current_game_state: Res<State<GameMenuState>>| {
                        // Force the game state to remain in Settings when entering Gameplay submenu
                        if *current_game_state.get() != GameMenuState::Settings {
                            info!("Fixing GameMenuState to Settings for gameplay settings");
                        }
                        game_state.set(GameMenuState::Settings);
                        info!("Entering Gameplay settings - ensuring GameMenuState is Settings");
                    }
                ).run_if(in_state(GameMenuState::Settings))
            )
            // Settings state - Controls settings
            .add_systems(
                OnEnter(SettingsMenuState::Controls),
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
                            Name::new("Controls Settings Input Blocker")
                        ));
                    },
                    setup_controls_settings,
                    |mut game_state: ResMut<NextState<GameMenuState>>, current_game_state: Res<State<GameMenuState>>| {
                        // Force the game state to remain in Settings when entering Controls submenu
                        if *current_game_state.get() != GameMenuState::Settings {
                            info!("Fixing GameMenuState to Settings for controls settings");
                        }
                        game_state.set(GameMenuState::Settings);
                        info!("Entering Controls settings - ensuring GameMenuState is Settings");
                    }
                ).run_if(in_state(GameMenuState::Settings))
            )
            // Settings interaction system
            .add_systems(
                Update,
                (
                    settings_button_action,
                    volume_slider_interaction
                )
            )
            // Apply settings on startup
            .add_systems(Startup, apply_settings)
            // Save settings when exiting any settings menu
            .add_systems(OnExit(SettingsMenuState::Audio), save_settings)
            .add_systems(OnExit(SettingsMenuState::Video), save_settings)
            .add_systems(OnExit(SettingsMenuState::Gameplay), save_settings)
            // Cleanup systems for each settings state exit
            .add_systems(OnExit(SettingsMenuState::Video), cleanup_settings_menu
                .run_if(not(in_state(GameMenuState::Settings)))
            )
            .add_systems(OnExit(SettingsMenuState::Audio), cleanup_settings_menu
                .run_if(not(in_state(GameMenuState::Settings)))
            )
            .add_systems(OnExit(SettingsMenuState::Gameplay), cleanup_settings_menu
                .run_if(not(in_state(GameMenuState::Settings)))
            )
            .add_systems(OnExit(SettingsMenuState::Controls), cleanup_settings_menu
                .run_if(not(in_state(GameMenuState::Settings)))
            )
            // Cleanup system for the main settings menu
            .add_systems(OnExit(GameMenuState::Settings), (
                settings_cleanup,
                // Run a separate system to update visibility after cleanup
                |world: &mut World| {
                    info!("Running visibility update system");
                    // Find all MenuItem entities that still exist
                    let mut query = world.query_filtered::<Entity, With<MenuItem>>();
                    let entities: Vec<Entity> = query.iter(world).collect();
                    
                    info!("Found {} menu entities to update visibility", entities.len());
                    
                    // Update visibility for each entity that exists
                    for entity in entities {
                        if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
                            // Get the name for logging if available
                            let name_str = if let Some(name) = entity_mut.get::<Name>() {
                                name.as_str().to_string()
                            } else {
                                "unnamed".to_string()
                            };
                            
                            // Log what we're doing
                            info!("Setting entity {} ({}) to Visible", entity.index(), name_str);
                            
                            // Update visibility
                            entity_mut.insert(Visibility::Visible);
                        }
                    }
                    
                    info!("Finished updating visibility for remaining menu entities");
                }
            ).chain());
    }
}

/// Apply saved settings on startup
fn apply_settings(
    persistent_settings: Option<Res<Persistent<RummageSettings>>>,
    mut volume_settings: ResMut<VolumeSettings>,
    mut gameplay_settings: ResMut<GameplaySettings>,
    mut graphics_quality: ResMut<GraphicsQuality>,
    mut global_volume: ResMut<bevy::prelude::GlobalVolume>,
) {
    if let Some(persistent) = persistent_settings {
        info!("Applying saved settings: {:?}", *persistent);

        // Update all settings resources
        *volume_settings = persistent.volume.clone();
        *gameplay_settings = persistent.gameplay.clone();
        *graphics_quality = persistent.graphics.clone();

        // Apply volume settings
        global_volume.volume = bevy::audio::Volume::new(volume_settings.master);
    } else {
        info!("No persistent settings available, using defaults");
        global_volume.volume = bevy::audio::Volume::new(volume_settings.master);
    }
}

/// Save all settings when exiting any settings menu
fn save_settings(
    persistent_settings: Option<ResMut<Persistent<RummageSettings>>>,
    volume_settings: Res<VolumeSettings>,
    gameplay_settings: Res<GameplaySettings>,
    graphics_quality: Res<GraphicsQuality>,
) {
    if let Some(mut persistent) = persistent_settings {
        let settings = RummageSettings {
            volume: volume_settings.clone(),
            gameplay: gameplay_settings.clone(),
            graphics: graphics_quality.clone(),
        };
        info!("Saving settings: {:?}", settings);
        if let Err(e) = persistent.set(settings) {
            error!("Failed to save settings: {:?}", e);
        }
    } else {
        warn!("Cannot save settings: persistent storage not available");
    }
}

fn settings_cleanup(
    mut commands: Commands,
    settings_entities: Query<(Entity, &Name), With<MenuItem>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    current_state: Res<State<GameMenuState>>,
) {
    // Cleanup settings menu entities
    let mut removed_count = 0;
    for (entity, name) in settings_entities.iter() {
        let name_str = name.to_string();
        // More precise targeting to avoid removing main menu items
        if name_str.contains("Settings") 
            || name_str.contains("Option") 
            || name_str.contains("Slider") 
            || name_str.contains("Checkbox") 
            || name_str.contains("settings")
            || name_str.contains("Audio Settings")
            || name_str.contains("Video Settings")
            || name_str.contains("Gameplay Settings")
            || name_str.contains("Controls Settings")
            || name_str.contains("Input Blocker")
        {
            info!("Despawning settings entity: '{}'", name_str);
            commands.entity(entity).despawn_recursive();
            removed_count += 1;
        }
    }
    info!("Settings cleanup complete - removed {} entities", removed_count);

    // Additional logging
    if current_state.get() != &GameMenuState::Settings {
        info!("State is not Settings during cleanup, it is: {:?}", current_state.get());
    }
    
    // Return to the origin state after settings
    if let Some(origin) = context.origin_state {
        info!("Returning to origin state from settings: {:?}", origin);
        next_state.set(origin);
        if origin == GameMenuState::MainMenu {
            commands.insert_resource(MainMenuNeedsSetup(true));
            info!("Set MainMenuNeedsSetup to true");
        }
    } else {
        info!("No origin state found in context during settings cleanup");
    }
    
    // Reset the context after use
    context.origin_state = None;
} 