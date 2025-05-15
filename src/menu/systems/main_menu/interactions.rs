use crate::menu::{
    components::MenuButtonAction, save_load::SaveLoadUiContext, save_load::SaveLoadUiState,
    state::GameMenuState,
};
use bevy::prelude::*;

/// Type alias for the main menu button interaction query
type MainMenuInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static MenuButtonAction,
        &'static mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>),
>;

/// Handles button interactions for the main menu
pub fn handle_main_menu_interactions(
    mut interaction_query: MainMenuInteractionQuery,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
    mut save_load_state: ResMut<NextState<SaveLoadUiState>>,
    mut save_load_context: ResMut<SaveLoadUiContext>,
) {
    for (interaction, action, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Button pressed - execute the action
                match action {
                    MenuButtonAction::NewGame => {
                        info!("New Game button pressed");
                        next_state.set(GameMenuState::InGame);
                    }
                    MenuButtonAction::LoadGame => {
                        info!("Load Game button pressed");
                        save_load_context.from_pause_menu = false;
                        save_load_state.set(SaveLoadUiState::LoadGame);
                    }
                    MenuButtonAction::Settings => {
                        info!("Settings button pressed");
                        next_state.set(GameMenuState::Settings);
                    }
                    MenuButtonAction::Multiplayer => {
                        info!("Multiplayer button pressed");
                        // Placeholder for multiplayer functionality
                    }
                    MenuButtonAction::Quit => {
                        info!("Exit button pressed");
                        exit.write(bevy::app::AppExit::default());
                    }
                    MenuButtonAction::Credits => {
                        info!("Credits button pressed");
                        next_state.set(GameMenuState::Credits);
                    }
                    _ => {
                        info!("Button pressed with action: {:?}", action);
                    }
                }
                // Set button color to pressed state
                *background_color = Color::srgb(0.35, 0.35, 0.35).into();
            }
            Interaction::Hovered => {
                // Button is being hovered
                *background_color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                // No interaction
                *background_color = Color::srgba(0.15, 0.15, 0.15, 0.8).into();
            }
        }
    }
}
