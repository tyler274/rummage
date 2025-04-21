use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Val};

use super::ui_helpers::spawn_menu_button;
use crate::camera::components::AppLayer;
use crate::menu::{
    components::{MenuButtonAction, MenuItem, MenuRoot, ZLayers},
    input_blocker::InputBlocker,
}; // Import the helper function

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
        MenuItem, // Mark as menu item for cleanup
    ));

    // Spawn a pause menu root entity to center the actual menu container
    // This root should fill the screen or be positioned correctly by the caller state
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center, // Center horizontally
                justify_content: JustifyContent::Center, // Center vertically
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)), // Semi-transparent background on root
            Name::new("Pause Menu Root"),
            AppLayer::Menu.layer(),
            ZIndex::from(ZLayers::Background), // Root background layer
            MenuItem,                          // Mark root as menu item for cleanup
        ))
        .with_children(|parent| {
            // Spawn the main pause menu container (the grey box) directly as child of the root
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(450.0), // Adjusted height
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start, // Align content top-to-bottom inside
                        align_items: AlignItems::Center, // Center content horizontally inside
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                    MenuItem,
                    ZIndex::from(ZLayers::MenuContainer), // Container layer
                    Name::new("Pause Menu Container"),
                ))
                .with_children(|inner_parent| {
                    // Logo is handled by LogoPlugin (absolute positioned relative to camera)

                    // Title (PAUSED) - Adjust top margin for logo space
                    inner_parent.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextLayout::default(),
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect {
                                top: Val::Px(80.0), // Adjusted margin for logo
                                bottom: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Pause Menu Title"),
                        MenuItem,
                        AppLayer::Menu.layer(),
                        ZIndex::from(ZLayers::MenuButtonText),
                    ));

                    // Button container (all buttons together)
                    inner_parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0), // Take full width of parent container
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            MenuItem,
                            AppLayer::Menu.layer(),
                            ZIndex::from(ZLayers::MenuButtons), // Use button Z-layer
                            Name::new("Button Container"),
                        ))
                        .with_children(|button_parent| {
                            // Spawn all buttons including Quit Game
                            spawn_menu_button(
                                button_parent,
                                "Resume Game",
                                MenuButtonAction::Resume,
                                "Resume Game Button",
                            );
                            spawn_menu_button(
                                button_parent,
                                "Save Game",
                                MenuButtonAction::SaveGame,
                                "Save Game Button",
                            );
                            spawn_menu_button(
                                button_parent,
                                "Load Game",
                                MenuButtonAction::LoadGame,
                                "Load Game Button",
                            );
                            spawn_menu_button(
                                button_parent,
                                "Settings",
                                MenuButtonAction::Settings,
                                "Settings Button",
                            );
                            spawn_menu_button(
                                button_parent,
                                "Exit to Main Menu",
                                MenuButtonAction::MainMenu,
                                "Exit to Main Menu Button",
                            );
                            spawn_menu_button(
                                // Quit game back in the group
                                button_parent,
                                "Quit Game",
                                MenuButtonAction::Quit,
                                "Quit Game Button",
                            );
                        });
                });
        });
}
