mod game_plugin;

use bevy::prelude::*;

// Import what we need
use crate::camera::components::GameCamera;
use crate::camera::systems::setup_camera;
use crate::cards::drag::DragPlugin;
use crate::cards::{CardPlugin, CardZone};
use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::zones::{Zone, ZoneManager};
use crate::menu::GameMenuState;
use crate::player::{PlayerPlugin, components::Player, resources::PlayerConfig, spawn_players};
use crate::text::DebugConfig;

pub struct MainRummagePlugin;

impl Plugin for MainRummagePlugin {
    fn build(&self, app: &mut App) {
        // Add Player Plugin
        app.add_plugins(PlayerPlugin)
            // Add Card Plugin for card dragging and other card functionality
            .add_plugins(CardPlugin)
            // Add Drag Plugin for drag and drop functionality
            .add_plugins(DragPlugin)
            // Add Text Plugin for text rendering and debugging
            .add_plugins(crate::text::TextPlugin::default())
            // Initialize debug config resource
            .insert_resource(DebugConfig {
                show_text_positions: false,
            })
            // Add Save/Load system
            .add_plugins(SaveLoadPlugin)
            // Setup game configuration
            .insert_resource(
                PlayerConfig::new()
                    .with_player_count(4)
                    .with_spawn_all_cards(true)
                    .with_starting_life(40)
                    .with_card_size(Vec2::new(67.2, 93.6)) // Reduce card size to 1/10th original size
                    .with_card_spacing_multiplier(0.8) // Add tighter spacing between cards
                    .with_player_card_distance(300.0) // Reduce distance to bring players closer together
                    .with_player_card_offset(0, -100.0) // Bottom player - reduced offset
                    .with_player_card_offset(1, 0.0) // Right player
                    .with_player_card_offset(2, 100.0) // Top player - reduced offset
                    .with_player_card_offset(3, 0.0), // Left player
            )
            // Initialize zone manager resource
            .init_resource::<ZoneManager>()
            // Add game setup system
            .add_systems(
                OnEnter(GameMenuState::InGame),
                (setup_game, setup_game_camera, ensure_game_camera_visible),
            )
            // System to connect cards to zones after they're spawned - moved to FixedUpdate
            .add_systems(
                Update,
                (
                    connect_cards_to_zones,
                    check_card_status,
                    register_unzoned_cards.run_if(in_state(GameMenuState::InGame)),
                ),
            );
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
    game_state: Res<State<GameMenuState>>,
) {
    // Only run when the game state is InGame
    if *game_state.get() != GameMenuState::InGame {
        return;
    }

    for (entity, _) in query.iter() {
        info!("Connecting cards to zones...");

        let card_count = card_query.iter().count();
        info!("Found {} cards to connect to zones", card_count);

        if card_count == 0 {
            error!(
                "No cards found to connect to zones - cards are not being spawned properly. this error is only important when we are loading a game or in a game"
            );

            // Let's check if the zone manager already has any cards registered
            let mut total_cards = 0;

            // Count cards in hands
            for player_hand in zone_manager.hands.values() {
                total_cards += player_hand.len();
            }

            // Count cards in libraries
            for player_library in zone_manager.libraries.values() {
                total_cards += player_library.len();
            }

            // Count cards in battlefield
            total_cards += zone_manager.battlefield.len();

            // Count cards in graveyards
            for player_graveyard in zone_manager.graveyards.values() {
                total_cards += player_graveyard.len();
            }

            if total_cards > 0 {
                info!(
                    "Found {} cards already registered in the zone manager",
                    total_cards
                );
            } else {
                error!("Zone manager has no cards registered!");
            }
        }

        // Process each card and add it to the appropriate zone in ZoneManager
        for (card_entity, card_zone) in card_query.iter() {
            // First check if this card is already registered to avoid duplicates
            let already_registered = zone_manager.get_card_zone(card_entity).is_some();

            if already_registered {
                info!(
                    "Card {:?} is already registered in the zone manager, skipping",
                    card_entity
                );
                continue;
            }

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
                Zone::Exile | Zone::Stack | Zone::CommandZone => {
                    // These zones are managed differently and might require special handling
                    warn!(
                        "Card {:?} in {:?} zone - this zone is managed differently",
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

// Extension trait for ZoneManager to simplify diagnostic checks
#[allow(dead_code)]
trait ZoneManagerExt {
    fn get_card_zone(&self, card: Entity) -> Option<Zone>;
    fn get_player_zone(&self, player: Entity, zone: Zone) -> Option<&Vec<Entity>>;
}

impl ZoneManagerExt for ZoneManager {
    fn get_card_zone(&self, card: Entity) -> Option<Zone> {
        // Use the public get_card_zone method
        self.get_card_zone(card)
    }

    fn get_player_zone(&self, player: Entity, zone: Zone) -> Option<&Vec<Entity>> {
        // Use the public get_player_zone method
        self.get_player_zone(player, zone)
    }
}

// Ensure game camera is visible when entering game state
fn ensure_game_camera_visible(mut game_camera_query: Query<&mut Visibility, With<GameCamera>>) {
    if game_camera_query.is_empty() {
        error!("No game camera found when entering game state!");
        return;
    }

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
    game_state: Res<State<GameMenuState>>,
) {
    // Only run when the game state is InGame
    if *game_state.get() != GameMenuState::InGame {
        return;
    }

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

    // Count cards in all zones
    let mut zone_manager_card_count = 0;

    // Count cards in hands
    for player_hand in zone_manager.hands.values() {
        zone_manager_card_count += player_hand.len();
    }

    // Count cards in libraries
    for player_library in zone_manager.libraries.values() {
        zone_manager_card_count += player_library.len();
    }

    // Count cards in battlefield
    zone_manager_card_count += zone_manager.battlefield.len();

    // Count cards in graveyards
    for player_graveyard in zone_manager.graveyards.values() {
        zone_manager_card_count += player_graveyard.len();
    }

    info!("Zone Manager contains {} cards", zone_manager_card_count);

    if cards.is_empty() {
        error!(
            "No cards found! Cards are not being spawned properly. this error is only important when we are loading a game or in a game"
        );

        // Don't return early, continue with diagnostics
    } else {
        info!("Found {} cards in the world", cards.iter().count());
    }

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
        // Changed from single() to safely handle multiple cameras
        let camera_count = game_camera_query.iter().count();
        if camera_count > 1 {
            warn!(
                "Multiple game cameras found ({}), this may cause rendering issues",
                camera_count
            );
            info!(
                "Game camera entities: {:?}",
                game_camera_query.iter().collect::<Vec<_>>()
            );
        } else {
            info!(
                "Game camera entity: {:?}",
                game_camera_query.iter().next().unwrap()
            );
        }
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

// Setup game camera when entering the game state
fn setup_game_camera(commands: Commands, game_cameras: Query<Entity, With<GameCamera>>) {
    // Check if a game camera already exists
    if !game_cameras.is_empty() {
        info!("Game camera already exists, not creating a new one");
        return;
    }

    info!("No game camera found, creating a new one for the game state");

    // Call the camera module's setup system directly
    setup_camera(commands);
}

/// System to register cards that are not in any zone
fn register_unzoned_cards(
    cards: Query<
        (Entity, &CardZone),
        (
            With<crate::cards::Card>,
            Without<crate::game_engine::zones::ZoneMarker>,
        ),
    >,
    player_query: Query<(Entity, &Player)>,
    mut zone_manager: ResMut<ZoneManager>,
) {
    let card_count = cards.iter().count();
    if card_count == 0 {
        return;
    }

    info!(
        "Found {} cards not registered in any zone, attempting to register them",
        card_count
    );

    // Create a map of player indices to player entities
    let mut player_map = std::collections::HashMap::new();
    for (entity, player) in player_query.iter() {
        player_map.insert(player.player_index, entity);
    }

    // Register each card to the appropriate player's hand based on position
    for (card_entity, card_zone) in cards.iter() {
        // First check if this card is already registered to avoid duplicates
        let already_registered = zone_manager.get_card_zone(card_entity).is_some();
        if already_registered {
            continue;
        }

        let owner = if let Some(owner) = card_zone.zone_owner {
            owner
        } else if player_map.len() > 0 {
            // Default to first player if no owner is specified
            player_map.get(&0).copied().unwrap_or(Entity::PLACEHOLDER)
        } else {
            warn!(
                "No players found to assign card ownership for card {:?}",
                card_entity
            );
            Entity::PLACEHOLDER
        };

        // Initialize player zones if they don't exist yet
        zone_manager.init_player_zones(owner);

        // Add the card to the player's hand by default
        zone_manager.add_to_hand(owner, card_entity);
        info!(
            "Registered card {:?} to player {:?}'s hand",
            card_entity, owner
        );
    }
}
