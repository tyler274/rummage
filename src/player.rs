use crate::card::{
    card_types_to_string, Card, CardDetails, CardTextContent, CardTextType, Draggable,
};
use crate::cards::get_example_cards;
use crate::mana::ManaPool;
use bevy::prelude::*;
use bevy::sprite::Sprite;
use bevy_turborand::prelude::*;

#[allow(dead_code)]
#[derive(Component, Default, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub life: i32,
    pub mana_pool: ManaPool,
    pub cards: Vec<Card>,
}

pub fn spawn_hand(mut commands: Commands, _asset_server: Res<AssetServer>, rng: ResMut<GlobalRng>) {
    let cards = get_example_cards(rng);
    let card_size = Vec2::new(100.0, 140.0);
    let spacing = 120.0;
    let start_x = -(cards.len() as f32 * spacing) / 2.0 + spacing / 2.0;

    for (i, card) in cards.into_iter().enumerate() {
        let z = i as f32;
        let card_entity = commands
            .spawn((
                card.clone(),
                Sprite {
                    custom_size: Some(card_size),
                    ..default()
                },
                Transform::from_xyz(start_x + i as f32 * spacing, 0.0, z),
                Draggable {
                    dragging: false,
                    drag_offset: Vec2::ZERO,
                    z_index: z,
                },
            ))
            .id();

        // Spawn card name text
        commands
            .spawn((CardTextContent {
                text: card.name.clone(),
                text_type: CardTextType::Name,
            },))
            .set_parent(card_entity);

        // Spawn mana cost text
        commands
            .spawn((CardTextContent {
                text: card.cost.to_string(),
                text_type: CardTextType::Cost,
            },))
            .set_parent(card_entity);

        // Spawn type line text
        commands
            .spawn((CardTextContent {
                text: card_types_to_string(&card.types),
                text_type: CardTextType::Type,
            },))
            .set_parent(card_entity);

        // Spawn power/toughness text for creatures
        if let CardDetails::Creature(creature) = &card.card_details {
            commands
                .spawn((CardTextContent {
                    text: format!("{}/{}", creature.power, creature.toughness),
                    text_type: CardTextType::PowerToughness,
                },))
                .set_parent(card_entity);
        }
    }
}
