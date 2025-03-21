use crate::menu::state::MenuState;
use crate::menu::{
    components::{MenuBackground, MenuCamera, MenuItem, MenuRoot},
    styles::button_styles::{create_main_menu_button, create_main_menu_button_text},
};
use crate::net::multiplayer::state::MultiplayerState;
use crate::save_load::{SaveExists, check_save_exists};
use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

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
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(300.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(20.0)),
                        ..default()
                    },
                    Name::new("Main Menu Button Container"),
                    MenuItem,
                    Visibility::Visible,
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    GlobalZIndex(50),
                ))
                .with_children(|container| {
                    // New Game button
                    container
                        .spawn((
                            create_main_menu_button(),
                            Name::new("New Game Button"),
                            MenuItem,
                            Visibility::Visible,
                            InheritedVisibility::default(),
                            ViewVisibility::default(),
                            GlobalZIndex(51),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                create_main_menu_button_text(&asset_server, "New Game"),
                                Name::new("New Game Button Text"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ));
                        });

                    // Load Game button - only show if save exists
                    if save_exists.0 {
                        container
                            .spawn((
                                create_main_menu_button(),
                                Name::new("Load Game Button"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(51),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    create_main_menu_button_text(&asset_server, "Load Game"),
                                    Name::new("Load Game Button Text"),
                                    MenuItem,
                                    Visibility::Visible,
                                    InheritedVisibility::default(),
                                    ViewVisibility::default(),
                                    GlobalZIndex(52),
                                ));
                            });
                    }

                    // Multiplayer button
                    container
                        .spawn((
                            create_main_menu_button(),
                            Name::new("Multiplayer Button"),
                            MenuItem,
                            Visibility::Visible,
                            InheritedVisibility::default(),
                            ViewVisibility::default(),
                            GlobalZIndex(51),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                create_main_menu_button_text(&asset_server, "Multiplayer"),
                                Name::new("Multiplayer Button Text"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ));
                        });

                    // Settings button
                    container
                        .spawn((
                            create_main_menu_button(),
                            Name::new("Settings Button"),
                            MenuItem,
                            Visibility::Visible,
                            InheritedVisibility::default(),
                            ViewVisibility::default(),
                            GlobalZIndex(51),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                create_main_menu_button_text(&asset_server, "Settings"),
                                Name::new("Settings Button Text"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ));
                        });

                    // Credits button
                    container
                        .spawn((
                            create_main_menu_button(),
                            Name::new("Credits Button"),
                            MenuItem,
                            Visibility::Visible,
                            InheritedVisibility::default(),
                            ViewVisibility::default(),
                            GlobalZIndex(51),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                create_main_menu_button_text(&asset_server, "Credits"),
                                Name::new("Credits Button Text"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ));
                        });

                    // Exit button
                    container
                        .spawn((
                            create_main_menu_button(),
                            Name::new("Exit Button"),
                            MenuItem,
                            Visibility::Visible,
                            InheritedVisibility::default(),
                            ViewVisibility::default(),
                            GlobalZIndex(51),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                create_main_menu_button_text(&asset_server, "Exit"),
                                Name::new("Exit Button Text"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ));
                        });
                });
        });

        info!("Main menu buttons attached to camera entity");
    } else {
        warn!("No menu camera found, creating standalone main menu");

        // Create a root node with buttons as children
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                Name::new("Main Menu Root"),
                MenuRoot,
                MenuItem,
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                GlobalZIndex(50),
            ))
            .with_children(|parent| {
                // Buttons container
                parent
                    .spawn((
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(300.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(20.0)),
                            ..default()
                        },
                        Name::new("Main Menu Button Container"),
                        MenuItem,
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                        GlobalZIndex(51),
                    ))
                    .with_children(|container| {
                        // New Game button
                        container
                            .spawn((
                                create_main_menu_button(),
                                Name::new("New Game Button"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    create_main_menu_button_text(&asset_server, "New Game"),
                                    Name::new("New Game Button Text"),
                                    MenuItem,
                                    Visibility::Visible,
                                    InheritedVisibility::default(),
                                    ViewVisibility::default(),
                                    GlobalZIndex(53),
                                ));
                            });

                        // Load Game button - only show if save exists
                        if save_exists.0 {
                            container
                                .spawn((
                                    create_main_menu_button(),
                                    Name::new("Load Game Button"),
                                    MenuItem,
                                    Visibility::Visible,
                                    InheritedVisibility::default(),
                                    ViewVisibility::default(),
                                    GlobalZIndex(52),
                                ))
                                .with_children(|button| {
                                    button.spawn((
                                        create_main_menu_button_text(&asset_server, "Load Game"),
                                        Name::new("Load Game Button Text"),
                                        MenuItem,
                                        Visibility::Visible,
                                        InheritedVisibility::default(),
                                        ViewVisibility::default(),
                                        GlobalZIndex(53),
                                    ));
                                });
                        }

                        // Multiplayer button
                        container
                            .spawn((
                                create_main_menu_button(),
                                Name::new("Multiplayer Button"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    create_main_menu_button_text(&asset_server, "Multiplayer"),
                                    Name::new("Multiplayer Button Text"),
                                    MenuItem,
                                    Visibility::Visible,
                                    InheritedVisibility::default(),
                                    ViewVisibility::default(),
                                    GlobalZIndex(53),
                                ));
                            });

                        // Settings button
                        container
                            .spawn((
                                create_main_menu_button(),
                                Name::new("Settings Button"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    create_main_menu_button_text(&asset_server, "Settings"),
                                    Name::new("Settings Button Text"),
                                    MenuItem,
                                    Visibility::Visible,
                                    InheritedVisibility::default(),
                                    ViewVisibility::default(),
                                    GlobalZIndex(53),
                                ));
                            });

                        // Credits button
                        container
                            .spawn((
                                create_main_menu_button(),
                                Name::new("Credits Button"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    create_main_menu_button_text(&asset_server, "Credits"),
                                    Name::new("Credits Button Text"),
                                    MenuItem,
                                    Visibility::Visible,
                                    InheritedVisibility::default(),
                                    ViewVisibility::default(),
                                    GlobalZIndex(53),
                                ));
                            });

                        // Exit button
                        container
                            .spawn((
                                create_main_menu_button(),
                                Name::new("Exit Button"),
                                MenuItem,
                                Visibility::Visible,
                                InheritedVisibility::default(),
                                ViewVisibility::default(),
                                GlobalZIndex(52),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    create_main_menu_button_text(&asset_server, "Exit"),
                                    Name::new("Exit Button Text"),
                                    MenuItem,
                                    Visibility::Visible,
                                    InheritedVisibility::default(),
                                    ViewVisibility::default(),
                                    GlobalZIndex(53),
                                ));
                            });
                    });
            });

        info!("Created standalone main menu");
    }
}

/// Handle button clicks in the main menu
pub fn handle_main_menu_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Name, &Parent),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Parent, With<Text>>,
    mut next_state: ResMut<NextState<MenuState>>,
    mut multi_state: ResMut<NextState<MultiplayerState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, mut color, name, parent) in interaction_query.iter_mut() {
        let button_name = name.as_str();

        match *interaction {
            Interaction::Pressed => {
                info!("Button pressed: {}", button_name);

                // Handle different buttons based on their names
                match button_name {
                    "New Game Button" => {
                        info!("New Game selected");
                        next_state.set(MenuState::NewGame);
                    }
                    "Load Game Button" => {
                        info!("Load Game selected");
                        next_state.set(MenuState::LoadGame);
                    }
                    "Multiplayer Button" => {
                        info!("Multiplayer selected");
                        multi_state.set(MultiplayerState::Menu);
                    }
                    "Settings Button" => {
                        info!("Settings selected");
                        next_state.set(MenuState::Settings);
                    }
                    "Credits Button" => {
                        info!("Credits selected");
                        next_state.set(MenuState::Credits);
                    }
                    "Exit Button" => {
                        info!("Exit selected");
                        exit.send(bevy::app::AppExit);
                    }
                    _ => {
                        // Check for text elements with parent buttons
                        for text_parent in text_query.iter() {
                            if text_parent.get() == parent.get() {
                                info!("Button with text pressed: {}", button_name);
                                // Handle based on the parent button's name
                                if button_name.contains("New Game") {
                                    next_state.set(MenuState::NewGame);
                                } else if button_name.contains("Load Game") {
                                    next_state.set(MenuState::LoadGame);
                                } else if button_name.contains("Multiplayer") {
                                    multi_state.set(MultiplayerState::Menu);
                                } else if button_name.contains("Settings") {
                                    next_state.set(MenuState::Settings);
                                } else if button_name.contains("Credits") {
                                    next_state.set(MenuState::Credits);
                                } else if button_name.contains("Exit") {
                                    exit.send(bevy::app::AppExit);
                                }
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                // Highlight button on hover
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                // Reset color when not interacting
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

/// Sets up the main menu background
pub fn setup_menu_background(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    window: Query<&Window>,
) {
    // Get window dimensions to set appropriate background size
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    info!(
        "Setting up menu background with window dimensions: {}x{}",
        width, height
    );

    // Find the menu camera to attach the background to
    if let Some(camera) = menu_cameras.iter().next() {
        info!("Found menu camera for background: {:?}", camera);

        // Create and attach background to camera
        commands.entity(camera).with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(width),
                        height: Val::Px(height),
                        position_type: bevy::ui::PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(-10),
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.85).into(),
                    ..default()
                },
                Name::new("Menu Background"),
                MenuBackground,
                MenuItem,
                GlobalZIndex(-10),
            ));
        });

        info!("Menu background attached to camera entity");
    } else {
        warn!("No menu camera found, creating standalone background");

        // Create a standalone background
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(width),
                    height: Val::Px(height),
                    position_type: bevy::ui::PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                z_index: ZIndex::Global(-10),
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.85).into(),
                ..default()
            },
            Name::new("Menu Background"),
            MenuBackground,
            MenuRoot,
            MenuItem,
            GlobalZIndex(-10),
        ));

        info!("Created standalone menu background");
    }
}
