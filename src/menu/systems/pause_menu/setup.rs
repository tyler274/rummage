use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Val};

use crate::camera::components::AppLayer;
use crate::menu::{
    components::{MenuButtonAction, MenuItem, MenuRoot, ZLayers},
    input_blocker::InputBlocker,
    styles::NORMAL_BUTTON,
};

/// Sets up the pause menu interface
pub fn setup_pause_menu(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
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
        ZIndex::from(ZLayers::Overlay),
    ));

    // Spawn a pause menu root entity to help with cleanup
    commands.spawn((
        MenuRoot,
        Name::new("Pause Menu Root"),
        AppLayer::Menu.layer(),
        ZIndex::from(ZLayers::Background),
    ));

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
            ZIndex::from(ZLayers::Background),
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
                    MenuItem,
                    ZIndex::from(ZLayers::MenuContainer),
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
                        ZIndex::from(ZLayers::LogoIcon),
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
                        ZIndex::from(ZLayers::MenuButtonText),
                    ));

                    // Create a container for buttons to control spacing and add buttons within it
                    parent
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
                            ZIndex::from(ZLayers::MenuContainer),
                            Name::new("Button Container"),
                        ))
                        .with_children(|button_parent| {
                            // Manually spawn all buttons based on Quit Game pattern

                            // Resume Game Button
                            button_parent
                                .spawn((
                                    Name::new("Resume Game Button"),
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
                                    MenuButtonAction::Resume,
                                    MenuItem,
                                    ZIndex::from(ZLayers::MenuButtons),
                                ))
                                .with_children(|text_parent| {
                                    text_parent.spawn((
                                        Text::new("Resume Game"),
                                        TextFont {
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        MenuItem,
                                        ZIndex::from(ZLayers::MenuButtonText),
                                    ));
                                });

                            // Save Game Button
                            button_parent
                                .spawn((
                                    Name::new("Save Game Button"),
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
                                    MenuButtonAction::SaveGame,
                                    MenuItem,
                                    ZIndex::from(ZLayers::MenuButtons),
                                ))
                                .with_children(|text_parent| {
                                    text_parent.spawn((
                                        Text::new("Save Game"),
                                        TextFont {
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        MenuItem,
                                        ZIndex::from(ZLayers::MenuButtonText),
                                    ));
                                });

                            // Load Game Button
                            button_parent
                                .spawn((
                                    Name::new("Load Game Button"),
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
                                    MenuButtonAction::LoadGame,
                                    MenuItem,
                                    ZIndex::from(ZLayers::MenuButtons),
                                ))
                                .with_children(|text_parent| {
                                    text_parent.spawn((
                                        Text::new("Load Game"),
                                        TextFont {
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        MenuItem,
                                        ZIndex::from(ZLayers::MenuButtonText),
                                    ));
                                });

                            // Settings Button
                            button_parent
                                .spawn((
                                    Name::new("Settings Button"),
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
                                    MenuButtonAction::Settings,
                                    MenuItem,
                                    ZIndex::from(ZLayers::MenuButtons),
                                ))
                                .with_children(|text_parent| {
                                    text_parent.spawn((
                                        Text::new("Settings"),
                                        TextFont {
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        MenuItem,
                                        ZIndex::from(ZLayers::MenuButtonText),
                                    ));
                                });

                            // Exit to Main Menu Button
                            button_parent
                                .spawn((
                                    Name::new("Exit to Main Menu Button"),
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
                                    MenuButtonAction::MainMenu,
                                    MenuItem,
                                    ZIndex::from(ZLayers::MenuButtons),
                                ))
                                .with_children(|text_parent| {
                                    text_parent.spawn((
                                        Text::new("Exit to Main Menu"),
                                        TextFont {
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        MenuItem,
                                        ZIndex::from(ZLayers::MenuButtonText),
                                    ));
                                });

                            // Create Quit button with proper Z-index
                            button_parent
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
                                    ZIndex::from(ZLayers::MenuButtons),
                                ))
                                .with_children(|quit_button_text_parent| {
                                    quit_button_text_parent.spawn((
                                        Text::new("Quit Game"),
                                        TextFont {
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        MenuItem,
                                        ZIndex::from(ZLayers::MenuButtonText),
                                    ));
                                });
                        });
                });
        });
}
