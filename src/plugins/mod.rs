mod game_plugin;

use bevy::prelude::*;

// Import what we need
use crate::camera::components::GameCamera;
use crate::cards::CardZone;
use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::zones::{Zone, ZoneManager};
use crate::menu::GameMenuState;
use crate::player::{PlayerPlugin, resources::PlayerConfig, spawn_players};

pub struct MainRummagePlugin;

impl Plugin for MainRummagePlugin {
    fn build(&self, app: &mut App) {
        // Add Player Plugin
        app.add_plugins(PlayerPlugin)
            // Add Save/Load system
            .add_plugins(SaveLoadPlugin)
            // Setup game configuration
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
            // Initialize zone manager resource
            .init_resource::<ZoneManager>()
            // Add game setup system
            .add_systems(OnEnter(GameMenuState::InGame), setup_game)
            // System to connect cards to zones after they're spawned
            .add_systems(Update, connect_cards_to_zones)
            // Add a system to ensure game camera is visible in InGame state
            .add_systems(OnEnter(GameMenuState::InGame), ensure_game_camera_visible);
    }
}

// Game setup system that spawns players
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    context: Res<crate::menu::state::StateTransitionContext>,
    player_config: Res<PlayerConfig>,
    _zone_manager: ResMut<ZoneManager>,
) {
    // Skip setup if we're coming from pause menu
    if context.from_pause_menu {
        info!("Resuming from pause menu, skipping game setup");
        return;
    }

    // Normal game setup - this is a fresh game
    info!("Spawning players...");

    // Spawn the players (passing commands by reference)
    spawn_players(
        &mut commands,
        asset_server,
        game_cameras,
        Some(player_config),
    );

    info!("Game setup complete!");

    // Post-setup connection of cards to zones
    // Add system to connect spawned cards to hand zones in the next frame
    commands.spawn((
        Name::new("Card-to-Zone Connection System"),
        InitializeCardsEvent,
    ));
}

// One-time event to connect cards to zones after they're spawned
#[derive(Component)]
pub struct InitializeCardsEvent;

// Additional systems that run after game setup
pub fn connect_cards_to_zones(
    mut commands: Commands,
    query: Query<(Entity, &InitializeCardsEvent)>,
    card_query: Query<(Entity, &CardZone)>,
    mut zone_manager: ResMut<ZoneManager>,
) {
    for (entity, _) in query.iter() {
        info!("Connecting cards to zones...");

        // Process each card and add it to the appropriate zone in ZoneManager
        for (card_entity, card_zone) in card_query.iter() {
            match card_zone.zone {
                Zone::Hand => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_hand(owner, card_entity);
                        info!("Added card {:?} to player {:?}'s hand", card_entity, owner);
                    }
                }
                Zone::Library => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_library(owner, card_entity);
                    }
                }
                Zone::Battlefield => {
                    zone_manager.add_to_battlefield(
                        card_zone.zone_owner.unwrap_or(Entity::PLACEHOLDER),
                        card_entity,
                    );
                }
                Zone::Graveyard => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_graveyard(owner, card_entity);
                    }
                }
                _ => {}
            }
        }

        // Remove the one-time event
        commands.entity(entity).despawn();
    }
}

// Ensure game camera is visible when entering game state
fn ensure_game_camera_visible(mut game_camera_query: Query<&mut Visibility, With<GameCamera>>) {
    for mut visibility in game_camera_query.iter_mut() {
        *visibility = Visibility::Visible;
        info!("Ensuring game camera is visible for card rendering");
    }
}
