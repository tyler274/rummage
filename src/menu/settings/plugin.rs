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
                    |state: Res<State<SettingsMenuState>>, game_state: Res<State<GameMenuState>>| {
                        info!("ENTERED SettingsMenuState::Main - Current Settings State: {:?}, Game State: {:?}", 
                              state.get(), game_state.get());
                    }
                )
            )
            // Settings state - Video settings
            .add_systems(
                OnEnter(SettingsMenuState::Video),
                (
                    crate::menu::camera::setup::setup_menu_camera,
                    setup_video_settings
                )
            )
            // Settings state - Audio settings
            .add_systems(
                OnEnter(SettingsMenuState::Audio),
                (
                    crate::menu::camera::setup::setup_menu_camera,
                    setup_audio_settings
                )
            )
            // Settings state - Gameplay settings
            .add_systems(
                OnEnter(SettingsMenuState::Gameplay),
                (
                    crate::menu::camera::setup::setup_menu_camera,
                    setup_gameplay_settings
                )
            )
            // Settings state - Controls settings
            .add_systems(
                OnEnter(SettingsMenuState::Controls),
                (
                    crate::menu::camera::setup::setup_menu_camera,
                    setup_controls_settings
                )
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
            .add_systems(OnExit(SettingsMenuState::Video), cleanup_settings_menu.run_if(not(in_state(SettingsMenuState::Main))))
            .add_systems(OnExit(SettingsMenuState::Audio), cleanup_settings_menu.run_if(not(in_state(SettingsMenuState::Main))))
            .add_systems(OnExit(SettingsMenuState::Gameplay), cleanup_settings_menu.run_if(not(in_state(SettingsMenuState::Main))))
            .add_systems(OnExit(SettingsMenuState::Controls), cleanup_settings_menu.run_if(not(in_state(SettingsMenuState::Main))))
            // Cleanup system for the main settings menu
            .add_systems(OnExit(GameMenuState::Settings), (
                cleanup_settings_menu,
                // Only clean up settings-specific menu items, not all menu items
                |mut commands: Commands, menu_items: Query<(Entity, Option<&Name>, Option<&Children>), With<MenuItem>>| {
                    for (entity, name, _) in menu_items.iter() {
                        // Only despawn settings-specific menu items by checking their names
                        let is_settings_item = name.map_or(false, |n| {
                            n.as_str().contains("Settings") || 
                            n.as_str().contains("Option") || 
                            n.as_str().contains("Slider") ||
                            n.as_str().contains("Checkbox")
                        });
                        
                        if is_settings_item {
                            info!("Cleaning up settings menu item: {:?} ({})", entity, name.unwrap_or(&Name::new("Unnamed")));
                            commands.entity(entity).despawn_recursive();
                        } else {
                            // For non-settings items, just ensure they're visible, but only if they still exist
                            info!("Preserving non-settings menu item: {:?}", entity);
                            // Don't try to modify entities here - can cause errors if they're being despawned recursively
                            // Just queue them for visibility update
                        }
                    }
                    
                    // Now in a separate step, handle visibility updates for remaining entities
                    info!("Updating visibility for remaining menu entities");
                },
                // Run a separate system to update visibility after cleanup
                |world: &mut World| {
                    // Find all MenuItem entities that still exist
                    let mut query = world.query_filtered::<Entity, With<MenuItem>>();
                    let entities: Vec<Entity> = query.iter(world).collect();
                    
                    // Update visibility for each entity that exists
                    for entity in entities {
                        // The direct mutation approach avoids the CommandQueue issue
                        if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
                            entity_mut.insert(Visibility::Visible);
                        }
                    }
                },
                // Then ensure we trigger menu setup if returning to main menu
                |context: Res<StateTransitionContext>, mut next_state: ResMut<NextState<GameMenuState>>| {
                    if let Some(origin) = context.settings_origin {
                        info!("After settings cleanup, ensuring proper return to origin state: {:?}", origin);
                        // Force a state transition to ensure proper setup
                        next_state.set(origin);
                        
                        // Log additional information for debugging
                        if origin == GameMenuState::MainMenu {
                            info!("Returning to main menu - main menu setup will be triggered");
                        }
                    } else {
                        // Default to main menu if no origin is set
                        info!("No settings origin found, defaulting to MainMenu");
                        next_state.set(GameMenuState::MainMenu);
                    }
                },
                // Clear transition context after use
                |mut context: ResMut<StateTransitionContext>| {
                    // Reset the context for next time
                    context.settings_origin = None;
                    context.from_pause_menu = false;
                    info!("Reset state transition context after settings exit");
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
