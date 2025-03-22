use crate::menu::camera::setup::MenuCamera;
use crate::menu::state::StateTransitionContext;
use crate::menu::{
    components::{MenuItem, MenuRoot, NeedsMainMenuSetup},
    save_load::SaveExists,
    settings::SettingsMenuState,
    state::GameMenuState,
};
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
    } else if context.from_pause_menu || *current_state.get() == GameMenuState::PausedGame {
        // If the flag is set or we're coming from the pause menu, set the origin to PausedGame
        info!("Detected transition from pause menu");
        context.settings_origin = Some(GameMenuState::PausedGame);
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

/// Monitor state transitions and handle diagnostics
pub fn monitor_state_transitions(
    state: Res<State<GameMenuState>>,
    _next_state: ResMut<NextState<GameMenuState>>,
    mut last_state: Local<Option<GameMenuState>>,
    menu_items: Query<Entity, With<MenuItem>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_roots: Query<Entity, With<MenuRoot>>,
    all_cameras: Query<&Camera>,
    save_exists: Option<ResMut<SaveExists>>,
) {
    // Check if state has changed
    if last_state.is_none() || last_state.unwrap() != *state.get() {
        info!(
            "Game menu state changed from {:?} to {:?}",
            last_state,
            state.get()
        );
        *last_state = Some(*state.get());

        match state.get() {
            GameMenuState::MainMenu => {
                // Set up main menu if necessary (e.g., on first run or after game)
                if menu_items.iter().count() == 0 {
                    info!("No menu items found for main menu, setting up");
                    commands.insert_resource(SaveExists(false));

                    // Ensure menu camera exists before setting up UI
                    if menu_cameras.iter().count() == 0 {
                        info!("No menu camera found for UI, creating one...");
                        commands.spawn((Camera2d::default(), MenuCamera, Name::new("Menu Camera")));
                    }

                    // Get the save exists resource
                    if let Some(save_exists_res) = save_exists {
                        // Set up the main menu directly
                        crate::menu::systems::main_menu::setup::setup_main_menu(
                            commands,
                            asset_server,
                            menu_cameras,
                            existing_roots,
                            all_cameras,
                            save_exists_res,
                        );
                    } else {
                        // Wait for the resource to be available in the next frame
                        info!("SaveExists resource not available, deferring main menu setup");
                        commands.insert_resource(NeedsMainMenuSetup(true));
                    }
                }
            }
            GameMenuState::Settings => {
                // Log when entering settings
                info!("Entering settings state");
            }
            GameMenuState::PausedGame => {
                // Log when entering paused game state
                info!("Entering paused game state");
            }
            _ => {
                // Log when entering other states
                info!("Entering state: {:?}", state.get());
            }
        }
    }
}

/// Periodically check menu items in MainMenu state
pub fn check_menu_items_exist(
    state: Res<State<GameMenuState>>,
    mut commands: Commands,
    menu_items: Query<Entity, With<MenuItem>>,
) {
    // Periodically check if we're in MainMenu state but have no visible menu items
    if *state.get() == GameMenuState::MainMenu {
        let count = menu_items.iter().count();

        if count == 0 {
            info!("No menu items found in MainMenu state! Scheduling setup...");

            // Since we can't directly call setup here because of the borrowing issues,
            // we'll set a flag in a resource to trigger the setup in another system
            commands.insert_resource(NeedsMainMenuSetup(true));
        }
    }
}

/// Perform main menu setup if needed based on flag
pub fn perform_main_menu_setup_if_needed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_roots: Query<Entity, With<MenuRoot>>,
    all_cameras: Query<&Camera>,
    setup_flag: Option<Res<NeedsMainMenuSetup>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
    save_exists: Option<ResMut<SaveExists>>,
) {
    // Even if setup_flag is None, log a message
    if setup_flag.is_none() {
        info!("No NeedsMainMenuSetup flag found, considering setting it up");
    }

    if let Some(flag) = setup_flag {
        if flag.0 {
            info!("Setting up main menu as requested by NeedsMainMenuSetup flag");
            commands.remove_resource::<NeedsMainMenuSetup>();

            if let Some(save_exists_res) = save_exists {
                // Set up the main menu directly
                crate::menu::systems::main_menu::setup::setup_main_menu(
                    commands,
                    asset_server,
                    menu_cameras,
                    existing_roots,
                    all_cameras,
                    save_exists_res,
                );
            } else {
                // Insert the resource and defer setup to next frame
                info!("SaveExists resource not available, creating it and deferring setup");
                commands.insert_resource(SaveExists(false));
                commands.insert_resource(NeedsMainMenuSetup(true));
                return;
            }

            info!("Main menu setup complete, ensuring MainMenu state is set");
            next_state.set(GameMenuState::MainMenu);
        }
    }
}

/// Log when exiting settings
pub fn log_settings_exit(context: Res<StateTransitionContext>) {
    // Log the transition from settings
    info!(
        "Exiting settings, returning to {:?}",
        context.settings_origin
    );
}
