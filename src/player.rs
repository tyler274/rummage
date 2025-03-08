use crate::card::{
    card_types_to_string, Card, CardDetails, CardTextContent, CardTextType, Draggable,
};
use crate::cards::get_example_cards;
use crate::mana::ManaPool;
use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component, Default, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub life: i32,
    pub mana_pool: ManaPool,
    pub cards: Vec<Card>,
}

pub fn spawn_hand(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Create a new player
    let player = Player {
        name: "Player 1".to_string(),
        life: 20,
        mana_pool: ManaPool::default(),
        cards: Vec::new(),
    };

    // Spawn the player entity
    let player_entity = commands.spawn(player.clone()).id();

    // Get example cards and clone them for display
    let cards = get_example_cards(player_entity);
    let display_cards = cards.clone();

    // Update the player's cards while preserving other fields
    commands
        .entity(player_entity)
        .insert(Player { cards, ..player });

    let card_size = Vec2::new(100.0, 140.0);
    let spacing = 120.0;
    let start_x = -(display_cards.len() as f32 * spacing) / 2.0 + spacing / 2.0;

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
                Draggable {
                    dragging: false,
                    drag_offset: Vec2::ZERO,
                    z_index: z,
                },
            ))
            .id();

        // Spawn card name text
        commands
            .spawn((
                CardTextContent {
                    text: card.name.clone(),
                    text_type: CardTextType::Name,
                },
                Transform::from_xyz(0.0, card_size.y * 0.3, z + 0.1),
            ))
            .set_parent(card_entity);

        // Spawn mana cost text
        commands
            .spawn((
                CardTextContent {
                    text: card.cost.to_string(),
                    text_type: CardTextType::Cost,
                },
                Transform::from_xyz(card_size.x * 0.4, card_size.y * 0.3, z + 0.1),
            ))
            .set_parent(card_entity);

        // Spawn type line text
        commands
            .spawn((
                CardTextContent {
                    text: card_types_to_string(&card.types),
                    text_type: CardTextType::Type,
                },
                Transform::from_xyz(0.0, 0.0, z + 0.1),
            ))
            .set_parent(card_entity);

        // Spawn power/toughness text for creatures
        if let CardDetails::Creature(creature) = &card.card_details {
            commands
                .spawn((
                    CardTextContent {
                        text: format!("{}/{}", creature.power, creature.toughness),
                        text_type: CardTextType::PowerToughness,
                    },
                    Transform::from_xyz(card_size.x * 0.4, -card_size.y * 0.4, z + 0.1),
                ))
                .set_parent(card_entity);
        }
    }
}
