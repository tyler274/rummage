use crate::camera::components::{AppLayer, GameCamera};
/// Player management and card spawning system.
///
/// This module handles:
/// - Player state and resources
/// - Initial hand setup
/// - Card spawning and positioning
/// - Text component creation for cards
///
/// # Card Layout
/// Cards are spawned in a horizontal arrangement with:
/// - Consistent spacing between cards
/// - Proper z-indexing for overlapping
/// - Centered alignment in the view
///
/// # Text Components
/// Each spawned card includes several text components:
/// - Card name (top left)
/// - Mana cost (top right)
/// - Type line (center)
/// - Power/Toughness (bottom right, creatures only)
/// - Rules text (center body)
///
/// Each text component is spawned as a child entity of the card,
/// ensuring proper positioning and movement during drag operations.
use crate::card::{Card, CardDetails, Draggable, get_example_cards};
use crate::mana::{ManaPool, convert_rules_text_to_symbols};
use crate::text::{CardTextContent, CardTextType};
use bevy::prelude::*;
use rand::seq::SliceRandom;

/// Represents a player in the game with their associated state
#[allow(dead_code)]
#[derive(Component, Default, Debug, Clone)]
pub struct Player {
    /// Player's display name
    pub name: String,
    /// Current life total
    pub life: i32,
    /// Available mana pool
    pub mana_pool: ManaPool,
    /// Cards in the player's possession
    pub cards: Vec<Card>,
}

/// Spawns the initial hand of cards for a player
///
/// This function:
/// 1. Creates a new player entity
/// 2. Generates and shuffles a deck of cards
/// 3. Takes the first 7 cards for the initial hand
/// 4. Spawns visual representations of the cards
/// 5. Creates text components for each card
///
/// The cards are arranged in a horizontal line with proper spacing
/// and z-indexing for visual clarity.
pub fn spawn_hand(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Create a new player
    let player = Player {
        name: "Player 1".to_string(),
        life: 20,
        mana_pool: ManaPool::default(),
        cards: Vec::new(),
    };

    // Spawn the player entity
    let player_entity = commands
        .spawn((
            player.clone(),
            AppLayer::game_layers(), // Add to all game layers
        ))
        .id();

    // Get example cards and clone them for display
    let mut cards = get_example_cards(player_entity);
    // Shuffle the initial hand
    cards.shuffle(&mut rand::rng());
    // Take the first 7 cards for display
    let display_cards = cards.iter().take(7).cloned().collect::<Vec<_>>();

    // Update the player's cards while preserving other fields
    commands
        .entity(player_entity)
        .insert(Player { cards, ..player });

    let card_size = Vec2::new(672.0, 936.0);
    let spacing = card_size.x * 1.1; // Reduced spacing multiplier for tighter layout
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
                    color: Color::WHITE,
                    custom_size: Some(card_size),
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
        commands
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
            .set_parent(card_entity);
    }
}
