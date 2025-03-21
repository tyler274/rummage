use crate::menu::{
    components::*, save_load::resources::SaveLoadUiState, state::GameMenuState,
    styles::HOVERED_BUTTON, styles::NORMAL_BUTTON, styles::PRESSED_BUTTON,
};
use bevy::{app::AppExit, prelude::*};

/// System to handle pause menu button interactions
pub fn pause_menu_action(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameMenuState>>,
    mut save_load_state: ResMut<NextState<SaveLoadUiState>>,
    mut save_load_context: ResMut<crate::menu::save_load::resources::SaveLoadUiContext>,
) {
    for (interaction, mut background_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();

                match action {
                    MenuButtonAction::Resume => {
                        // Resume the game
                        info!("Resuming game from pause menu");
                        game_state.set(GameMenuState::InGame);
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
                    MenuButtonAction::MainMenu => {
                        // Go back to the main menu
                        game_state.set(GameMenuState::MainMenu);
                    }
                    MenuButtonAction::Quit => {
                        // Exit the game
                        app_exit_events.send(AppExit::default());
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
