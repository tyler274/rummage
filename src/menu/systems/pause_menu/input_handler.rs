use crate::menu::{
    settings::SettingsMenuState,
    state::{AppState, GameMenuState, StateTransitionContext},
};
use bevy::prelude::*;

/// Handles keyboard input when in the pause menu, specifically ESC to toggle pause
pub fn esc_key_system(
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    menu_state: Res<State<GameMenuState>>,
    settings_state: Res<State<SettingsMenuState>>,
    _context: Res<StateTransitionContext>,
    mut next_menu_state: ResMut<NextState<GameMenuState>>,
    mut next_settings_state: ResMut<NextState<SettingsMenuState>>,
    mut _commands: Commands,
    mut next_game_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        info!(
            "ESC key pressed - current app state: {:?}, menu state: {:?}, settings state: {:?}",
            app_state.get(),
            menu_state.get(),
            settings_state.get()
        );

        if *app_state.get() == AppState::InGame {
            info!("Opening pause menu from game");
            next_game_state.set(AppState::Paused);
            next_menu_state.set(GameMenuState::PauseMenu);
        } else if *app_state.get() == AppState::Paused {
            match menu_state.get() {
                GameMenuState::PauseMenu => {
                    info!("Returning to game from pause menu");
                    next_game_state.set(AppState::InGame);
                }
                GameMenuState::Settings => {
                    if *settings_state.get() != SettingsMenuState::Main {
                        info!(
                            "Returning to main settings from submenu: {:?}",
                            settings_state.get()
                        );
                        next_settings_state.set(SettingsMenuState::Main);
                    } else {
                        // Return to pause menu from settings
                        info!("Returning to pause menu from settings");
                        next_menu_state.set(GameMenuState::PauseMenu);
                    }
                }
                _ => {
                    // Return to pause menu from any other state
                    info!("Returning to pause menu from state: {:?}", menu_state.get());
                    next_menu_state.set(GameMenuState::PauseMenu);
                }
            }
        } else if *app_state.get() == AppState::Menu {
            if *menu_state.get() != GameMenuState::Settings {
                if *settings_state.get() != SettingsMenuState::Main {
                    info!(
                        "ESC key pressed (Pause Handler) - current app state: {:?}, menu state: {:?}, settings state: {:?}",
                        app_state.get(),
                        menu_state.get(),
                        settings_state.get()
                    );
                    info!(
                        "(Pause Handler) Returning to main settings from submenu while in menu: {:?}",
                        settings_state.get()
                    );
                    next_settings_state.set(SettingsMenuState::Main);
                } else {
                    info!(
                        "ESC pressed in AppState::Menu / GameMenuState::{:?} (Pause Handler), no action needed.",
                        menu_state.get()
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
