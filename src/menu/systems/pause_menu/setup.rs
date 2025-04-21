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
                        height: Val::Px(550.0), // Increased height
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center, // Changed back to center
                        align_items: AlignItems::Center,
                        padding: UiRect {
                            top: Val::Px(20.0),
                            bottom: Val::Px(20.0), // Added bottom padding
                            ..default()
                        },
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                    MenuItem,
                    ZIndex::from(ZLayers::MenuContainer),
                    Name::new("Button Container"),
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
                    // Quit Game button is removed from here
                });
            // Spawn the "Quit Game" button separately, directly under the main container
            spawn_menu_button(
                parent, // Spawn as child of the main container node, not the inner button container
                "Quit Game",
                MenuButtonAction::Quit,
                "Quit Game Button",
            );
        });
}
