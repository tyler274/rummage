use crate::camera::components::AppLayer;
use crate::menu::camera::MenuCamera;
use crate::menu::components::{MenuItem, ZLayers};
use crate::menu::decorations::MenuDecorativeElement;
use crate::menu::logo::text::{create_english_text, create_hebrew_text};
use crate::menu::star_of_david::create_star_of_david;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;

/// Plugin for menu logo
pub struct LogoPlugin;

impl Plugin for LogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::MainMenu), setup_combined_logo)
            .add_systems(OnEnter(GameMenuState::PauseMenu), setup_pause_logo)
            .add_systems(OnExit(GameMenuState::MainMenu), cleanup_logo)
            .add_systems(OnExit(GameMenuState::PauseMenu), cleanup_logo);

        debug!("Logo plugin registered - combines Star of David with text");
    }
}

/// Sets up the combined logo with Star of David and text for the main menu
fn setup_combined_logo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
) {
    info!("Setting up combined logo with Star of David and text for main menu");

    // Find the menu camera to attach to
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching logo to menu camera: {:?}", camera_entity);

        // Attach logo to camera entity
        commands.entity(camera_entity).with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    MenuDecorativeElement,
                    MenuItem,
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
                    InheritedVisibility::VISIBLE,
                    ViewVisibility::default(),
                    ZIndex::from(ZLayers::MenuContainer),
                    Name::new("Main Menu Logo Container"),
                ))
                .with_children(|logo_parent| {
                    // Spawn the Star of David as part of the logo
                    logo_parent
                        .spawn((create_star_of_david(), Name::new("Main Menu Star of David")));

                    // Add Hebrew text
                    logo_parent.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Main Menu Hebrew Text"),
                    ));

                    // Add English text
                    logo_parent.spawn((
                        create_english_text(&asset_server),
                        Name::new("Main Menu English Text"),
                    ));
                });
        });
    } else {
        warn!("No menu camera found, cannot attach logo!");
    }
}

/// Cleans up the logo entities
fn cleanup_logo(mut commands: Commands, logos: Query<Entity, With<MenuDecorativeElement>>) {
    let count = logos.iter().count();
    info!("Cleaning up {} logo entities", count);

    for entity in logos.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Sets up the pause menu logo
fn setup_pause_logo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
) {
    info!("Setting up logo for pause menu");

    // Find the menu camera for attachment
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching pause logo to camera: {:?}", camera_entity);

        // Attach to camera entity
        commands.entity(camera_entity).with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    MenuDecorativeElement,
                    MenuItem,
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
                    InheritedVisibility::VISIBLE,
                    ViewVisibility::default(),
                    ZIndex::from(ZLayers::MenuContainer),
                    Name::new("Pause Menu Logo Container"),
                ))
                .with_children(|logo_parent| {
                    // Spawn the Star of David
                    logo_parent.spawn((create_star_of_david(), Name::new("Pause Star of David")));

                    // Add Hebrew text
                    logo_parent.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Pause Hebrew Text"),
                    ));

                    // Add English text
                    logo_parent.spawn((
                        create_english_text(&asset_server),
                        Name::new("Pause English Text"),
                    ));
                });
        });
    } else {
        warn!("No menu camera found, cannot attach pause logo!");
    }
}
