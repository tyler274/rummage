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
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

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
                    setup_game,
                    // Ensure zoom is set before spawning hands
                    set_initial_zoom
                        .run_if(|context: Res<crate::menu::state::StateTransitionContext>| {
                            !context.from_pause_menu
                        })
                        .after(setup_game),
                    // Spawn hands after setup AND after initial zoom is set
                    spawn_player_visual_hands
                        .after(setup_game)
                        .after(set_initial_zoom),
                    // Snapshot system is controlled by feature flag
                    #[cfg(feature = "snapshot")]
                    take_snapshot_after_setup
                        .run_if(|context: Res<crate::menu::state::StateTransitionContext>| {
                            !context.from_pause_menu
                        })
                        .after(setup_game),
                ),
            )
            .add_systems(
                Update,
                (handle_window_resize, camera_movement).run_if(in_state(AppState::InGame)),
            );
    }
}

// System to set up the game state
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_config: Res<PlayerConfig>,
    existing_game_cameras: Query<Entity, With<GameCamera>>,
) {
    info!("Setting up game state...");

    // --- Game Camera Setup (moved from camera/systems.rs) ---
    // Check if a game camera already exists
    if existing_game_cameras.is_empty() {
        info!("Spawning Game Camera...");
        let camera_entity = commands
            .spawn((
                Camera2d,
                Camera {
                    order: 0, // Explicitly set order to 0 for game camera
                    ..default()
                },
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
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
            .id();
        info!("Game camera spawned with entity {:?}", camera_entity);
        // Initialize camera pan state here as well if needed, or ensure it's done elsewhere
        // commands.insert_resource(CameraPanState::default()); // Pan state is initialized in plugin
    } else {
        info!(
            "Game camera already exists ({:?}), not spawning a new one.",
            existing_game_cameras.iter().collect::<Vec<_>>()
        );
        // Optionally, ensure existing camera has correct settings (layers, transform etc.)
    }
    // --- End Game Camera Setup ---

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
