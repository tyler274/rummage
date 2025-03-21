use crate::menu::logo::{
    StarOfDavid, create_english_text, create_hebrew_text, create_logo, create_star_of_david,
};
use crate::menu::{camera::MenuCamera, components::MenuRoot, decorations::MenuDecorativeElement};
use bevy::prelude::*;

/// Sets up a Star of David for the main menu and attaches it to the menu camera
pub fn setup_main_menu_star(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    asset_server: Res<AssetServer>,
    existing_stars: Query<(Entity, &Visibility, Option<&Name>), With<StarOfDavid>>,
) {
    info!("Setting up Star of David for main menu");

    // Check for existing stars first and handle their visibility
    let star_count = existing_stars.iter().count();
    if star_count > 0 {
        info!(
            "Found {} existing Star of David entities, managing visibility for main menu",
            star_count
        );

        // First hide ALL stars to ensure a clean slate
        for (entity, _, _) in existing_stars.iter() {
            commands.entity(entity).insert(Visibility::Hidden);
        }

        // Then find main menu-specific stars to show (or create if none exist)
        let mut main_menu_stars = Vec::new();

        // Find main menu-specific stars
        for (entity, _, name) in existing_stars.iter() {
            let is_main_menu_star = name.map_or(false, |n| n.as_str().contains("Main Menu"));
            if is_main_menu_star {
                main_menu_stars.push(entity);
            }
        }

        // Show only the first main menu star if any exist
        if let Some(&first_main_menu_star) = main_menu_stars.first() {
            info!("Making main menu star visible: {:?}", first_main_menu_star);
            commands
                .entity(first_main_menu_star)
                .insert(Visibility::Visible);
            return; // Exit early, we're done
        }

        // Otherwise create a new main menu star (fall through to creation code)
        info!("No main menu-specific stars found, will create a new one");
    }

    // Find the menu camera for text elements
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!(
            "Attaching Star of David and text to main menu camera: {:?}",
            camera_entity
        );

        // Create a complete entity hierarchy using a single commands operation
        commands.entity(camera_entity).with_children(|parent| {
            // Create a parent entity that will contain the star and text elements
            parent
                .spawn((
                    create_logo(),
                    Name::new("Main Menu Logo Group"),
                    MenuDecorativeElement,
                ))
                .with_children(|logo_parent| {
                    // Spawn the Star of David with the logo container as parent
                    logo_parent
                        .spawn((create_star_of_david(), Name::new("Main Menu Star of David")));

                    // Spawn the Hebrew text below the star
                    logo_parent.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Main Menu Hebrew Logo Text"),
                    ));

                    // Spawn the English text below the Hebrew text
                    logo_parent.spawn((
                        create_english_text(&asset_server),
                        Name::new("Main Menu English Logo Text"),
                    ));
                });
        });

        info!("Created main menu logo container with Star of David and text elements");
    } else {
        warn!("No menu camera found for attaching Star of David");
    }
}

/// Setup Star of David for pause menu
pub fn setup_pause_star(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    logo_positions: Query<(Entity, &Name), With<Node>>,
    asset_server: Res<AssetServer>,
    existing_stars: Query<(Entity, &Visibility, Option<&Name>), With<StarOfDavid>>,
) {
    // Check for existing stars first and handle their visibility
    let star_count = existing_stars.iter().count();
    if star_count > 0 {
        info!(
            "Found {} existing Star of David entities, managing visibility for pause menu",
            star_count
        );

        // First hide ALL stars to ensure a clean slate
        for (entity, _, _) in existing_stars.iter() {
            commands.entity(entity).insert(Visibility::Hidden);
        }

        // Then find pause-specific stars to show (or create if none exist)
        let mut pause_stars = Vec::new();

        // Find pause-specific stars
        for (entity, _, name) in existing_stars.iter() {
            let is_pause_star = name.map_or(false, |n| n.as_str().contains("Pause"));
            if is_pause_star {
                pause_stars.push(entity);
            }
        }

        // Show only the first pause star if any exist
        if let Some(&first_pause_star) = pause_stars.first() {
            info!("Making pause star visible: {:?}", first_pause_star);
            commands
                .entity(first_pause_star)
                .insert(Visibility::Visible);
            return; // Exit early, we're done
        }

        // Otherwise create a new pause star (fall through to creation code)
        info!("No pause-specific stars found, will create a new one");
    }

    // First, look for the Logo Position node we created in the pause menu
    let mut logo_position_entity = None;
    for (entity, name) in logo_positions.iter() {
        if name.as_str() == "Logo Position" {
            logo_position_entity = Some(entity);
            info!("Found Logo Position node: {:?}", entity);
            break;
        }
    }

    if let Some(position_entity) = logo_position_entity {
        info!(
            "Attaching Star of David to Logo Position node: {:?}",
            position_entity
        );

        // Create a complete entity hierarchy using a single commands operation
        commands.entity(position_entity).with_children(|parent| {
            // Create a parent entity that will contain the star and text elements
            parent
                .spawn((
                    create_logo(),
                    Name::new("Pause Logo Group"),
                    MenuDecorativeElement,
                ))
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
        return;
    }

    // If no logo position found, try to attach to the menu camera as a fallback
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!(
            "No Logo Position found, attaching Star of David to menu camera: {:?} as fallback",
            camera_entity
        );

        // Create a complete entity hierarchy using a single commands operation
        commands.entity(camera_entity).with_children(|parent| {
            // Create a parent entity that will contain the star and text elements
            parent
                .spawn((
                    create_logo(),
                    Name::new("Pause Logo Group"),
                    MenuDecorativeElement,
                ))
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
        warn!("No menu camera or Logo Position found for pause menu Star of David and text");

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
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Transform::default(),
                GlobalTransform::default(),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        create_logo(),
                        Name::new("Pause Logo Group"),
                        MenuDecorativeElement,
                    ))
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
