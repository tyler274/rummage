use crate::menu::{
    camera::MenuCamera,
    components::MenuRoot,
    logo::{create_english_text, create_hebrew_text},
    star_of_david::{StarOfDavid, create_star_of_david},
};
use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

/// Sets up the main menu star animation and logo
pub fn setup_main_menu_star(_commands: &mut Commands, _asset_server: &AssetServer) {
    info!("Setting up main menu star");

    // Query for menu camera - we'll let the logo plugin handle attaching to camera
    // This function is now deprecated in favor of the logo plugin
    warn!("setup_main_menu_star is deprecated, use LogoPlugin instead");
}

/// Setup Star of David for pause menu
pub fn setup_pause_star(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    asset_server: Res<AssetServer>,
    existing_stars: Query<Entity, With<StarOfDavid>>,
) {
    // Skip if we already have stars to prevent duplicates
    if !existing_stars.is_empty() {
        info!("Stars of David already exist, skipping creation in pause menu");
        return;
    }

    info!("Setting up Star of David for pause menu");

    // Find the menu camera for text elements
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!(
            "Attaching Star of David and text to pause menu camera: {:?}",
            camera_entity
        );

        // Create a complete entity hierarchy using a single commands operation
        commands.entity(camera_entity).with_children(|parent| {
            // Create a parent entity that will contain the star and text elements
            parent
                .spawn((create_logo_container(), Name::new("Pause Logo Group")))
                .with_children(|logo_parent| {
                    // Spawn the Star of David with the logo container as parent
                    logo_parent.spawn((create_star_of_david(), Name::new("Pause Star of David")));

                    // Spawn the Hebrew text below the star
                    logo_parent.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Pause Hebrew Logo Text"),
                    ));

                    // Spawn the English text below the Hebrew text
                    logo_parent.spawn((
                        create_english_text(&asset_server),
                        Name::new("Pause English Logo Text"),
                    ));
                });
        });

        info!("Created pause menu logo container with Star of David and text elements");
    } else {
        warn!("No menu camera found for pause menu Star of David and text");

        // If no camera is found, create a standalone UI node hierarchy
        commands
            .spawn((
                // Ensure this is a proper UI node at the root
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                Name::new("Pause Menu Root"),
                MenuRoot,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Transform::default(),
                GlobalTransform::default(),
            ))
            .with_children(|parent| {
                parent
                    .spawn((create_logo_container(), Name::new("Pause Logo Group")))
                    .with_children(|logo_parent| {
                        // Spawn the Star of David with the logo container as parent
                        logo_parent
                            .spawn((create_star_of_david(), Name::new("Pause Star of David")));

                        // Spawn the Hebrew text below the star
                        logo_parent.spawn((
                            create_hebrew_text(&asset_server),
                            Name::new("Pause Hebrew Logo Text"),
                        ));

                        // Spawn the English text below the Hebrew text
                        logo_parent.spawn((
                            create_english_text(&asset_server),
                            Name::new("Pause English Logo Text"),
                        ));
                    });
            });

        info!("Created standalone pause menu logo container with Star of David and text elements");
    }
}

/// Creates the logo container for menu items
pub fn create_logo_container() -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(300.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        Visibility::Visible,
        ZIndex::default(),    // Ensure we have a ZIndex for proper UI hierarchy
        Transform::default(), // Use Transform instead of TransformBundle
        GlobalTransform::default(), // Add GlobalTransform explicitly
    )
}
