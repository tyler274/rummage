mod game_plugin;

use bevy::prelude::*;

// Import what we need
use crate::camera::components::GameCamera;
use crate::cards::CardZone;
use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::zones::{Zone, ZoneManager};
use crate::menu::GameMenuState;
use crate::player::{PlayerPlugin, components::Player, resources::PlayerConfig, spawn_players};

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
                    .with_card_size(Vec2::new(67.2, 93.6)) // Reduce card size to 1/10th original size
                    .with_player_card_distance(400.0)
                    .with_player_card_offset(0, -200.0) // Bottom player
                    .with_player_card_offset(1, 0.0) // Right player
                    .with_player_card_offset(2, 200.0) // Top player
                    .with_player_card_offset(3, 0.0), // Left player
            )
            // Initialize zone manager resource
            .init_resource::<ZoneManager>()
            // Add game setup system
            .add_systems(OnEnter(GameMenuState::InGame), setup_game)
            // System to connect cards to zones after they're spawned
            .add_systems(Update, connect_cards_to_zones)
            // Add a system to ensure game camera is visible in InGame state
            .add_systems(OnEnter(GameMenuState::InGame), ensure_game_camera_visible)
            // Add a diagnostic system to check card status
            .add_systems(Update, check_card_status);
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

        let card_count = card_query.iter().count();
        info!("Found {} cards to connect to zones", card_count);

        if card_count == 0 {
            warn!("No cards found to connect to zones - cards may not be visible");
        }

        // Process each card and add it to the appropriate zone in ZoneManager
        for (card_entity, card_zone) in card_query.iter() {
            match card_zone.zone {
                Zone::Hand => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_hand(owner, card_entity);
                        info!("Added card {:?} to player {:?}'s hand", card_entity, owner);
                    } else {
                        warn!("Card {:?} has no owner, cannot add to hand", card_entity);
                    }
                }
                Zone::Library => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_library(owner, card_entity);
                        info!(
                            "Added card {:?} to player {:?}'s library",
                            card_entity, owner
                        );
                    } else {
                        warn!("Card {:?} has no owner, cannot add to library", card_entity);
                    }
                }
                Zone::Battlefield => {
                    let owner = card_zone.zone_owner.unwrap_or(Entity::PLACEHOLDER);
                    zone_manager.add_to_battlefield(owner, card_entity);
                    info!(
                        "Added card {:?} to battlefield with owner {:?}",
                        card_entity, owner
                    );
                }
                Zone::Graveyard => {
                    if let Some(owner) = card_zone.zone_owner {
                        zone_manager.add_to_graveyard(owner, card_entity);
                        info!(
                            "Added card {:?} to player {:?}'s graveyard",
                            card_entity, owner
                        );
                    } else {
                        warn!(
                            "Card {:?} has no owner, cannot add to graveyard",
                            card_entity
                        );
                    }
                }
                _ => {
                    warn!(
                        "Card {:?} has unknown zone {:?}",
                        card_entity, card_zone.zone
                    );
                }
            }
        }

        // Remove the one-time event
        commands.entity(entity).despawn();
        info!("Card connection complete");
    }
}

// Ensure game camera is visible when entering game state
fn ensure_game_camera_visible(mut game_camera_query: Query<&mut Visibility, With<GameCamera>>) {
    for mut visibility in game_camera_query.iter_mut() {
        *visibility = Visibility::Visible;
        info!("Ensuring game camera is visible for card rendering");
    }
}

// Diagnostic system to check card status
fn check_card_status(
    cards: Query<(Entity, &Transform, &Visibility), With<crate::cards::Card>>,
    player_query: Query<(Entity, &Player)>,
    game_camera_query: Query<Entity, With<GameCamera>>,
    zone_manager: Res<ZoneManager>,
    mut has_run: Local<bool>,
) {
    // Only run once
    if *has_run {
        return;
    }

    // Wait a few frames before checking
    static mut FRAME_COUNT: u32 = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT < 30 {
            return;
        }
    }

    *has_run = true;

    if cards.is_empty() {
        error!("No cards found! Cards are not being spawned properly.");
        return;
    }

    info!("Found {} cards in the world", cards.iter().count());

    // Check if cards are registered in the zone manager
    let mut cards_with_zones = 0;
    let mut cards_without_zones = 0;

    for (entity, transform, visibility) in cards.iter() {
        let is_visible = match visibility {
            Visibility::Visible => "visible",
            Visibility::Hidden => "hidden",
            Visibility::Inherited => "inherited",
        };

        // Check if the card is registered in any zone
        let zone = zone_manager.get_card_zone(entity);

        if let Some(zone) = zone {
            cards_with_zones += 1;
            info!(
                "Card {:?} at position {:?} is {} and in zone {:?}",
                entity, transform.translation, is_visible, zone
            );
        } else {
            cards_without_zones += 1;
            warn!(
                "Card {:?} at position {:?} is {} but not registered in any zone!",
                entity, transform.translation, is_visible
            );
        }
    }

    info!(
        "Zone registration status: {} cards in zones, {} cards without zones",
        cards_with_zones, cards_without_zones
    );

    // Check player entities
    if player_query.is_empty() {
        error!("No player entities found!");
    } else {
        info!("Found {} player entities", player_query.iter().count());
        for (entity, player) in player_query.iter() {
            info!(
                "Player {:?} with name '{}' at index {}",
                entity, player.name, player.player_index
            );

            // Check cards in player's hand
            if let Some(hand) = zone_manager.get_player_zone(entity, Zone::Hand) {
                info!("Player {} has {} cards in hand", player.name, hand.len());
            } else {
                warn!("Player {} has no hand zone registered!", player.name);
            }
        }
    }

    // Check game camera
    if game_camera_query.is_empty() {
        error!("No game camera found!");
    } else {
        info!("Game camera entity: {:?}", game_camera_query.single());
    }

    // Print some cards for debugging
    for (i, (entity, transform, visibility)) in cards.iter().enumerate().take(3) {
        let is_visible = match visibility {
            Visibility::Visible => "visible",
            Visibility::Hidden => "hidden",
            Visibility::Inherited => "inherited",
        };

        info!(
            "Card {}: Entity {:?} at position {:?} is {}",
            i, entity, transform.translation, is_visible
        );
    }
}
