use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Val};

use crate::camera::components::AppLayer;
use crate::menu::{
    components::{MenuButtonAction, MenuItem, MenuRoot},
    input_blocker::InputBlocker,
    styles::NORMAL_BUTTON,
    systems::pause_menu::buttons::spawn_menu_button,
};

/// Sets up the pause menu interface
pub fn setup_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_menu_items: Query<Entity, With<MenuItem>>,
) {
    // First despawn any existing menu items to avoid duplication
    for entity in existing_menu_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Add an input blocker to catch keyboard/mouse input
    commands.spawn((
        InputBlocker,
        AppLayer::Menu.layer(),
        Name::new("Pause Menu Input Blocker"),
    ));

    // Spawn a pause menu root entity to help with cleanup
    commands.spawn((
        MenuRoot,
        Name::new("Pause Menu Root"),
        AppLayer::Menu.layer(),
    ));

    // Entity ID for the button container
    let mut button_container_entity = None;

    // Then spawn the pause menu UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            MenuItem,
            AppLayer::Menu.layer(),
            ZIndex::default(),
        ))
        .with_children(|parent| {
            // Pause menu container
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(450.0), // Increased height to accommodate logo
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start, // Changed to start for better positioning
                        align_items: AlignItems::Center,
                        padding: UiRect {
                            top: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Logo container is now the first child above the PAUSED text
                    parent.spawn((
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(150.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect {
                                bottom: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Logo Position"),
                        MenuItem,
                        AppLayer::Menu.layer(),
                    ));

                    // Title (now appears after the logo)
                    parent.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor(Color::WHITE),
                        Name::new("Pause Menu Title"),
                        MenuItem,
                        AppLayer::Menu.layer(),
                    ));

                    // Create a container for buttons to control spacing
                    let container_entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(80.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    top: Val::Px(30.0),
                                    ..default()
                                },
                                ..default()
                            },
                            MenuItem,
                            AppLayer::Menu.layer(),
                        ))
                        .id();

                    // Store the entity ID for later use
                    button_container_entity = Some(container_entity);
                });
        });

    // Add buttons to the container
    if let Some(container_entity) = button_container_entity {
        // Add the standard buttons
        commands.entity(container_entity).with_children(|parent| {
            // Resume Game Button
            spawn_menu_button(
                parent,
                "Resume Game",
                MenuButtonAction::Resume,
                &asset_server,
            );

            // Save Game Button
            spawn_menu_button(
                parent,
                "Save Game",
                MenuButtonAction::SaveGame,
                &asset_server,
            );

            // Load Game Button
            spawn_menu_button(
                parent,
                "Load Game",
                MenuButtonAction::LoadGame,
                &asset_server,
            );

            // Settings Button
            spawn_menu_button(
                parent,
                "Settings",
                MenuButtonAction::Settings,
                &asset_server,
            );

            // Exit to Main Menu Button
            spawn_menu_button(
                parent,
                "Exit to Main Menu",
                MenuButtonAction::MainMenu,
                &asset_server,
            );

            // Create Quit button
            parent
                .spawn((
                    Name::new("Quit Game Button"),
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    MenuItem,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit Game"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        });
    }
}
