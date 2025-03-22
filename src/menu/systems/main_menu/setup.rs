use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, PositionType, Val};

use crate::menu::{
    backgrounds::MenuBackground,
    camera::MenuCamera,
    components::{MenuItem, MenuRoot},
    save_load::SaveExists,
    save_load::resources::check_save_exists,
    systems::{logo::setup_main_menu_star, main_menu::buttons::create_main_menu_buttons},
};

/// Sets up the main menu interface with buttons and layout
pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_roots: Query<Entity, With<MenuRoot>>,
    save_exists: ResMut<SaveExists>,
) {
    info!("Setting up main menu interface");

    // Get or create the menu camera
    let camera_entity = if let Some(camera) = menu_cameras.iter().next() {
        info!("Using existing menu camera: {:?}", camera);
        camera
    } else {
        info!("No menu camera found, creating one");
        commands
            .spawn((
                Camera2d::default(),
                Camera {
                    order: 1, // Ensure this has a distinct order
                    ..default()
                },
                MenuCamera,
                Name::new("Menu Camera"),
            ))
            .id()
    };

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
        ImageNode::new(asset_server.load("textures/menu_background.png")),
        MenuBackground,
        MenuItem,
        Name::new("Menu Background"),
    ));

    // Setup Star of David
    setup_main_menu_star(&mut commands, &asset_server);

    // Check if save exists and store the value
    let has_save = {
        let mut save_exists_ref = save_exists;
        check_save_exists(&mut save_exists_ref);
        save_exists_ref.0
    };

    // Attach the main menu to the camera entity
    commands
        .entity(camera_entity)
        .with_children(|camera_parent| {
            // Create the main container
            camera_parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    MenuRoot,
                    Name::new("Main Menu Root"),
                ))
                .with_children(|parent| {
                    create_main_menu_buttons(parent, &asset_server, has_save);
                });
        });
}
