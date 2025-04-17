use bevy::audio::{AudioSink, PlaybackSettings};
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
    // Query for existing music sinks associated with the MainMenuMusic component
    music_sinks: Query<&AudioSink, With<MainMenuMusic>>,
) {
    info!("Setting up main menu interface");

    // --- Camera Setup ---
    // Always despawn existing menu cameras first to ensure a clean state
    let mut existing_camera_count = 0;
    for entity in existing_menu_items.iter() {
        commands.entity(entity).despawn_recursive();
        existing_camera_count += 1;
    }
    if existing_camera_count > 0 {
        info!("Despawned {} existing menu cameras.", existing_camera_count);
    }

    // Find the highest camera order among remaining cameras
    let mut highest_order = 0;
    for camera in all_cameras.iter() {
        // Ensure we don't consider the cameras we just marked for despawn
        // (Commands are deferred, so they might still appear in this query in the same frame)
        // A more robust way would be to query without MenuCamera, but this works for now.
        if camera.order > highest_order {
            highest_order = camera.order;
        }
    }

    // Spawn the main menu camera with proper order
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
    info!("Created main menu camera with order {}", highest_order + 1);

    // --- Root Node Cleanup ---
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

    // Handle background music: Resume if paused, otherwise start fresh
    let mut music_started = false;
    // Check if any sink exists and is paused
    if let Ok(sink) = music_sinks.get_single() {
        if sink.is_paused() {
            info!("Resuming main menu music.");
            sink.play();
            music_started = true;
        } else {
            // If it exists but is not paused (playing or stopped), treat as handled
            info!("Main menu music already exists and is not paused.");
            music_started = true;
        }
    } else if music_sinks.iter().count() > 1 {
        warn!("Multiple MainMenuMusic sinks found during setup! Starting new music anyway.");
        // Let it fall through to start new music, but log a warning.
    }

    if !music_started {
        info!("Starting main menu music.");
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
    }

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
