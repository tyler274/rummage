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

    info!("Spawning {} players...", config.player_count);

    // Spawn each player
    for player_index in 0..config.player_count {
        // Create a new player using the builder pattern
        let player = Player::builder(&format!("Player {}", player_index + 1))
            .life(config.starting_life)
            .player_index(player_index)
            .build();

        info!(
            "Creating player with index {} and name '{}'",
            player_index, player.name
        );

        // Get player position based on their index
        let player_transform = get_player_position(player_index, config.player_count);

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
            // Get example cards and clone them for display
            let cards = get_example_cards(player_entity);
            // Take the first 7 cards for display
            let display_cards = cards.iter().take(7).cloned().collect::<Vec<_>>();

            // Create a deck for the player
            let deck = get_shuffled_deck(player_entity);

            info!(
                "Added {} cards and a deck with {} cards to player {}",
                cards.len(),
                deck.cards.len(),
                player_index
            );

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
                info!("Spawning visual cards for player {}", player_index);
                spawn_visual_cards(
                    &mut commands,
                    display_cards,
                    &game_cameras,
                    &config.card_size,
                    config.card_spacing_multiplier,
                    player_transform.translation, // Pass player position to spawn_visual_cards
                );
            } else {
                info!(
                    "Player {} has cards in their data but visual cards are not spawned yet",
                    player_index
                );
            }
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
fn get_player_position(player_index: usize, total_players: usize) -> Transform {
    // Default position values
    let mut position = Vec3::new(0.0, 0.0, 0.0);

    // For now, we'll implement a simple 2-player setup (face to face)
    // In the future, this could be expanded for more players in different positions
    match (player_index, total_players) {
        // Player 1 (index 0) - bottom of the screen
        (0, _) => {
            position.y = -15.0; // Position Player 1 much further down so cards appear at very bottom
        }
        // Player 2 (index 1) - top of the screen (opponent)
        (1, _) => {
            position.y = 15.0; // Keep Player 2 at top (opponent) mirroring Player 1 position
            // Rotate 180 degrees (π radians) to face Player 1
            return Transform::from_xyz(position.x, position.y, position.z)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::PI));
        }
        // For future expansion - 3+ players
        (idx, count) if idx < count => {
            // Calculate positions in a circle for 3+ players
            let angle = (idx as f32 / count as f32) * 2.0 * std::f32::consts::PI;
            let radius = 15.0; // Increased distance from center to match player 1/2 positions
            position.x = radius * angle.cos();
            position.y = radius * angle.sin();
            // Face the center
            let rotation = Quat::from_rotation_z(angle + std::f32::consts::PI);
            return Transform::from_xyz(position.x, position.y, position.z).with_rotation(rotation);
        }
        // Fallback for any other case
        _ => {}
    }

    Transform::from_translation(position)
}

/// Helper function to spawn visual card entities
fn spawn_visual_cards(
    commands: &mut Commands,
    display_cards: Vec<Card>,
    game_cameras: &Query<Entity, With<GameCamera>>,
    card_size: &Vec2,
    spacing_multiplier: f32,
    player_position: Vec3, // Add player position parameter
) {
    // Increase the spacing between cards
    let spacing = card_size.x * spacing_multiplier * 1.5; // Increased spacing by 50%

    // Calculate the total width of all cards with spacing
    let total_width = display_cards.len() as f32 * spacing;

    // Move the starting position further to the left for better distribution
    let start_x = -(total_width) / 2.0 + spacing / 2.0;

    info!(
        "Spawning {} cards with spacing {:.2}, total width {:.2}, starting at x={:.2}",
        display_cards.len(),
        spacing,
        total_width,
        start_x
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

    // Spawn visual cards in a row
    for (i, card) in display_cards.into_iter().enumerate() {
        let z = i as f32;
        // Position cards at player position
        let x_pos = start_x + i as f32 * spacing + player_position.x;
        let y_pos = player_position.y + 2.0; // Position cards slightly above the player

        let transform = Transform::from_xyz(x_pos, y_pos, z);

        info!(
            "Positioning card '{}' at ({:.2}, {:.2}, {:.2})",
            card.name, x_pos, y_pos, z
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
