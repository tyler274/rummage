use crate::menu::{
    settings::SettingsMenuState,
    state::{AppState, GameMenuState, StateTransitionContext},
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

// Define the system parameter struct for EscKeyState
#[derive(SystemParam)]
pub struct EscKeyStateParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    app_state: Res<'w, State<AppState>>,
    menu_state: Res<'w, State<GameMenuState>>,
    settings_state: Res<'w, State<SettingsMenuState>>,
    #[system_param(ignore)]
    _context: Res<'w, StateTransitionContext>, // Ignoring unused param
    next_menu_state: ResMut<'w, NextState<GameMenuState>>,
    next_settings_state: ResMut<'w, NextState<SettingsMenuState>>,
    #[system_param(ignore)]
    _commands: Commands<'w, 's>, // Ignoring unused param
    next_game_state: ResMut<'w, NextState<AppState>>,
}

/// Handles keyboard input when in the pause menu, specifically ESC to toggle pause
pub fn esc_key_system(mut params: EscKeyStateParams) {
    if params.keys.just_pressed(KeyCode::Escape) {
        info!(
            "ESC key pressed - current app state: {:?}, menu state: {:?}, settings state: {:?}",
            params.app_state.get(),
            params.menu_state.get(),
            params.settings_state.get()
        );

        if *params.app_state.get() == AppState::InGame {
            info!("Opening pause menu from game");
            params.next_game_state.set(AppState::Paused);
            params.next_menu_state.set(GameMenuState::PauseMenu);
        } else if *params.app_state.get() == AppState::Paused {
            match params.menu_state.get() {
                GameMenuState::PauseMenu => {
                    info!("Returning to game from pause menu");
                    params.next_game_state.set(AppState::InGame);
                }
                GameMenuState::Settings => {
                    if *params.settings_state.get() != SettingsMenuState::Main {
                        info!(
                            "Returning to main settings from submenu: {:?}",
                            params.settings_state.get()
                        );
                        params.next_settings_state.set(SettingsMenuState::Main);
                    } else {
                        // Return to pause menu from settings
                        info!("Returning to pause menu from settings");
                        params.next_menu_state.set(GameMenuState::PauseMenu);
                    }
                }
                _ => {
                    // Return to pause menu from any other state
                    info!(
                        "Returning to pause menu from state: {:?}",
                        params.menu_state.get()
                    );
                    params.next_menu_state.set(GameMenuState::PauseMenu);
                }
            }
        } else if *params.app_state.get() == AppState::Menu {
            if *params.menu_state.get() != GameMenuState::Settings {
                if *params.settings_state.get() != SettingsMenuState::Main {
                    info!(
                        "ESC key pressed (Pause Handler) - current app state: {:?}, menu state: {:?}, settings state: {:?}",
                        params.app_state.get(),
                        params.menu_state.get(),
                        params.settings_state.get()
                    );
                    info!(
                        "(Pause Handler) Returning to main settings from submenu while in menu: {:?}",
                        params.settings_state.get()
                    );
                    params.next_settings_state.set(SettingsMenuState::Main);
                } else {
                    info!(
                        "ESC pressed in AppState::Menu / GameMenuState::{:?} (Pause Handler), no action needed.",
                        params.menu_state.get()
                    );
                }
            } else {
                info!(
                    "ESC pressed in AppState::Menu / GameMenuState::Settings (Pause Handler), deferring to handle_settings_back_input.",
                );
            }
        }
    }
}

pub fn handle_pause_menu_input(
    _keyboard_input: Res<ButtonInput<KeyCode>>,
    _app_exit_events: EventWriter<AppExit>,
    _game_state: ResMut<NextState<GameMenuState>>,
    _context: Res<StateTransitionContext>,
    _commands: Commands,
) {
}
