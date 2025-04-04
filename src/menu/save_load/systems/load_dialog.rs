use crate::camera::components::AppLayer;
use crate::game_engine::save::resources::SaveMetadata;
use crate::menu::input_blocker::InputBlocker;
use crate::menu::save_load::components::*;
use bevy::prelude::*;
use bevy::text::JustifyText;

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
            // Main dialog panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(500.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 1.0)),
                    SaveLoadUi,
                ))
                .with_children(|parent| {
                    // Dialog title
                    parent.spawn((
                        Text::new("Load Game"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        SaveLoadUi,
                    ));

                    // Load slots container
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(250.0),
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::vertical(Val::Px(20.0)),
                                ..default()
                            },
                            SaveLoadUi,
                        ))
                        .with_children(|parent| {
                            // Get save metadata if available
                            if let Some(metadata) = save_metadata {
                                let saves = metadata.saves.clone();
                                if saves.is_empty() {
                                    // No saves found, show message
                                    parent.spawn((
                                        Text::new("No saved games found"),
                                        TextFont {
                                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        SaveLoadUi,
                                    ));
                                } else {
                                    // Create load slots for each save
                                    for save_info in saves.iter() {
                                        let turn_info = format!("Turn {}", save_info.turn_number);

                                        // Get save description
                                        let description = Some(save_info.description.clone());

                                        spawn_load_slot_button(
                                            parent,
                                            &save_info.slot_name,
                                            &description,
                                            &turn_info,
                                            &asset_server,
                                        );
                                    }
                                }
                            } else {
                                // No metadata found
                                parent.spawn((
                                    Text::new("No save metadata found"),
                                    TextFont {
                                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                    TextLayout::new_with_justify(JustifyText::Center),
                                    SaveLoadUi,
                                ));
                            }
                        });

                    // Button row
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::SpaceEvenly,
                                ..default()
                            },
                            SaveLoadUi,
                        ))
                        .with_children(|parent| {
                            // Cancel button
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(120.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                                    SaveLoadButtonAction::Cancel,
                                    SaveLoadUi,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Cancel"),
                                        TextFont {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        SaveLoadUi,
                                    ));
                                });
                        });
                });
        });
}

/// Spawns a load slot button in the load dialog
fn spawn_load_slot_button(
    parent: &mut ChildBuilder,
    slot_name: &str,
    description: &Option<String>,
    turn_info: &str,
    asset_server: &AssetServer,
) {
    // Load slot button
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            SaveLoadButtonAction::LoadFromSlot(slot_name.to_string()),
            SaveLoadUi,
        ))
        .with_children(|parent| {
            // Slot name
            parent.spawn((
                Text::new(slot_name.to_string()),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Left),
                SaveLoadUi,
            ));

            // Description
            if let Some(desc) = description {
                parent.spawn((
                    Text::new(desc.clone()),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                    TextLayout::new_with_justify(JustifyText::Left),
                    SaveLoadUi,
                ));
            }

            // Turn info
            parent.spawn((
                Text::new(turn_info.to_string()),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgba(0.6, 0.8, 0.6, 1.0)),
                TextLayout::new_with_justify(JustifyText::Left),
                SaveLoadUi,
            ));
        });
}
