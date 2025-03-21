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
    existing_menu_items: Query<Entity, With<MenuCamera>>,
    existing_roots: Query<Entity, With<MenuRoot>>,
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
        // Spawn camera if none exists
        commands.spawn((Camera2d::default(), MenuCamera, Name::new("Menu Camera")));
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
        ImageNode::new(asset_server.load("textures/menu_background.png")),
        MenuBackground,
        MenuItem,
        Name::new("Menu Background"),
    ));

    // Setup Star of David
    setup_main_menu_star(&mut commands, &asset_server);

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
            BackgroundColor(Color::NONE),
            MenuRoot,
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
