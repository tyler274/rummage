use crate::menu::camera::setup::MenuCamera;
use crate::menu::state::StateTransitionContext;
use crate::menu::{
    components::{MenuItem, MenuRoot},
    save_load::SaveExists,
    settings::SettingsMenuState,
    state::GameMenuState,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

/// Set up the transition context for settings menu
pub fn setup_settings_transition(
    mut context: ResMut<StateTransitionContext>,
    current_state: Res<State<GameMenuState>>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
) {
    info!(
        "Setting up settings transition from state: {:?}, from_pause_menu: {}",
        current_state.get(),
        context.from_pause_menu
    );

    // Always reset from_pause_menu flag when transitioning from MainMenu
    if *current_state.get() == GameMenuState::MainMenu {
        info!("Resetting from_pause_menu flag because we're in MainMenu state");
        context.from_pause_menu = false;
        // Explicitly set the settings origin to MainMenu
        info!("Explicitly setting settings_origin to MainMenu");
        context.settings_origin = Some(GameMenuState::MainMenu);
    } else if context.from_pause_menu || *current_state.get() == GameMenuState::PauseMenu {
        // If the flag is set or we're coming from the pause menu, set the origin to PauseMenu
        info!("Detected transition from pause menu");
        context.settings_origin = Some(GameMenuState::PauseMenu);
    } else {
        // Fall back to checking the current state
        match current_state.get() {
            GameMenuState::Settings if context.settings_origin.is_none() => {
                // If we're already in Settings state but have no origin,
                // default to main menu
                info!("Already in Settings state with no origin, defaulting to main menu");
                context.settings_origin = Some(GameMenuState::MainMenu);
            }
            _ => {
                if context.settings_origin.is_none() {
                    // Default to main menu if coming from an unexpected state
                    info!("Entering settings from unexpected state, defaulting to main menu");
                    context.settings_origin = Some(GameMenuState::MainMenu);
                } else {
                    info!(
                        "Using existing settings origin: {:?}",
                        context.settings_origin
                    );
                }
            }
        }
    }

    // Ensure we're showing the main settings screen when entering settings
    info!("Setting SettingsMenuState to Main");
    settings_state.set(SettingsMenuState::Main);
}

// SystemParam struct for monitor_state_transitions
#[derive(SystemParam)]
pub struct StateMonitorParams<'w, 's> {
    state: Res<'w, State<GameMenuState>>,
    #[system_param(ignore)]
    _next_state: ResMut<'w, NextState<GameMenuState>>, // Ignoring unused param
    menu_items: Query<'w, 's, Entity, With<MenuItem>>,
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    menu_cameras: Query<'w, 's, Entity, With<MenuCamera>>,
    existing_roots: Query<'w, 's, Entity, With<MenuRoot>>,
    all_cameras: Query<'w, 's, &'static Camera>,
    save_exists: Option<ResMut<'w, SaveExists>>,
}

/// Monitor state transitions and handle diagnostics
pub fn monitor_state_transitions(
    mut params: StateMonitorParams,
    mut last_state: Local<Option<GameMenuState>>,
) {
    // Check if state has changed
    if last_state.is_none() || last_state.unwrap() != *params.state.get() {
        info!(
            "Game menu state changed from {:?} to {:?}",
            last_state,
            params.state.get()
        );
        *last_state = Some(*params.state.get());

        match params.state.get() {
            GameMenuState::MainMenu => {
                // Set up main menu if necessary (e.g., on first run or after game)
                if params.menu_items.iter().count() == 0 {
                    info!("No menu items found for main menu, setting up");
                    params.commands.insert_resource(SaveExists(false));

                    // Ensure menu camera exists before setting up UI
                    if params.menu_cameras.iter().count() == 0 {
                        info!("No menu camera found for UI, creating one with proper order...");

                        // Find highest current camera order
                        let mut highest_order = 0;
                        for camera in params.all_cameras.iter() {
                            if camera.order > highest_order {
                                highest_order = camera.order;
                            }
                        }

                        // Create camera with next order and proper UI components
                        params.commands.spawn((
                            Camera2d,
                            Camera {
                                order: highest_order + 1,
                                ..default()
                            },
                            MenuCamera,
                            Name::new("Menu Camera"),
                            // Add essential UI components to make it a valid UI parent
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ViewVisibility::default(),
                            InheritedVisibility::default(),
                            Visibility::Visible,
                            ZIndex::default(),
                        ));

                        info!(
                            "Created emergency menu camera with order {}",
                            highest_order + 1
                        );
                    }

                    // Get the save exists resource
                    if let Some(save_exists_res) = params.save_exists {
                        // Set up the main menu directly
                        crate::menu::systems::main_menu::setup::setup_main_menu(
                            params.commands,
                            params.asset_server,
                            params.menu_cameras,
                            params.existing_roots,
                            params.all_cameras,
                            save_exists_res,
                        );
                    } else {
                        // Wait for the resource to be available in the next frame
                        info!("SaveExists resource not available, deferring main menu setup");
                    }
                }
            }
            GameMenuState::Settings => {
                // Log when entering settings
                info!("Entering settings state");
            }
            GameMenuState::PauseMenu => {
                // Log when entering paused game state
                info!("Entering pause menu state");
            }
            _ => {
                // Log when entering other states
                info!("Entering state: {:?}", params.state.get());
            }
        }
    }
}

/// Periodically check menu items in MainMenu state
/// Log when exiting settings
pub fn log_settings_exit(context: Res<StateTransitionContext>) {
    // Log the transition from settings
    info!(
        "Exiting settings, returning to {:?}",
        context.settings_origin
    );
}

/// Handles transition from MainMenu to Settings
pub fn handle_main_to_settings(
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut _context: ResMut<StateTransitionContext>,
) {
    info!("Transitioning from MainMenu to Settings");
    next_state.set(GameMenuState::Settings);
}

/// Handles transition from PauseMenu to Settings
pub fn handle_pause_to_settings(
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut _context: ResMut<StateTransitionContext>,
) {
    info!("Transitioning from PauseMenu to Settings");
    next_state.set(GameMenuState::Settings);
}

/// Handles transition from any menu state back to MainMenu
pub fn handle_back_to_main(
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut _context: ResMut<StateTransitionContext>,
) {
    info!("Transitioning back to MainMenu");
    next_state.set(GameMenuState::MainMenu);
}

/// Handles transition from Credits back to MainMenu
pub fn handle_credits_to_main(
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut _context: ResMut<StateTransitionContext>,
) {
    info!("Transitioning from Credits to MainMenu");
    next_state.set(GameMenuState::MainMenu);
}
