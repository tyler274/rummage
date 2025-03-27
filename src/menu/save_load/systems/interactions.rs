use crate::game_engine::save::events::{LoadGameEvent, SaveGameEvent};
use crate::menu::save_load::components::SaveLoadButtonAction;
use crate::menu::save_load::resources::{SaveLoadUiContext, SaveLoadUiState};
use crate::menu::state::GameMenuState;
use crate::menu::styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use bevy::prelude::*;

/// Handles button interactions in the save/load UI
pub fn handle_save_load_buttons(
    mut interaction_query: Query<
        (&Interaction, &SaveLoadButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut save_load_state: ResMut<NextState<SaveLoadUiState>>,
    mut game_state: ResMut<NextState<GameMenuState>>,
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

                        // Set game state to in-game (loading will happen in game systems)
                        game_state.set(GameMenuState::InGame);
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
