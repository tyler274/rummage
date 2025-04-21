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
                        height: Val::Px(450.0), // Reduced height back
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start, // Align items top-to-bottom
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)), // Uniform padding
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                    MenuItem,
                    ZIndex::from(ZLayers::MenuContainer),
                    Name::new("Pause Menu Container"), // Renamed for clarity
                ))
                .with_children(|inner_parent| {
                    // Logo is handled by LogoPlugin, no need for a placeholder here.

                    // Title (PAUSED) - Add top margin to account for absolute positioned logo
                    inner_parent.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextLayout::default(), // Use default layout
                        TextColor(Color::WHITE),
                        Node {
                            // Use Node for margin
                            margin: UiRect {
                                top: Val::Px(150.0),   // Pushed down significantly for logo space
                                bottom: Val::Px(20.0), // Space below title
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Pause Menu Title"),
                        MenuItem,
                        AppLayer::Menu.layer(),
                        ZIndex::from(ZLayers::MenuButtonText), // Title text z-index
                    ));

                    // Create a container for buttons (excluding Quit Game)
                    inner_parent
                        .spawn((
                            Node {
                                width: Val::Percent(80.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                // Removed top margin, spacing handled by title margin and button margins
                                ..default()
                            },
                            MenuItem,
                            AppLayer::Menu.layer(),
                            ZIndex::from(ZLayers::MenuContainer), // Container z-index
                            Name::new("Inner Button Container"),
                        ))
                        .with_children(|button_parent| {
                            // Use the helper function to spawn buttons, EXCLUDING Quit Game
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
                        });
                }); // End of inner_parent children

            // Spawn the "Quit Game" button separately, as a direct child of the outer container (parent)
            parent
                .spawn((
                    Node {
                        // Wrap button spawner in a Node for positioning if needed
                        margin: UiRect {
                            top: Val::Px(10.0),
                            ..default()
                        }, // Add some space above Quit button
                        ..default()
                    },
                    MenuItem, // Mark container as MenuItem if needed
                    AppLayer::Menu.layer(),
                    ZIndex::from(ZLayers::MenuButtons), // Ensure it's on button layer
                    Name::new("Quit Button Wrapper"),
                ))
                .with_children(|quit_wrapper| {
                    spawn_menu_button(
                        quit_wrapper,
                        "Quit Game",
                        MenuButtonAction::Quit,
                        "Quit Game Button",
                    );
                });
        }); // End of outer parent children
}
