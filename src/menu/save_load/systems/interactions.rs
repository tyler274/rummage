use crate::game_engine::save::events::{LoadGameEvent, SaveGameEvent};
use crate::menu::save_load::components::SaveLoadButtonAction;
use crate::menu::save_load::resources::{SaveLoadUiContext, SaveLoadUiState};
use crate::menu::state::{AppState, GameMenuState};
use crate::menu::styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use bevy::prelude::*;

/// Type alias for the query used in `handle_save_load_buttons`.
type SaveLoadButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static SaveLoadButtonAction,
        &'static mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>),
>;

/// Handles button interactions in the save/load UI
pub fn handle_save_load_buttons(
    mut interaction_query: SaveLoadButtonInteractionQuery,
    mut save_load_state: ResMut<NextState<SaveLoadUiState>>,
    mut game_state: ResMut<NextState<GameMenuState>>,
    mut _app_state: ResMut<NextState<AppState>>,
    mut save_events: EventWriter<SaveGameEvent>,
    mut load_events: EventWriter<LoadGameEvent>,
    context: ResMut<SaveLoadUiContext>,
) {
    // Process button interactions
    for (interaction, action, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Button pressed
                *bg_color = PRESSED_BUTTON.into();

                match action {
                    SaveLoadButtonAction::SaveToSlot(slot_name) => {
                        info!("Save game requested for slot: {}", slot_name);

                        // Send save game event with the slot name
                        save_events.send(SaveGameEvent {
                            slot_name: slot_name.clone(),
                            description: Some(format!(
                                "Save from {}",
                                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
                            )),
                            with_snapshot: false,
                        });

                        // Clear the UI
                        save_load_state.set(SaveLoadUiState::Hidden);

                        // Return to appropriate state
                        if context.from_pause_menu {
                            game_state.set(GameMenuState::PauseMenu);
                        } else {
                            game_state.set(GameMenuState::MainMenu);
                        }
                    }
                    SaveLoadButtonAction::LoadFromSlot(slot_name) => {
                        info!("Load game requested for slot: {}", slot_name);

                        // Send load game event with the slot name
                        load_events.send(LoadGameEvent {
                            slot_name: slot_name.clone(),
                        });

                        // Clear the UI
                        save_load_state.set(SaveLoadUiState::Hidden);

                        // Set game state to Loading (will transition to InGame after load)
                        game_state.set(GameMenuState::Loading);
                        // Set AppState to InGame immediately? Or wait?
                        // Let's try setting AppState later, after loading is confirmed.
                    }
                    SaveLoadButtonAction::Cancel => {
                        info!("Cancelling save/load operation");

                        // Clear the UI
                        save_load_state.set(SaveLoadUiState::Hidden);

                        // Return to appropriate state
                        if context.from_pause_menu {
                            game_state.set(GameMenuState::PauseMenu);
                        } else {
                            game_state.set(GameMenuState::MainMenu);
                        }
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                // Button hovered
                *bg_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // No interaction
                *bg_color = NORMAL_BUTTON.into();
            }
        }
    }
}
