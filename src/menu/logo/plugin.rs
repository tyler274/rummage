use crate::camera::components::AppLayer;
use crate::menu::camera::MenuCamera;
use crate::menu::components::{MenuItem, ZLayers};
use crate::menu::decorations::MenuDecorativeElement;
use crate::menu::logo::text::{create_english_text, create_hebrew_text};
use crate::menu::main_menu::systems::setup::setup_main_menu;
use crate::menu::star_of_david::create_star_of_david;
use crate::menu::state::{AppState, GameMenuState};
use bevy::prelude::*;

/// Marker resource to track if the main menu logo has been spawned for the current state instance
#[derive(Resource)]
struct MainMenuLogoSpawned;

/// Resource to track logo initialization attempts
#[derive(Resource, Default)]
struct LogoInitTracker {
    /// Timer for delayed attempts
    timer: Option<Timer>,
    /// Number of attempts made
    attempts: u32,
}

/// Plugin for menu logo
pub struct LogoPlugin;

impl Plugin for LogoPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resource to track logo initialization
            .init_resource::<LogoInitTracker>()
            // Add a startup system to ensure the logo is created before states are processed
            .add_systems(Startup, setup_startup_logo)
            // Setup pause logo on enter
            .add_systems(OnEnter(GameMenuState::PauseMenu), setup_pause_logo)
            // Add logo setup to PostUpdate schedule, running once when entering MainMenu
            .add_systems(
                PostUpdate,
                setup_combined_logo
                    .run_if(in_state(GameMenuState::MainMenu))
                    // Run only if the marker resource doesn't exist yet for this state instance
                    .run_if(not(resource_exists::<MainMenuLogoSpawned>))
                    .after(setup_main_menu),
            )
            // Cleanup logo when leaving main menu or pause menu
            .add_systems(
                OnExit(GameMenuState::MainMenu),
                (cleanup_main_menu_logo, remove_main_menu_logo_spawned_flag),
            ) // Add flag cleanup
            .add_systems(
                OnExit(GameMenuState::PauseMenu),
                cleanup_pause_menu_logo, // Use a separate function or the same if logic is identical
            )
            // Add general cleanup when exiting the overall Menu AppState
            .add_systems(OnExit(AppState::Menu), cleanup_all_logos);

        debug!("Logo plugin registered - simplifies logo handling");
    }
}

/// Sets up the logo at application startup
fn setup_startup_logo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_logos: Query<Entity, With<MenuDecorativeElement>>,
) {
    info!("Running startup logo setup");

    // If no logos exist yet
    if existing_logos.iter().count() == 0 {
        // If there are no menu cameras, create one first (This might still be needed if MainMenu setup fails)
        if menu_cameras.iter().count() == 0 {
            info!("No menu camera found - creating one before logo setup");
            let camera_entity = commands
                .spawn((
                    Camera2d,
                    Camera {
                        order: 100, // Use a much higher order
                        ..default()
                    },
                    MenuCamera,
                    Name::new("Startup Menu Camera"),
                    Node {
                        // Add essential UI components
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ViewVisibility::default(),
                    InheritedVisibility::VISIBLE,
                    Visibility::Visible,
                    ZIndex::default(),
                    crate::camera::components::AppLayer::menu_layers(),
                ))
                .id();

            info!(
                "Created startup menu camera entity: {:?} with order 100",
                camera_entity
            );

            create_logo_on_camera(&mut commands, asset_server, camera_entity, "Startup");
            info!("Logo attached to startup camera");
        } else {
            info!("Using existing menu camera for startup logo");
            if let Some(camera_entity) = menu_cameras.iter().next() {
                create_logo_on_camera(&mut commands, asset_server, camera_entity, "Startup");
            }
        }
    } else {
        info!("Logo already exists at startup");
    }
}

/// Sets up the combined logo - now runs in PostUpdate, once per MainMenu entry.
fn setup_combined_logo(
    mut commands: Commands, // Now needs mut
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
) {
    info!("Setting up combined logo via PostUpdate schedule");

    // Find the menu camera to attach to
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching logo to menu camera: {:?}", camera_entity);
        // Pass mut commands now by reference
        create_logo_on_camera(&mut commands, asset_server, camera_entity, "Main Menu");
        // Mark the logo as spawned for this state entry
        commands.insert_resource(MainMenuLogoSpawned);
        info!("Inserted MainMenuLogoSpawned flag");
    } else {
        // This warning might still appear if camera setup takes more than one frame,
        // but the system will try again next frame until it succeeds or state changes.
        warn!("No menu camera found during PostUpdate, will retry next frame if still in MainMenu");
    }
}

/// Creates a logo on the specified camera entity
fn create_logo_on_camera(
    commands: &mut Commands, // Pass by mutable reference
    asset_server: Res<AssetServer>,
    camera_entity: Entity,
    prefix: &str,
) {
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
                Visibility::Visible, // Assume visible by default
                InheritedVisibility::VISIBLE,
                ZIndex::from(ZLayers::MenuButtons),
                Name::new(format!("{} Logo Container", prefix)),
            ))
            .with_children(|logo_parent| {
                logo_parent.spawn((
                    create_star_of_david(),
                    Name::new(format!("{} Star of David", prefix)),
                ));
                logo_parent.spawn((
                    create_hebrew_text(&asset_server),
                    Name::new(format!("{} Hebrew Logo Text", prefix)),
                ));
                logo_parent.spawn((
                    create_english_text(&asset_server),
                    Name::new(format!("{} English Logo Text", prefix)),
                ));
            });
    });
}

/// Cleans up logo entities when leaving the main menu state
fn cleanup_main_menu_logo(
    mut commands: Commands,
    logos: Query<Entity, With<MenuDecorativeElement>>, // Simplified query
) {
    let mut count = 0;
    for entity in logos.iter() {
        // Always clean up logos when leaving main menu
        commands.entity(entity).despawn_recursive();
        count += 1;
    }

    if count > 0 {
        info!("Cleaned up {} logo entities on exiting MainMenu", count);
    }
}

/// Cleans up logo entities when leaving the pause menu state
// (Assuming identical logic to main menu cleanup for now)
fn cleanup_pause_menu_logo(
    mut commands: Commands,
    logos: Query<Entity, With<MenuDecorativeElement>>,
) {
    let mut count = 0;
    for entity in logos.iter() {
        commands.entity(entity).despawn_recursive();
        count += 1;
    }

    if count > 0 {
        info!("Cleaned up {} logo entities on exiting PauseMenu", count);
    }
}

/// Removes the marker resource when leaving the main menu state
fn remove_main_menu_logo_spawned_flag(mut commands: Commands) {
    commands.remove_resource::<MainMenuLogoSpawned>();
    info!("Removed MainMenuLogoSpawned flag");
}

/// Cleans up ALL logo entities when exiting the Menu AppState
fn cleanup_all_logos(mut commands: Commands, logos: Query<Entity, With<MenuDecorativeElement>>) {
    let mut count = 0;
    for entity in logos.iter() {
        commands.entity(entity).despawn_recursive();
        count += 1;
    }
    if count > 0 {
        info!(
            "Cleaned up {} logo entities on exiting AppState::Menu",
            count
        );
    }
}

/// Sets up the pause menu logo
fn setup_pause_logo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
) {
    info!("Setting up logo for pause menu");

    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching pause logo to camera: {:?}", camera_entity);
        create_logo_on_camera(&mut commands, asset_server, camera_entity, "Pause Menu");
    } else {
        warn!("No menu camera found, cannot attach pause logo!");
    }
}
