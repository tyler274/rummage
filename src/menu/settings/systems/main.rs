use crate::menu::components::MenuItem;
use crate::menu::settings::components::*;
use crate::menu::settings::state::SettingsMenuState;
use crate::menu::state::{GameMenuState, StateTransitionContext};
use crate::menu::styles::*;
use bevy::prelude::*;

use super::common::*;

/// Sets up the main settings menu
pub fn setup_main_settings(mut commands: Commands, context: Res<StateTransitionContext>) {
    info!(
        "Setting up main settings menu - START (from origin: {:?})",
        context.settings_origin
    );

    // Create root node
    let root_entity =
        spawn_settings_root(&mut commands, Color::srgba(0.1, 0.1, 0.1, 0.95), "Settings");

    commands.entity(root_entity).insert(MainSettingsScreen);

    commands.entity(root_entity).with_children(|parent| {
        // Title
        spawn_settings_title(parent, "SETTINGS");

        // Settings container
        let container = spawn_settings_container(parent);

        commands.entity(container).with_children(|parent| {
            spawn_settings_button(parent, "Video", SettingsButtonAction::VideoSettings);
            spawn_settings_button(parent, "Audio", SettingsButtonAction::AudioSettings);
            spawn_settings_button(parent, "Gameplay", SettingsButtonAction::GameplaySettings);
            spawn_settings_button(parent, "Controls", SettingsButtonAction::ControlsSettings);
            spawn_settings_button(parent, "Back", SettingsButtonAction::Back);
        });
    });

    info!(
        "Setting up main settings menu - COMPLETE. Root entity: {:?}",
        root_entity
    );
}

/// Handles button interactions in the settings menu
pub fn settings_button_action(
    mut interaction_query: Query<
        (&Interaction, &SettingsButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<SettingsMenuState>>,
    mut game_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    current_settings_state: Res<State<SettingsMenuState>>,
    current_game_state: Res<State<GameMenuState>>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                info!(
                    "Button pressed - Current Game State: {:?}, Current Settings State: {:?}",
                    current_game_state.get(),
                    current_settings_state.get()
                );

                match action {
                    SettingsButtonAction::VideoSettings => {
                        info!("Transitioning to Video settings submenu");
                        next_state.set(SettingsMenuState::Video);
                    }
                    SettingsButtonAction::AudioSettings => {
                        info!("Transitioning to Audio settings submenu");
                        next_state.set(SettingsMenuState::Audio);
                    }
                    SettingsButtonAction::GameplaySettings => {
                        info!("Transitioning to Gameplay settings submenu");
                        next_state.set(SettingsMenuState::Gameplay);
                    }
                    SettingsButtonAction::ControlsSettings => {
                        info!("Transitioning to Controls settings submenu");
                        next_state.set(SettingsMenuState::Controls);
                    }
                    SettingsButtonAction::Back => {
                        // Return to the previous menu (main menu or pause menu)
                        if let Some(origin) = context.settings_origin {
                            info!("Returning to origin state: {:?}", origin);
                            // First set the settings menu state to disabled to trigger cleanup
                            next_state.set(SettingsMenuState::Disabled);
                            // Then set the game menu state to the origin state
                            game_state.set(origin);
                            context.settings_origin = None;
                        } else {
                            // Default to main menu if origin is not set
                            info!("No origin state found, defaulting to MainMenu");
                            next_state.set(SettingsMenuState::Disabled);
                            game_state.set(GameMenuState::MainMenu);
                        }
                    }
                    SettingsButtonAction::BackToMainSettings => {
                        info!("Returning to main settings");
                        next_state.set(SettingsMenuState::Main);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(HOVERED_BUTTON);
            }
            Interaction::None => {
                *color = BackgroundColor(NORMAL_BUTTON);
            }
        }
    }
}
