use crate::camera::components::{AppLayer, GameCamera};
use crate::camera::{
    CameraPanState,
    config::CameraConfig,
    systems::{camera_movement, handle_window_resize, set_initial_zoom},
};
use crate::deck::{PlayerDeck, get_player_shuffled_deck};
use crate::player::components::Player;
use crate::player::playmat::spawn_player_playmat;
use crate::player::systems::spawn::cards;
use crate::player::systems::spawn::table::TableLayout;
use crate::player::{PlayerPlugin, resources::PlayerConfig};
#[cfg(feature = "snapshot")]
use crate::snapshot::systems::take_snapshot_after_setup;
use crate::text::DebugConfig;
use bevy::ecs::hierarchy::ChildOf;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::collections::HashSet;

// Add AppState import
use crate::menu::state::AppState;

/// Marker component to trigger visual hand spawning for a player
#[derive(Component)]
struct SpawnVisualHand {
    player_entity: Entity,
    deck: PlayerDeck,
    position: Vec3,
}

// Plugin for the actual game systems
pub struct RummagePlugin;

impl Plugin for RummagePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::cards::drag::DragPlugin)
            .add_plugins(crate::cards::CardPlugin)
            .add_plugins(crate::deck::DeckPlugin)
            .add_plugins(crate::game_engine::GameEnginePlugin)
            .add_plugins(crate::text::TextPlugin::default())
            .add_plugins(PlayerPlugin)
            .insert_resource(DebugConfig {
                show_text_positions: true,
            })
            .insert_resource(CameraConfig::default())
            .insert_resource(CameraPanState::default())
            .insert_resource(
                PlayerConfig::new()
                    .with_player_count(4)
                    .with_spawn_all_cards(true)
                    .with_starting_life(40)
                    .with_player_card_distance(400.0)
                    .with_player_card_offset(0, -1200.0) // Bottom player
                    .with_player_card_offset(1, 0.0) // Right player
                    .with_player_card_offset(2, 1200.0) // Top player
                    .with_player_card_offset(3, 0.0), // Left player
            )
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    // Chain systems with apply_deferred to ensure command application
                    spawn_game_camera,
                    ApplyDeferred, // Apply camera spawn command
                    setup_game,    // Now safe to query for camera if needed
                    ApplyDeferred, // Apply game setup commands
                    spawn_player_visual_hands // Hands depend on setup
                        .after(setup_game), // Only need after setup_game now
                    // Snapshot system (conditional)
                    #[cfg(feature = "snapshot")]
                    (
                        ApplyDeferred, // Ensure previous commands applied before snapshot
                        take_snapshot_after_setup
                            .run_if(|context: Res<crate::menu::state::StateTransitionContext>| {
                                !context.from_pause_menu
                            })
                            .after(setup_game), // Snapshot depends on game setup
                    )
                        .chain(), // Chain snapshot logic
                )
                    .chain(), // Chain the core setup sequence
            )
            .add_systems(
                Update,
                (
                    set_initial_zoom.run_if(in_state(AppState::InGame)),
                    handle_window_resize.run_if(in_state(AppState::InGame)),
                    camera_movement.run_if(in_state(AppState::InGame)),
                ),
            )
            // Re-add the debug logging system
            .add_systems(
                Update,
                debug_log_camera_state.run_if(in_state(AppState::InGame)),
            );
    }
}

// --- System to Log Camera State ---
fn debug_log_camera_state(
    // Use Added<T> filter to log only when camera first appears or changes significantly
    camera_query: Query<
        (
            Entity,
            &Transform,
            &Visibility,
            &InheritedVisibility,
            &ViewVisibility,
            &Camera,
            &RenderLayers,
        ),
        (
            With<GameCamera>,
            Or<(Added<GameCamera>, Changed<Transform>, Changed<Visibility>)>,
        ),
    >,
    // Use Local to log only once per camera entity initially, then on changes
    mut logged_cameras: Local<HashSet<Entity>>,
) {
    for (entity, transform, visibility, inherited_visibility, view_visibility, camera, layers) in
        camera_query.iter()
    {
        if logged_cameras.insert(entity) {
            info!(
                "[CAMERA DEBUG] Initial State - Entity: {:?}, Order: {}, Transform: {:?}, Vis: {:?}, InheritedVis: {:?}, ViewVis: {:?}, Layers: {:?}",
                entity,
                camera.order,
                transform,
                visibility,
                inherited_visibility,
                view_visibility,
                layers
            );
        } else {
            // Log subsequent changes
            info!(
                "[CAMERA DEBUG] Changed State - Entity: {:?}, Order: {}, Transform: {:?}, Vis: {:?}, InheritedVis: {:?}, ViewVis: {:?}, Layers: {:?}",
                entity,
                camera.order,
                transform,
                visibility,
                inherited_visibility,
                view_visibility,
                layers
            );
        }
    }
}
// --- End System ---

// --- System to Spawn Game Camera ---
fn spawn_game_camera(
    mut commands: Commands,
    existing_game_cameras: Query<Entity, With<GameCamera>>,
) {
    info!("Checking for existing game cameras...");
    if existing_game_cameras.is_empty() {
        info!("Spawning Game Camera...");
        let camera_entity = commands
            .spawn((
                Camera2d::default(), // Use Camera2d component directly
                Projection::Orthographic(OrthographicProjection::default_2d()), // Add Projection component
                Camera {
                    order: 0, // Explicitly set order to 0 for game camera
                    ..default()
                },
                // Ensure Visibility components are explicitly added for clarity
                Visibility::Visible,
                InheritedVisibility::default(), // Should become true if not parented
                ViewVisibility::default(), // Should become true if Vis is Visible & Inherited is true
                Transform::from_xyz(0.0, 0.0, 999.0),
                GlobalTransform::default(),
                GameCamera,
                // Ensure camera renders game layers
                RenderLayers::from_layers(&[
                    AppLayer::Game.as_usize(),
                    AppLayer::Cards.as_usize(),
                    AppLayer::GameWorld.as_usize(),
                    AppLayer::Background.as_usize(),
                    AppLayer::GameUI.as_usize(),
                    AppLayer::Shared.as_usize(),
                ]),
                Name::new("Game Camera"),
            ))
            // Explicitly remove Parent component to prevent accidental parenting
            .remove::<ChildOf>()
            .id();
        info!(
            "Successfully spawned GameCamera with entity: {:?}",
            camera_entity
        );
    } else {
        info!(
            "Game camera already exists ({:?}), not spawning a new one.",
            existing_game_cameras.iter().collect::<Vec<_>>()
        );
    }
}
// --- End System ---

// System to set up the game state (now without camera spawning)
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_config: Res<PlayerConfig>,
) {
    info!(
        "Setting up game state (players, playmats)... N={}",
        player_config.player_count
    );

    let config = player_config.clone();
    info!("Spawning {} players...", config.player_count);
    let playmat_size = Vec2::new(430.0, 330.0);
    let table = TableLayout::new(config.player_count, config.player_card_distance)
        .with_playmat_size(playmat_size);

    for player_index in 0..config.player_count {
        let position_name = table.get_position_name(player_index);
        let player = Player::new(&format!("Player {} ({})", player_index + 1, position_name))
            .with_life(config.starting_life)
            .with_player_index(player_index);
        let player_transform = table.get_player_position(player_index);
        let player_entity = commands
            .spawn((
                player.clone(),
                player_transform,
                GlobalTransform::default(),
                AppLayer::game_layers(),
            ))
            .id();

        spawn_player_playmat(
            &mut commands,
            &asset_server,
            player_entity,
            &player,
            &config,
            player_transform.translation,
        );

        let deck = get_player_shuffled_deck(
            player_entity,
            player_index,
            Some(&format!("Player {} Deck", player_index + 1)),
        );
        commands
            .entity(player_entity)
            .insert(PlayerDeck::new(deck.clone()));

        // If cards should be spawned, add marker component
        if player_index == 0 || config.spawn_all_cards {
            commands.spawn(SpawnVisualHand {
                player_entity,
                deck: PlayerDeck::new(deck),
                position: player_transform.translation,
            });
        }
    }
    info!("Player setup complete, markers added for visual hands.");
}

// System to handle spawning visual hands based on the marker
fn spawn_player_visual_hands(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    player_query: Query<&Player>,
    player_config: Res<PlayerConfig>,
    marker_query: Query<(Entity, &SpawnVisualHand)>,
) {
    if marker_query.is_empty() {
        return;
    }

    let config = player_config.clone();
    let table = TableLayout::new(config.player_count, config.player_card_distance);

    if game_cameras.is_empty() {
        error!("No game camera found, cannot spawn visual cards.");
        for (marker_entity, _) in marker_query.iter() {
            commands.entity(marker_entity).despawn();
        }
        return;
    }

    for (marker_entity, marker) in marker_query.iter() {
        info!("Spawning visual hand for player {:?}", marker.player_entity);

        let mut deck_copy = marker.deck.deck.clone();
        let display_cards = deck_copy.draw_multiple(7);

        if display_cards.is_empty() {
            warn!(
                "Deck for player {:?} was empty, cannot spawn hand.",
                marker.player_entity
            );
            commands.entity(marker_entity).despawn();
            continue;
        }

        // Get Player component to access player_index
        if let Ok(player) = player_query.get(marker.player_entity) {
            let player_index = player.player_index;

            cards::spawn_visual_cards(
                &mut commands,
                &config.card_size,
                config.card_spacing_multiplier,
                player_index,
                marker.player_entity,
                &table,
                Some(&asset_server).map(|v| &**v),
                display_cards,
            );
        } else {
            warn!(
                "Could not find Player component for entity {:?}, skipping hand spawn.",
                marker.player_entity
            );
        }

        commands.entity(marker_entity).despawn();
    }
}
