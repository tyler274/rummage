use crate::camera::components::AppLayer;
use crate::menu::camera::MenuCamera;
use crate::menu::components::{MenuItem, ZLayers};
use crate::menu::decorations::MenuDecorativeElement;
use crate::menu::logo::text::{create_english_text, create_hebrew_text};
use crate::menu::settings::components::OnMainSettingsMenu;
use crate::menu::settings::systems::despawn_screen;
use crate::menu::star_of_david::create_star_of_david;
use crate::menu::state::{AppState, GameMenuState, StateTransitionContext};
use bevy::prelude::*;

/// Resource to track logo initialization attempts
#[derive(Resource, Default)]
struct LogoInitTracker {
    /// Timer for delayed attempts
    timer: Option<Timer>,
    /// Number of attempts made
    attempts: u32,
}

/// Component to mark the logo that should persist across settings transitions
#[derive(Component)]
struct PersistentLogo;

/// Plugin for menu logo
pub struct LogoPlugin;

impl Plugin for LogoPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resource to track logo initialization
            .init_resource::<LogoInitTracker>()
            // Add a startup system to ensure the logo is created before states are processed
            .add_systems(Startup, setup_startup_logo)
            // Add the logo setup, but now only on entering the main menu, not on startup
            .add_systems(OnEnter(GameMenuState::MainMenu), setup_combined_logo)
            .add_systems(OnEnter(GameMenuState::PauseMenu), setup_pause_logo)
            // Add a system to ensure the logo exists when in MainMenu
            .add_systems(
                Update,
                ensure_logo_exists.run_if(in_state(GameMenuState::MainMenu)),
            )
            // Only hide logo when entering settings, rather than cleaning it up completely
            .add_systems(OnEnter(GameMenuState::Settings), hide_logo_for_settings)
            // Restore logo visibility when returning from settings, AFTER the settings screen is despawned
            .add_systems(
                OnExit(GameMenuState::Settings),
                restore_logo_visibility.after(despawn_screen::<OnMainSettingsMenu>),
            )
            // Cleanup only on major transitions
            .add_systems(OnExit(GameMenuState::MainMenu), cleanup_non_persistent_logo)
            .add_systems(
                OnExit(GameMenuState::PauseMenu),
                cleanup_non_persistent_logo,
            )
            // Add general cleanup when exiting the overall Menu AppState
            .add_systems(OnExit(AppState::Menu), cleanup_all_logos);

        debug!("Logo plugin registered - combines Star of David with text");
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
        // If there are no menu cameras, create one first
        if menu_cameras.iter().count() == 0 {
            info!("No menu camera found - creating one before logo setup");
            let camera_entity = commands
                .spawn((
                    Camera2d,
                    Camera {
                        order: 100, // Use a much higher order to avoid conflicts with default cameras
                        ..default()
                    },
                    MenuCamera,
                    Name::new("Startup Menu Camera"),
                    // Add essential UI components to make it a valid UI parent
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    // Full visibility components to ensure UI items inherit visibility properly
                    ViewVisibility::default(),
                    InheritedVisibility::VISIBLE,
                    Visibility::Visible,
                    // Standard ZIndex
                    ZIndex::default(),
                    // Add render layers for menu items
                    crate::camera::components::AppLayer::menu_layers(),
                ))
                .id();

            info!(
                "Created startup menu camera entity: {:?} with order 100",
                camera_entity
            );

            // Now add the logo to the camera we just created
            create_logo_on_camera(commands, asset_server, camera_entity, "Startup");
            info!("Logo attached to startup camera");
        } else {
            // Use existing camera
            info!("Using existing menu camera for startup logo");
            if let Some(camera_entity) = menu_cameras.iter().next() {
                create_logo_on_camera(commands, asset_server, camera_entity, "Startup");
            }
        }
    } else {
        info!("Logo already exists at startup");
    }
}

/// System to ensure the logo exists when in the MainMenu state
fn ensure_logo_exists(
    commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_logos: Query<Entity, With<MenuDecorativeElement>>,
    time: Res<Time>,
    mut init_tracker: ResMut<LogoInitTracker>,
) {
    // If we already have a logo, reset the tracker and return
    if existing_logos.iter().count() > 0 {
        init_tracker.timer = None;
        init_tracker.attempts = 0;
        return;
    }

    // If there's no logo but we have a menu camera, try to create it with a timer
    if menu_cameras.iter().count() > 0 {
        // Initialize timer if not already set
        if init_tracker.timer.is_none() {
            init_tracker.timer = Some(Timer::from_seconds(0.2, TimerMode::Repeating));
            info!("Starting logo initialization timer");
        }

        // Tick the timer
        if let Some(ref mut timer) = init_tracker.timer {
            timer.tick(time.delta());

            // Try to initialize on timer completion
            if timer.just_finished() {
                init_tracker.attempts += 1;
                info!(
                    "Attempting logo initialization (attempt {})",
                    init_tracker.attempts
                );

                if let Some(camera_entity) = menu_cameras.iter().next() {
                    create_logo_on_camera(commands, asset_server, camera_entity, "Timer");
                }

                // After 5 attempts, stop trying
                if init_tracker.attempts >= 5 {
                    warn!("Giving up on logo initialization after 5 attempts");
                    init_tracker.timer = None;
                }
            }
        }
    }
}

/// Sets up the combined logo with Star of David and text for the main menu
fn setup_combined_logo(
    commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_logos: Query<Entity, With<MenuDecorativeElement>>,
) {
    // If we already have logos, don't create more
    if existing_logos.iter().count() > 0 {
        info!("Logos already exist, not creating new ones");
        return;
    }

    info!("Setting up combined logo with Star of David and text for main menu");

    // Find the menu camera to attach to
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching logo to menu camera: {:?}", camera_entity);
        create_logo_on_camera(commands, asset_server, camera_entity, "Main Menu");
    } else {
        warn!("No menu camera found, cannot attach logo!");
    }
}

/// Creates a logo on the specified camera entity
fn create_logo_on_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_entity: Entity,
    prefix: &str,
) {
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
                PersistentLogo, // Mark as persistent
                MenuItem,
                AppLayer::Menu.layer(),
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                ZIndex::from(ZLayers::MenuButtons),
                Name::new(format!("{} Logo Container", prefix)),
            ))
            .with_children(|logo_parent| {
                // Spawn the Star of David as part of the logo
                logo_parent.spawn((
                    create_star_of_david(),
                    Name::new(format!("{} Star of David", prefix)),
                ));

                // Add Hebrew text
                logo_parent.spawn((
                    create_hebrew_text(&asset_server),
                    Name::new(format!("{} Hebrew Logo Text", prefix)),
                ));

                // Add English text
                logo_parent.spawn((
                    create_english_text(&asset_server),
                    Name::new(format!("{} English Logo Text", prefix)),
                ));
            });
    });
}

/// Hide logo when entering settings instead of destroying it
fn hide_logo_for_settings(
    mut logos: Query<&mut Visibility, With<MenuDecorativeElement>>,
    mut transition_context: ResMut<StateTransitionContext>,
    _current_state: Res<State<GameMenuState>>,
) {
    // Store info about what state we're coming from - REMOVED: This should be handled by handle_settings_enter
    // transition_context.settings_origin = Some(*current_state.get());
    transition_context.returning_from_settings = false;

    info!("Hiding logo while entering settings");
    for mut visibility in logos.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

/// Restore logo visibility when returning from settings
fn restore_logo_visibility(
    mut logos: Query<&mut Visibility, With<MenuDecorativeElement>>,
    mut transition_context: ResMut<StateTransitionContext>,
) {
    // Mark that we're returning from settings
    transition_context.returning_from_settings = true;

    // Restore visibility
    let count = logos.iter().count();
    if count > 0 {
        info!(
            "Restoring visibility of {} logos when exiting settings",
            count
        );

        for mut visibility in logos.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

/// Cleans up only non-persistent logo entities
fn cleanup_non_persistent_logo(
    mut commands: Commands,
    logos: Query<(Entity, Option<&PersistentLogo>), With<MenuDecorativeElement>>,
    current_state: Res<State<GameMenuState>>,
) {
    let mut count = 0;

    for (entity, persistent) in logos.iter() {
        // Only clean up non-persistent logos
        if persistent.is_none() {
            commands.entity(entity).despawn_recursive();
            count += 1;
        }
    }

    if count > 0 {
        info!(
            "Cleaned up {} non-persistent logo entities from state: {:?}",
            count,
            current_state.get()
        );
    }
}

/// Cleans up ALL logo entities, regardless of persistence
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
    commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    existing_logos: Query<Entity, With<MenuDecorativeElement>>,
) {
    // If we already have logos, don't create more
    if existing_logos.iter().count() > 0 {
        info!("Logos already exist for pause menu, not creating new ones");
        return;
    }

    info!("Setting up logo for pause menu");

    // Find the menu camera for attachment
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!("Attaching pause logo to camera: {:?}", camera_entity);
        create_logo_on_camera(commands, asset_server, camera_entity, "Pause Menu");
    } else {
        warn!("No menu camera found, cannot attach pause logo!");
    }
}
