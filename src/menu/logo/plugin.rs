use crate::{
    camera::components::AppLayer,
    menu::{
        components::{MenuItem, ZLayers},
        decorations::MenuDecorativeElement,
        logo::{create_english_text, create_hebrew_text},
        main_menu::plugin::MainMenuSetupSet,
        star_of_david::create_star_of_david,
        state::{AppState, GameMenuState},
    },
};
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

/// SystemSet for logo setup logic to ensure ordering
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LogoSetupSet {
    Setup,
}

/// Plugin for menu logo
pub struct LogoPlugin;

impl Plugin for LogoPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resource to track logo initialization
            .init_resource::<LogoInitTracker>()
            // Configure the LogoSetupSet to run after the menu camera setup
            // Note: We need to import setup_menu_camera or refer to it correctly
            // Assuming setup_menu_camera is accessible or we use a marker set
            .configure_sets(
                OnEnter(GameMenuState::PauseMenu),
                LogoSetupSet::Setup.after(crate::menu::camera::setup::setup_menu_camera),
            )
            // Add a startup system to ensure the logo is created before states are processed
            .add_systems(Startup, setup_startup_logo)
            // Setup pause logo on enter, now in its own set
            .add_systems(
                OnEnter(GameMenuState::PauseMenu),
                setup_pause_logo.in_set(LogoSetupSet::Setup),
            )
            // Add logo setup to PostUpdate schedule, running once when entering MainMenu
            .add_systems(
                PostUpdate,
                setup_combined_logo
                    .run_if(in_state(GameMenuState::MainMenu))
                    // Run only if the marker resource doesn't exist yet for this state instance
                    .run_if(not(resource_exists::<MainMenuLogoSpawned>))
                    .after(MainMenuSetupSet),
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
    menu_cameras: Query<Entity, With<crate::camera::components::MenuCamera>>,
    existing_logos: Query<Entity, With<MenuDecorativeElement>>,
) {
    info!("Running startup logo setup");

    // If no logos exist yet and a menu camera already exists
    if existing_logos.is_empty() {
        if let Some(camera_entity) = menu_cameras.iter().next() {
            info!(
                "Attaching startup logo to existing camera: {:?}",
                camera_entity
            );
            create_logo_on_camera(&mut commands, asset_server, camera_entity, "Startup");
        } else {
            // Don't create a camera here, let the specific menu state handle it.
            info!("No menu camera found at startup, logo not created yet.");
        }
    } else {
        info!("Logo already exists at startup");
    }
}

/// Sets up the combined logo - now runs in PostUpdate, once per MainMenu entry.
fn setup_combined_logo(
    mut commands: Commands, // Now needs mut
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<crate::camera::components::MenuCamera>>,
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
        commands.entity(entity).despawn();
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
        commands.entity(entity).despawn();
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
        commands.entity(entity).despawn();
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
    menu_cameras: Query<Entity, With<crate::camera::components::MenuCamera>>,
) {
    info!("Setting up logo for pause menu");

    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching pause logo to camera: {:?}", camera_entity);
        create_logo_on_camera(&mut commands, asset_server, camera_entity, "Pause Menu");
    } else {
        warn!("No menu camera found, cannot attach pause logo!");
    }
}
