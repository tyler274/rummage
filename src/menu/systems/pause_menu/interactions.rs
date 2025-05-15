use crate::menu::{
    components::{MenuButtonAction, MenuItem},
    save_load::{SaveLoadUiContext, SaveLoadUiState},
    settings::state::SettingsMenuState,
    settings::systems::state_transitions::handle_settings_enter,
    state::AppState,
    state::{GameMenuState, StateTransitionContext},
};
use bevy::{app::AppExit, prelude::*};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

/// Type alias for the query used in `pause_menu_action`.
type PauseMenuButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static MenuButtonAction,
    ),
    (Changed<Interaction>, With<Button>, With<MenuItem>),
>;

/// Handles button actions in the pause menu
pub fn pause_menu_action(
    mut interaction_query: PauseMenuButtonInteractionQuery,
    mut game_menu_state: ResMut<NextState<GameMenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    mut app_exit_events: EventWriter<AppExit>,
    mut save_load_state: ResMut<NextState<SaveLoadUiState>>,
    mut save_load_context: ResMut<SaveLoadUiContext>,
) {
    for (interaction, mut background_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();

                match action {
                    MenuButtonAction::Resume => {
                        // Resume the game
                        info!("Resuming game from pause menu");
                        game_menu_state.set(GameMenuState::InGame);
                        app_state.set(AppState::InGame);
                    }
                    MenuButtonAction::SaveGame => {
                        // Show save game UI
                        info!("Opening save game dialog");
                        save_load_context.from_pause_menu = true;
                        save_load_state.set(SaveLoadUiState::SaveGame);
                    }
                    MenuButtonAction::LoadGame => {
                        // Show load game UI
                        info!("Opening load game dialog");
                        save_load_context.from_pause_menu = true;
                        save_load_state.set(SaveLoadUiState::LoadGame);
                    }
                    MenuButtonAction::Settings => {
                        info!("Opening settings from pause menu");
                        handle_settings_enter(
                            &mut settings_state,
                            &mut game_menu_state,
                            &mut context,
                            GameMenuState::PauseMenu,
                        );
                    }
                    MenuButtonAction::MainMenu => {
                        // Go back to the main menu
                        game_menu_state.set(GameMenuState::MainMenu);
                    }
                    MenuButtonAction::Quit => {
                        // Exit the game
                        app_exit_events.write(AppExit::default());
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}
