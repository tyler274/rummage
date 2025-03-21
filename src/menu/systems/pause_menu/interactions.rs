use crate::game_engine::save::{LoadGameEvent, SaveGameEvent};
use crate::menu::{
    components::*,
    state::{GameMenuState, StateTransitionContext},
    styles::*,
};
use bevy::prelude::*;

/// Handles pause menu button interactions and actions
pub fn pause_menu_action(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    mut exit: EventWriter<bevy::app::AppExit>,
    _save_events: EventWriter<SaveGameEvent>,
    _load_events: EventWriter<LoadGameEvent>,
    mut save_load_state: ResMut<NextState<crate::menu::save_load::SaveLoadUiState>>,
    mut save_load_context: ResMut<crate::menu::save_load::SaveLoadUiContext>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                info!("Pause menu button pressed: {:?}", action);
                *color = PRESSED_BUTTON_COLOR.into();

                match action {
                    MenuButtonAction::Resume => {
                        info!("Resuming game from pause menu");
                        next_state.set(GameMenuState::InGame);
                    }
                    MenuButtonAction::SaveGame => {
                        info!("Opening save game dialog from pause menu");
                        // Set context for save dialog
                        save_load_context.operation_type =
                            crate::menu::save_load::SaveLoadOperation::Save;
                        save_load_context.return_to_pause_menu = true;
                        // Transition to save/load UI
                        save_load_state.set(crate::menu::save_load::SaveLoadUiState::Active);
                    }
                    MenuButtonAction::LoadGame => {
                        info!("Opening load game dialog from pause menu");
                        // Set context for load dialog
                        save_load_context.operation_type =
                            crate::menu::save_load::SaveLoadOperation::Load;
                        save_load_context.return_to_pause_menu = true;
                        // Transition to save/load UI
                        save_load_state.set(crate::menu::save_load::SaveLoadUiState::Active);
                    }
                    MenuButtonAction::Settings => {
                        info!("Opening settings from pause menu");
                        next_state.set(GameMenuState::Settings);
                    }
                    MenuButtonAction::MainMenu => {
                        info!("Returning to main menu from pause menu");
                        next_state.set(GameMenuState::MainMenu);
                    }
                    MenuButtonAction::QuitGame => {
                        info!("Quitting game from pause menu");
                        exit.send(bevy::app::AppExit);
                    }
                    _ => {
                        warn!("Unhandled menu button action in pause menu: {:?}", action);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
