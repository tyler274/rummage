use bevy::audio::PlaybackSettings;
use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, PositionType, Val};

use crate::{
    camera::components::MenuCamera,
    menu::{
        components::{MenuItem, MenuRoot, ZLayers},
        save_load::SaveExists,
        save_load::resources::check_save_exists,
    },
};

use super::super::components::{MainMenuBackground, MainMenuMusic};
use super::buttons::create_main_menu_buttons;

/// Sets up the main menu interface with buttons and layout
pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_menu_items: Query<Entity, With<MenuCamera>>,
    existing_roots: Query<Entity, With<MenuRoot>>,
    all_cameras: Query<&Camera>,
    save_exists: ResMut<SaveExists>,
) {
    info!("Setting up main menu interface");

    // Check for existing menu items first
    let menu_items_count = existing_menu_items.iter().count();
    if menu_items_count > 0 {
        info!(
            "Found {} existing menu cameras, they will be handled by camera systems",
            menu_items_count
        );
    } else {
        // Find the highest camera order
        let mut highest_order = 0;
        for camera in all_cameras.iter() {
            if camera.order > highest_order {
                highest_order = camera.order;
            }
        }

        // Spawn camera with proper order if none exists
        commands.spawn((
            Camera2d,
            Camera {
                order: highest_order + 1,
                ..default()
            },
            MenuCamera,
            Name::new("Menu Camera"),
            // Add essential UI components to make it a valid UI parent
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ViewVisibility::default(),
            InheritedVisibility::default(),
            Visibility::Visible,
            ZIndex::default(),
        ));

        info!("Created menu camera with order {}", highest_order + 1);
    }

    // Clean up any existing menu items with MenuRoot
    for entity in existing_roots.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Setup background
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 1.0)),
        ImageNode::new(asset_server.load("images/menu_background.jpeg")),
        MainMenuBackground,
        MenuItem,
        Into::<ZIndex>::into(ZLayers::Background),
        Name::new("Menu Background"),
    ));

    // Set up background music
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/negev_hava_nagila.ogg")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
        },
        MainMenuMusic,
        MenuItem,
        Name::new("Main Menu Music"),
    ));

    // Note: Star of David and logo setup is now handled by the LogoPlugin

    // Create the main container
    let container = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)), // Semi-transparent overlay
            MenuRoot,
            MenuItem,            // Add MenuItem marker for visibility systems
            Visibility::Visible, // Force visibility
            Into::<ZIndex>::into(ZLayers::MenuContainer), // Add proper z-index
            Name::new("Main Menu Root"),
        ))
        .id();

    // Check if save exists and store the value
    let has_save = {
        let mut save_exists_ref = save_exists;
        check_save_exists(&mut save_exists_ref);
        save_exists_ref.0
    };

    // Add menu buttons as children to the container
    commands.entity(container).with_children(|parent| {
        create_main_menu_buttons(parent, &asset_server, has_save);
    });
}
