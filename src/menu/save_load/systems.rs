use crate::camera::components::AppLayer;
use crate::game_engine::save::events::{LoadGameEvent, SaveGameEvent};
use crate::game_engine::save::resources::SaveMetadata;
use crate::menu::input_blocker::InputBlocker;
use crate::menu::state::GameMenuState;
use crate::menu::styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use bevy::prelude::*;

use super::components::*;
use super::resources::*;

/// Sets up the save game dialog
pub fn setup_save_dialog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    save_metadata: Option<Res<SaveMetadata>>,
    context: ResMut<SaveLoadUiContext>,
) {
    info!("Setting up save game dialog");

    // First, create a full-screen transparent input blocker
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        AppLayer::Menu.layer(),
        InputBlocker,
        SaveLoadUi,
        Name::new("Save Dialog Input Blocker"),
    ));

    // Create a semi-transparent background overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            SaveLoadUi,
            SaveGamePanel,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Dialog panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(500.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderRadius::all(Val::Px(5.0)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Dialog title
                    parent.spawn((
                        Text::new("Save Game"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect {
                                bottom: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        },
                        AppLayer::Menu.layer(),
                    ));

                    // Save slots container
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(250.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                overflow: Overflow::visible(),
                                margin: UiRect {
                                    bottom: Val::Px(20.0),
                                    ..default()
                                },
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            BorderRadius::all(Val::Px(3.0)),
                            AppLayer::Menu.layer(),
                        ))
                        .with_children(|parent| {
                            // If we have save metadata, display existing save slots
                            if let Some(metadata) = save_metadata.as_ref() {
                                // Iterate over saves
                                for save_info in &metadata.saves {
                                    spawn_save_slot_button(
                                        parent,
                                        &save_info.slot_name,
                                        &Some(save_info.description.clone()),
                                        &asset_server,
                                    );
                                }
                            }

                            // Always add a "New Save" button
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::bottom(Val::Px(5.0)),
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    Button,
                                    SaveLoadButtonAction::CreateSaveSlot,
                                    AppLayer::Menu.layer(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Create New Save"),
                                        TextFont {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        AppLayer::Menu.layer(),
                                    ));
                                });
                        });

                    // Bottom buttons row
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            AppLayer::Menu.layer(),
                        ))
                        .with_children(|parent| {
                            // Cancel button
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    BorderRadius::all(Val::Px(3.0)),
                                    Button,
                                    SaveLoadButtonAction::Cancel,
                                    AppLayer::Menu.layer(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Cancel"),
                                        TextFont {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        AppLayer::Menu.layer(),
                                    ));
                                });

                            // If there's a selected slot, add a save button
                            if let Some(slot) = &context.selected_slot {
                                parent
                                    .spawn((
                                        Node {
                                            width: Val::Px(150.0),
                                            height: Val::Px(40.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(NORMAL_BUTTON),
                                        BorderRadius::all(Val::Px(3.0)),
                                        Button,
                                        SaveLoadButtonAction::SaveToSlot(slot.clone()),
                                        AppLayer::Menu.layer(),
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new("Save"),
                                            TextFont {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                            AppLayer::Menu.layer(),
                                        ));
                                    });
                            }
                        });
                });
        });
}

/// Sets up the load game dialog
pub fn setup_load_dialog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    save_metadata: Option<Res<SaveMetadata>>,
) {
    info!("Setting up load game dialog");

    // First, create a full-screen transparent input blocker
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        AppLayer::Menu.layer(),
        InputBlocker,
        SaveLoadUi,
        Name::new("Load Dialog Input Blocker"),
    ));

    // Create a semi-transparent background overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            SaveLoadUi,
            LoadGamePanel,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Dialog panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(500.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderRadius::all(Val::Px(5.0)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Dialog title
                    parent.spawn((
                        Text::new("Load Game"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect {
                                bottom: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        },
                        AppLayer::Menu.layer(),
                    ));

                    // Save slots container
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(250.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                overflow: Overflow::visible(),
                                margin: UiRect {
                                    bottom: Val::Px(20.0),
                                    ..default()
                                },
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            BorderRadius::all(Val::Px(3.0)),
                            AppLayer::Menu.layer(),
                        ))
                        .with_children(|parent| {
                            // If we have save metadata, display existing save slots
                            if let Some(metadata) = save_metadata.as_ref() {
                                if metadata.saves.is_empty() {
                                    // Display a message if no saves are available
                                    parent.spawn((
                                        Text::new("No saved games found"),
                                        TextFont {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        Node {
                                            margin: UiRect::top(Val::Px(20.0)),
                                            ..default()
                                        },
                                        AppLayer::Menu.layer(),
                                    ));
                                } else {
                                    // Iterate over saves
                                    for save_info in &metadata.saves {
                                        spawn_load_slot_button(
                                            parent,
                                            &save_info.slot_name,
                                            &Some(save_info.description.clone()),
                                            &format!("Turn {}", save_info.turn_number),
                                            &asset_server,
                                        );
                                    }
                                }
                            } else {
                                // Display a message if metadata is not available
                                parent.spawn((
                                    Text::new("Save metadata not available"),
                                    TextFont {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                    Node {
                                        margin: UiRect::top(Val::Px(20.0)),
                                        ..default()
                                    },
                                    AppLayer::Menu.layer(),
                                ));
                            }
                        });

                    // Bottom buttons row - just Cancel for load dialog
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            AppLayer::Menu.layer(),
                        ))
                        .with_children(|parent| {
                            // Cancel button
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    BorderRadius::all(Val::Px(3.0)),
                                    Button,
                                    SaveLoadButtonAction::Cancel,
                                    AppLayer::Menu.layer(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Cancel"),
                                        TextFont {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        AppLayer::Menu.layer(),
                                    ));
                                });
                        });
                });
        });
}

/// Spawns a save slot button for the save dialog
fn spawn_save_slot_button(
    parent: &mut ChildBuilder,
    slot_name: &str,
    description: &Option<String>,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            Button,
            SaveLoadButtonAction::SaveToSlot(slot_name.to_string()),
            SaveSlotButton,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Slot name
            parent.spawn((
                Text::new(slot_name),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                AppLayer::Menu.layer(),
            ));

            // Description (if available)
            if let Some(desc) = description {
                parent.spawn((
                    Text::new(desc),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                    AppLayer::Menu.layer(),
                ));
            }
        });
}

/// Spawns a load slot button for the load dialog
fn spawn_load_slot_button(
    parent: &mut ChildBuilder,
    slot_name: &str,
    description: &Option<String>,
    turn_info: &str,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            Button,
            SaveLoadButtonAction::LoadFromSlot(slot_name.to_string()),
            SaveSlotButton,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Top row with slot name and turn info
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Slot name
                    parent.spawn((
                        Text::new(slot_name),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        AppLayer::Menu.layer(),
                    ));

                    // Turn info
                    parent.spawn((
                        Text::new(turn_info),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                        AppLayer::Menu.layer(),
                    ));
                });

            // Description (if available)
            if let Some(desc) = description {
                parent.spawn((
                    Text::new(desc),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                    Node {
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    AppLayer::Menu.layer(),
                ));
            }
        });
}

/// Cleanup system for save/load UI
pub fn cleanup_save_load_ui(mut commands: Commands, query: Query<Entity, With<SaveLoadUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// System to handle save/load UI button interactions
pub fn handle_save_load_buttons(
    mut interaction_query: Query<
        (&Interaction, &SaveLoadButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut save_load_state: ResMut<NextState<SaveLoadUiState>>,
    mut game_state: ResMut<NextState<GameMenuState>>,
    mut save_events: EventWriter<SaveGameEvent>,
    mut load_events: EventWriter<LoadGameEvent>,
    mut context: ResMut<SaveLoadUiContext>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                match action {
                    SaveLoadButtonAction::SaveToSlot(slot_name) => {
                        info!("Saving game to slot: {}", slot_name);
                        save_events.send(SaveGameEvent {
                            slot_name: slot_name.clone(),
                            description: Some(format!("Manual save to {}", slot_name)),
                            with_snapshot: true,
                        });

                        // Update the context and go back to the appropriate menu
                        context.last_save_slot = Some(slot_name.clone());
                        save_load_state.set(SaveLoadUiState::Hidden);

                        if context.from_pause_menu {
                            game_state.set(GameMenuState::PausedGame);
                        } else {
                            game_state.set(GameMenuState::MainMenu);
                        }
                    }
                    SaveLoadButtonAction::LoadFromSlot(slot_name) => {
                        info!("Loading game from slot: {}", slot_name);
                        load_events.send(LoadGameEvent {
                            slot_name: slot_name.clone(),
                        });

                        // Update the context and go back to the game
                        context.last_save_slot = Some(slot_name.clone());
                        save_load_state.set(SaveLoadUiState::Hidden);
                        game_state.set(GameMenuState::InGame);
                    }
                    SaveLoadButtonAction::CreateSaveSlot => {
                        info!("Creating new save slot");
                        // For now, just use a timestamp as the name
                        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
                        let slot_name = format!("save_{}", timestamp);

                        save_events.send(SaveGameEvent {
                            slot_name: slot_name.clone(),
                            description: Some("New save".to_string()),
                            with_snapshot: true,
                        });

                        // Update the context and go back to the appropriate menu
                        context.last_save_slot = Some(slot_name);
                        save_load_state.set(SaveLoadUiState::Hidden);

                        if context.from_pause_menu {
                            game_state.set(GameMenuState::PausedGame);
                        } else {
                            game_state.set(GameMenuState::MainMenu);
                        }
                    }
                    SaveLoadButtonAction::Cancel => {
                        info!("Cancelling save/load dialog");
                        // Go back to the previous menu
                        save_load_state.set(SaveLoadUiState::Hidden);

                        if context.from_pause_menu {
                            game_state.set(GameMenuState::PausedGame);
                        } else {
                            game_state.set(GameMenuState::MainMenu);
                        }
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
