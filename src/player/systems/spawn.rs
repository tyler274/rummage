use crate::camera::components::{AppLayer, GameCamera};
use crate::card::{Card, CardDetails, Draggable};
use crate::deck::{get_example_cards, get_shuffled_deck};
use crate::mana::convert_rules_text_to_symbols;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use crate::text::CardTextContent;
use bevy::prelude::*;

/// Spawns players according to the PlayerConfig resource
///
/// This system:
/// 1. Reads the PlayerConfig to determine how many players to spawn
/// 2. Creates player entities with appropriate positioning
/// 3. Only spawns cards for player 1 by default (or all if configured)
pub fn spawn_players(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    player_config: Option<Res<PlayerConfig>>,
) {
    // Use default config if none exists
    let config = player_config.map(|c| c.clone()).unwrap_or_default();

    // Spawn each player
    for player_index in 0..config.player_count {
        // Create a new player using the builder pattern
        let player = Player::builder(&format!("Player {}", player_index + 1))
            .life(config.starting_life)
            .player_index(player_index)
            .build();

        // Spawn the player entity
        let player_entity = commands
            .spawn((
                player.clone(),
                AppLayer::game_layers(), // Add to all game layers
            ))
            .id();

        // Only spawn cards for player 1 or if spawn_all_cards is true
        if player_index == 0 || config.spawn_all_cards {
            // Get example cards and clone them for display
            let cards = get_example_cards(player_entity);
            // Take the first 7 cards for display
            let display_cards = cards.iter().take(7).cloned().collect::<Vec<_>>();

            // Create a deck for the player
            let deck = get_shuffled_deck(player_entity);

            // Update the player's cards while preserving other fields
            commands.entity(player_entity).insert(
                Player::builder(&player.name)
                    .life(player.life)
                    .player_index(player.player_index)
                    .cards(cards)
                    .deck(deck)
                    .build(),
            );

            // Only spawn visual cards for player 1 for now
            if player_index == 0 {
                spawn_visual_cards(
                    &mut commands,
                    display_cards,
                    &game_cameras,
                    &config.card_size,
                    config.card_spacing_multiplier,
                );
            }
        }
    }
}

/// Helper function to spawn visual card entities
fn spawn_visual_cards(
    commands: &mut Commands,
    display_cards: Vec<Card>,
    game_cameras: &Query<Entity, With<GameCamera>>,
    card_size: &Vec2,
    spacing_multiplier: f32,
) {
    let spacing = card_size.x * spacing_multiplier;
    let start_x = -(display_cards.len() as f32 * spacing) / 2.0 + spacing / 2.0;

    // Get game camera entity to set render target
    let game_camera_entities: Vec<Entity> = game_cameras.iter().collect();
    if !game_camera_entities.is_empty() {
        info!(
            "Found game camera for card rendering: {:?}",
            game_camera_entities[0]
        );
    } else {
        info!("No game camera found, using default camera");
    }

    // Spawn visual cards
    for (i, card) in display_cards.into_iter().enumerate() {
        let z = i as f32;
        let transform = Transform::from_xyz(start_x + i as f32 * spacing, 0.0, z);

        let card_entity = commands
            .spawn((
                card.clone(),
                Sprite {
                    color: Color::srgb(0.85, 0.85, 0.85),
                    custom_size: Some(*card_size),
                    ..default()
                },
                transform,
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Draggable {
                    dragging: false,
                    drag_offset: Vec2::ZERO,
                    z_index: z,
                },
                AppLayer::Cards.layer(), // Use the specific Cards layer
            ))
            .id();

        // Spawn card text content
        let text_entity = commands
            .spawn((
                CardTextContent {
                    name: card.name.clone(),
                    mana_cost: card.cost.to_string(),
                    type_line: card.type_line(),
                    rules_text: convert_rules_text_to_symbols(&card.rules_text),
                    power_toughness: if let CardDetails::Creature(creature) = &card.card_details {
                        Some(format!("{}/{}", creature.power, creature.toughness))
                    } else {
                        None
                    },
                },
                Transform::default(),
                AppLayer::Cards.layer(), // Use the specific Cards layer
            ))
            .set_parent(card_entity)
            .id();

        info!(
            "Spawned CardTextContent entity {:?} as child of card entity {:?}",
            text_entity, card_entity
        );
    }
}
