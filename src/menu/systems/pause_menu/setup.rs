use crate::camera::components::AppLayer;
use crate::menu::{
    components::*, input_blocker::InputBlocker, systems::pause_menu::buttons::spawn_menu_button,
};
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, Val};

/// Sets up the pause menu interface
pub fn setup_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_menu_items: Query<Entity, With<MenuItem>>,
) {
    // Check for existing menu items first and clean them up if necessary
    let existing_count = existing_menu_items.iter().count();
    if existing_count > 0 {
        info!(
            "Found {} existing menu items, cleaning up before creating pause menu",
            existing_count
        );
        for entity in existing_menu_items.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

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
        MenuItem,
        Name::new("Pause Menu Input Blocker"),
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
            GlobalZIndex(-5),
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
                        padding: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Logo container is now the first child above the PAUSED text
                    // The star component itself will be created in the setup_pause_star function
                    parent.spawn((
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(150.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
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
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(80.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(30.0)),
                                ..default()
                            },
                            MenuItem,
                            AppLayer::Menu.layer(),
                        ))
                        .with_children(|parent| {
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

                            // Quit Game Button
                            spawn_menu_button(
                                parent,
                                "Quit Game",
                                MenuButtonAction::QuitGame,
                                &asset_server,
                            );
                        });
                });
        });
}
