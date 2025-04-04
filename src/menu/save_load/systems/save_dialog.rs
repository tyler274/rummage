use crate::camera::components::AppLayer;
use crate::game_engine::save::resources::SaveMetadata;
use crate::menu::input_blocker::InputBlocker;
use crate::menu::save_load::components::*;
use crate::menu::save_load::resources::*;
use bevy::prelude::*;

/// Sets up the save game dialog
pub fn setup_save_dialog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    save_metadata: Option<Res<SaveMetadata>>,
    _context: ResMut<SaveLoadUiContext>,
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
                        Text::new("Save Game"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        SaveLoadUi,
                    ));

                    // Save slots container
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
                            let saves = if let Some(metadata) = save_metadata {
                                metadata.saves.clone()
                            } else {
                                info!("No save metadata found, using empty slots");
                                Vec::new()
                            };

                            // Create save slots (always create at least 3 slots)
                            for i in 1..=3 {
                                let slot_name = format!("Slot {}", i);

                                // Try to find if there's already a save in this slot
                                let description = saves
                                    .iter()
                                    .find(|save| save.slot_name == slot_name)
                                    .map(|save| save.description.clone());

                                spawn_save_slot_button(
                                    parent,
                                    &slot_name,
                                    &description,
                                    &asset_server,
                                );
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

/// Spawns a save slot button in the save dialog
fn spawn_save_slot_button(
    parent: &mut ChildBuilder,
    slot_name: &str,
    description: &Option<String>,
    asset_server: &AssetServer,
) {
    // Save slot button
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            SaveLoadButtonAction::SaveToSlot(slot_name.to_string()),
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

            // Slot description or "Empty" if no save
            let desc_text = if let Some(desc) = description {
                desc.clone()
            } else {
                "Empty".to_string()
            };

            parent.spawn((
                Text::new(desc_text),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                TextLayout::new_with_justify(JustifyText::Left),
                SaveLoadUi,
            ));
        });
}

pub fn show_save_dialog(
    _commands: Commands,
    _asset_server: ResMut<AssetServer>,
    _context: ResMut<SaveLoadUiContext>,
) {
    // ... existing code ...
}
