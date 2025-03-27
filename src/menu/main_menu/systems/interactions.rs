use crate::menu::{
    components::MenuButtonAction, save_load::SaveLoadUiContext, save_load::SaveLoadUiState,
    settings::state::SettingsMenuState, state::GameMenuState, state::StateTransitionContext,
};
use bevy::prelude::*;

/// Handles button interactions for the main menu
pub fn handle_main_menu_interactions(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut context: ResMut<StateTransitionContext>,
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
                        // Save our origin for when we return
                        context.settings_origin = Some(GameMenuState::MainMenu);
                        // Reset from_pause_menu flag when coming from main menu
                        context.from_pause_menu = false;
                        
                        // Force reset states to ensure proper transitions
                        settings_state.set(SettingsMenuState::Disabled);
                        
                        // First change to settings menu state
                        settings_state.set(SettingsMenuState::Main);
                        info!("Set SettingsMenuState to Main");
                        
                        // Then transition to the settings game state
                        next_state.set(GameMenuState::Settings);
                        info!("Set GameMenuState to Settings");
                        
                        info!(
                            "State transition for settings setup complete: origin=MainMenu, settings_state=Main, game_state=Settings"
                        );
                    }
                    MenuButtonAction::Multiplayer => {
                        info!("Multiplayer button pressed");
                        // Placeholder for multiplayer functionality
                    }
                    MenuButtonAction::Quit => {
                        info!("Exit button pressed");
                        exit.send(bevy::app::AppExit::default());
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
