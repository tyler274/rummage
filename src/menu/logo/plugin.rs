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
        app.add_systems(Startup, register_logo_components)
            // Add the logo setup on app start and when entering main menu
            .add_systems(Startup, setup_combined_logo)
            .add_systems(OnEnter(GameMenuState::MainMenu), setup_combined_logo)
            .add_systems(OnEnter(GameMenuState::PauseMenu), setup_pause_logo)
            // Cleanup on exit
            .add_systems(OnExit(GameMenuState::MainMenu), cleanup_logo)
            .add_systems(OnExit(GameMenuState::PauseMenu), cleanup_logo)
            // Cleanup on entering settings to avoid duplicate logos
            .add_systems(OnEnter(GameMenuState::Settings), cleanup_logo);

        debug!("Logo plugin registered - combines Star of David with text");
    }
}

/// Register logo-related components to ensure they're available at startup
fn register_logo_components(mut app: ResMut<App>) {
    app.register_type::<MenuDecorativeElement>();
    debug!("Logo components registered");
}

/// Sets up the combined logo with Star of David and text for the main menu
fn setup_combined_logo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_logos: Query<Entity, With<MenuDecorativeElement>>,
) {
    // First clean up any existing logos to avoid duplication
    for entity in existing_logos.iter() {
        commands.entity(entity).despawn_recursive();
        debug!("Cleaned up existing logo entity");
    }

    info!("Setting up combined logo with Star of David and text for main menu");

    // Find the menu camera to attach to
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching logo to menu camera: {:?}", camera_entity);

        // Attach logo to camera entity with explicit positioning
        commands.entity(camera_entity).with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        position_type: PositionType::Absolute,
                        top: Val::Px(20.0),
                        ..default()
                    },
                    MenuDecorativeElement,
                    MenuItem,
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
                    InheritedVisibility::VISIBLE,
                    ZIndex::from(ZLayers::MenuButtons),
                    Name::new("Main Menu Logo Container"),
                ))
                .with_children(|logo_parent| {
                    // Spawn the Star of David as part of the logo
                    logo_parent.spawn((create_star_of_david(), Name::new("Star of David")));

                    // Add Hebrew text
                    logo_parent.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Hebrew Logo Text"),
                    ));

                    // Add English text
                    logo_parent.spawn((
                        create_english_text(&asset_server),
                        Name::new("English Logo Text"),
                    ));
                });
        });
    } else {
        warn!("No menu camera found, cannot attach logo!");
    }
}

/// Cleans up the logo entities
fn cleanup_logo(
    mut commands: Commands,
    logos: Query<Entity, With<MenuDecorativeElement>>,
    current_state: Res<State<GameMenuState>>,
) {
    let count = logos.iter().count();
    if count > 0 {
        info!(
            "Cleaning up {} logo entities from state: {:?}",
            count,
            current_state.get()
        );

        for entity in logos.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Sets up the pause menu logo
fn setup_pause_logo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_logos: Query<Entity, With<MenuDecorativeElement>>,
) {
    // First clean up any existing logos to avoid duplication
    for entity in existing_logos.iter() {
        commands.entity(entity).despawn_recursive();
        debug!("Cleaned up existing logo entity during pause menu setup");
    }

    info!("Setting up logo for pause menu");

    // Find the menu camera for attachment
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching pause logo to camera: {:?}", camera_entity);

        // Attach to camera entity
        commands.entity(camera_entity).with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        position_type: PositionType::Absolute,
                        top: Val::Px(20.0),
                        ..default()
                    },
                    MenuDecorativeElement,
                    MenuItem,
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
                    InheritedVisibility::VISIBLE,
                    ZIndex::from(ZLayers::MenuButtons),
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
