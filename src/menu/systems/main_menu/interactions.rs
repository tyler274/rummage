use bevy::prelude::*;

use crate::menu::state::MenuState;
use crate::menu::systems::main_menu::states::MultiplayerState;

/// Handle button clicks in the main menu
pub fn handle_main_menu_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Name, &Parent),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Parent, With<Text>>,
    mut next_state: ResMut<NextState<MenuState>>,
    mut multi_state: ResMut<NextState<MultiplayerState>>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, mut color, name, parent) in interaction_query.iter_mut() {
        let button_name = name.as_str();

        match *interaction {
            Interaction::Pressed => {
                info!("Button pressed: {}", button_name);

                // Handle different buttons based on their names
                match button_name {
                    "New Game Button" => {
                        info!("New Game selected");
                        next_state.set(MenuState::NewGame);
                    }
                    "Load Game Button" => {
                        info!("Load Game selected");
                        next_state.set(MenuState::LoadGame);
                    }
                    "Multiplayer Button" => {
                        info!("Multiplayer selected");
                        multi_state.set(MultiplayerState::Menu);
                    }
                    "Settings Button" => {
                        info!("Settings selected");
                        next_state.set(MenuState::Settings);
                    }
                    "Credits Button" => {
                        info!("Credits selected");
                        next_state.set(MenuState::Credits);
                    }
                    "Exit Button" => {
                        info!("Exit selected");
                        app_exit_events.send(bevy::app::AppExit::default());
                    }
                    _ => {
                        // Check for text elements with parent buttons
                        for text_parent in text_query.iter() {
                            if text_parent.get() == parent.get() {
                                info!("Button with text pressed: {}", button_name);
                                // Handle based on the parent button's name
                                if button_name.contains("New Game") {
                                    next_state.set(MenuState::NewGame);
                                } else if button_name.contains("Load Game") {
                                    next_state.set(MenuState::LoadGame);
                                } else if button_name.contains("Multiplayer") {
                                    multi_state.set(MultiplayerState::Menu);
                                } else if button_name.contains("Settings") {
                                    next_state.set(MenuState::Settings);
                                } else if button_name.contains("Credits") {
                                    next_state.set(MenuState::Credits);
                                } else if button_name.contains("Exit") {
                                    app_exit_events.send(bevy::app::AppExit::default());
                                }
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                // Highlight button on hover
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                // Reset color when not interacting
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
