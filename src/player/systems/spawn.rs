use crate::camera::components::{AppLayer, GameCamera};
use crate::card::{Card, CardDetails, Draggable};
use crate::deck::{get_player_shuffled_deck, get_player_specific_cards};
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

    info!("Spawning {} players...", config.player_count);

    // Spawn each player
    for player_index in 0..config.player_count {
        // Get position name for logging
        let position_name = config.get_position_name(player_index);

        // Create a new player using the builder pattern
        let player = Player::new(&format!("Player {} ({})", player_index + 1, position_name))
            .with_life(config.starting_life)
            .with_player_index(player_index);

        info!(
            "Creating player with index {} and name '{}'",
            player_index, player.name
        );

        // Get player position based on their index and config
        let player_transform = get_player_position(player_index, &config);

        // Spawn the player entity
        let player_entity = commands
            .spawn((
                player.clone(),
                player_transform,
                GlobalTransform::default(),
                AppLayer::game_layers(), // Add to all game layers
            ))
            .id();

        info!(
            "Spawned player entity {:?} with index {} and name '{}' at position {:?}",
            player_entity, player_index, player.name, player_transform.translation
        );

        // Only spawn cards for player 1 or if spawn_all_cards is true
        if player_index == 0 || config.spawn_all_cards {
            // Get player-specific cards and clone them for display
            let cards = get_player_specific_cards(player_entity, player_index);

            // Take the first 7 cards for display (representing a starting hand)
            let display_cards = cards.iter().take(7).cloned().collect::<Vec<_>>();

            // Create a player-specific deck
            let deck = get_player_shuffled_deck(
                player_entity,
                player_index,
                Some(&format!("Player {} Deck", player_index + 1)),
            );

            info!(
                "Added {} cards and a deck with {} cards to player {}",
                cards.len(),
                deck.cards.len(),
                player_index
            );

            // Update the player's cards while preserving other fields
            commands.entity(player_entity).insert(
                Player::new(&player.name)
                    .with_life(player.life)
                    .with_player_index(player.player_index)
                    .with_cards(cards)
                    .with_deck(deck),
            );

            // Spawn visual cards for all players that have cards
            info!(
                "Spawning visual cards for player {} ({})",
                player_index, position_name
            );

            // Get the base position for the player's cards
            // We use the player's actual transform instead of overriding with the Y offset
            let card_position = player_transform.translation;

            // Create visual representations of the cards
            spawn_visual_cards(
                &mut commands,
                display_cards,
                &game_cameras,
                &config.card_size,
                config.card_spacing_multiplier,
                card_position,
                player_index,
            );
        } else {
            info!(
                "Skipping card spawning for player {} (index {})",
                player.name, player_index
            );
        }
    }

    info!("Player spawning complete!");
}

/// Calculate the appropriate position for a player based on their index
fn get_player_position(player_index: usize, config: &PlayerConfig) -> Transform {
    // Use the config's method to calculate the position
    let position = config.calculate_player_position(player_index);

    Transform::from_translation(position)
}

/// Helper function to spawn visual card entities
fn spawn_visual_cards(
    commands: &mut Commands,
    display_cards: Vec<Card>,
    game_cameras: &Query<Entity, With<GameCamera>>,
    card_size: &Vec2,
    spacing_multiplier: f32,
    player_position: Vec3,
    player_index: usize,
) {
    // Increase the spacing between cards
    let spacing = card_size.x * spacing_multiplier * 1.5;

    // Calculate the total width of all cards with spacing
    let total_width = display_cards.len() as f32 * spacing;

    // Move the starting position further to the left for better distribution
    let start_x = -(total_width) / 2.0 + spacing / 2.0;

    // Determine card orientation based on player position
    // For 4 players, we need different layouts based on table position:
    // - Player 0 (bottom): horizontal row, facing up
    // - Player 1 (right): vertical column, cards to the right side
    // - Player 2 (top): horizontal row, facing down
    // - Player 3 (left): vertical column, cards to the left side
    let (start_pos, card_direction) = match player_index % 4 {
        0 => (
            Vec3::new(start_x, player_position.y, 0.0),
            Vec3::new(spacing, 0.0, 0.0),
        ), // bottom: cards in row
        1 => (
            Vec3::new(player_position.x, start_x, 0.0),
            Vec3::new(0.0, spacing, 0.0),
        ), // right: cards in column
        2 => (
            Vec3::new(start_x, player_position.y, 0.0),
            Vec3::new(spacing, 0.0, 0.0),
        ), // top: cards in row
        3 => (
            Vec3::new(player_position.x, start_x, 0.0),
            Vec3::new(0.0, spacing, 0.0),
        ), // left: cards in column
        _ => (
            Vec3::new(start_x, player_position.y, 0.0),
            Vec3::new(spacing, 0.0, 0.0),
        ), // default
    };

    info!(
        "Spawning {} cards for player {} at position ({:.2}, {:.2}, {:.2})",
        display_cards.len(),
        player_index,
        start_pos.x,
        start_pos.y,
        start_pos.z
    );

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

    // Spawn visual cards in appropriate arrangement
    for (i, card) in display_cards.into_iter().enumerate() {
        // Use a base z-index based on player index
        let base_z = player_index as f32 * 100.0;
        let z = base_z + i as f32;

        // Calculate position based on direction and starting position
        let position = Vec3::new(
            start_pos.x + card_direction.x * i as f32,
            start_pos.y + card_direction.y * i as f32,
            z,
        );

        let transform = Transform::from_translation(position);

        info!(
            "Positioning card '{}' at ({:.2}, {:.2}, {:.2})",
            card.name, position.x, position.y, position.z
        );

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

    info!(
        "Finished spawning cards, total width={:.2}, using spacing={:.2}",
        total_width, spacing
    );
}
