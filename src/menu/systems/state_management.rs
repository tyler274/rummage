use crate::menu::{
    components::MenuItem,
    main_menu::setup_main_menu,
    settings::SettingsMenuState,
    state::{GameMenuState, StateTransitionContext},
};
use bevy::prelude::*;

/// Resource to track when we need to set up the main menu
#[derive(Resource, Default)]
pub struct NeedsMainMenuSetup(pub bool);

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
) {
    // If the state changed, log it
    if last_state.is_none() || *last_state.as_ref().unwrap() != *state.get() {
        if let Some(old_state) = last_state.as_ref() {
            info!("State changed from {:?} to {:?}", old_state, state.get());
        } else {
            info!("Initial state: {:?}", state.get());
        }
        *last_state = Some(*state.get());

        // If we're in MainMenu but have no menu items, force setup
        if *state.get() == GameMenuState::MainMenu {
            let count = menu_items.iter().count();
            if count == 0 {
                info!("We're in MainMenu state but have no menu items! Forcing setup...");
                // Recursively despawn any leftover items first
                for entity in menu_items.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                // Then run setup
                setup_main_menu(commands, asset_server, menu_items);
            } else {
                info!("In MainMenu state with {} menu items", count);

                // Force visibility on all menu items even if they exist
                for entity in menu_items.iter() {
                    commands.entity(entity).insert(Visibility::Visible);
                }
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

            // Since we can't directly call setup_main_menu here because of the borrowing issues,
            // we'll set a flag in a resource to trigger the setup in another system
            commands.insert_resource(NeedsMainMenuSetup(true));
        }
    }
}

/// Perform main menu setup if needed based on flag
pub fn perform_main_menu_setup_if_needed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_items: Query<Entity, With<MenuItem>>,
    setup_flag: Option<Res<NeedsMainMenuSetup>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    // Only proceed if the flag resource exists and is set to true
    if let Some(flag) = setup_flag {
        if flag.0 {
            // Remove the flag first
            commands.remove_resource::<NeedsMainMenuSetup>();

            // Set up the main menu
            info!("Setting up main menu from dedicated system");
            setup_main_menu(commands, asset_server, menu_items);

            // Force state refresh to trigger OnEnter systems
            let current_state = GameMenuState::MainMenu;
            next_state.set(current_state);
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
