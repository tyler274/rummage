use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

use crate::menu::{
    components::{MenuCamera, MenuItem, MenuRoot},
    save_load::SaveExists,
    systems::{
        components::text::MenuButtonTextBundle,
        main_menu::buttons::{MenuButtonBundle, MenuContainerBundle, MenuRootBundle},
    },
};

/// Sets up the main menu UI elements
pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    save_exists: Res<SaveExists>,
) {
    info!("Setting up main menu");

    // Find the menu camera to attach UI elements to
    if let Some(camera) = menu_cameras.iter().next() {
        info!("Found menu camera for main menu UI: {:?}", camera);

        // Create main menu buttons under the camera
        commands.entity(camera).with_children(|parent| {
            // Main menu buttons container
            parent
                .spawn(MenuContainerBundle::button_container())
                .with_children(|container| {
                    // New Game button
                    container
                        .spawn(MenuButtonBundle::new("New Game", 51))
                        .with_children(|button| {
                            button.spawn(MenuButtonTextBundle::new(&asset_server, "New Game", 52));
                        });

                    // Load Game button - only show if save exists
                    if save_exists.0 {
                        container
                            .spawn(MenuButtonBundle::new("Load Game", 51))
                            .with_children(|button| {
                                button.spawn(MenuButtonTextBundle::new(
                                    &asset_server,
                                    "Load Game",
                                    52,
                                ));
                            });
                    }

                    // Multiplayer button
                    container
                        .spawn(MenuButtonBundle::new("Multiplayer", 51))
                        .with_children(|button| {
                            button.spawn(MenuButtonTextBundle::new(
                                &asset_server,
                                "Multiplayer",
                                52,
                            ));
                        });

                    // Settings button
                    container
                        .spawn(MenuButtonBundle::new("Settings", 51))
                        .with_children(|button| {
                            button.spawn(MenuButtonTextBundle::new(&asset_server, "Settings", 52));
                        });

                    // Credits button
                    container
                        .spawn(MenuButtonBundle::new("Credits", 51))
                        .with_children(|button| {
                            button.spawn(MenuButtonTextBundle::new(&asset_server, "Credits", 52));
                        });

                    // Exit button
                    container
                        .spawn(MenuButtonBundle::new("Exit", 51))
                        .with_children(|button| {
                            button.spawn(MenuButtonTextBundle::new(&asset_server, "Exit", 52));
                        });
                });
        });

        info!("Main menu buttons attached to camera entity");
    } else {
        warn!("No menu camera found, creating standalone main menu");

        // Create a root node with buttons as children
        commands
            .spawn(MenuRootBundle::new())
            .with_children(|parent| {
                // Buttons container
                parent
                    .spawn(MenuContainerBundle::button_container())
                    .with_children(|container| {
                        // New Game button
                        container
                            .spawn(MenuButtonBundle::new("New Game", 52))
                            .with_children(|button| {
                                button.spawn(MenuButtonTextBundle::new(
                                    &asset_server,
                                    "New Game",
                                    53,
                                ));
                            });

                        // Load Game button - only show if save exists
                        if save_exists.0 {
                            container
                                .spawn(MenuButtonBundle::new("Load Game", 52))
                                .with_children(|button| {
                                    button.spawn(MenuButtonTextBundle::new(
                                        &asset_server,
                                        "Load Game",
                                        53,
                                    ));
                                });
                        }

                        // Multiplayer button
                        container
                            .spawn(MenuButtonBundle::new("Multiplayer", 52))
                            .with_children(|button| {
                                button.spawn(MenuButtonTextBundle::new(
                                    &asset_server,
                                    "Multiplayer",
                                    53,
                                ));
                            });

                        // Settings button
                        container
                            .spawn(MenuButtonBundle::new("Settings", 52))
                            .with_children(|button| {
                                button.spawn(MenuButtonTextBundle::new(
                                    &asset_server,
                                    "Settings",
                                    53,
                                ));
                            });

                        // Credits button
                        container
                            .spawn(MenuButtonBundle::new("Credits", 52))
                            .with_children(|button| {
                                button.spawn(MenuButtonTextBundle::new(
                                    &asset_server,
                                    "Credits",
                                    53,
                                ));
                            });

                        // Exit button
                        container
                            .spawn(MenuButtonBundle::new("Exit", 52))
                            .with_children(|button| {
                                button.spawn(MenuButtonTextBundle::new(&asset_server, "Exit", 53));
                            });
                    });
            });

        info!("Created standalone main menu");
    }
}
