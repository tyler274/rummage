use bevy::prelude::*;
use bevy::text::JustifyText;

use crate::cards::Card;
use crate::text::{
    components::{CardManaCostText, CardTextType, TextLayoutInfo},
    mana_symbols::{ManaSymbolOptions, render_mana_symbol},
    utils::{calculate_text_size, get_adaptive_font_size, get_card_layout},
};

/// Spawn mana cost text for a card
pub fn spawn_mana_cost_text(
    commands: &mut Commands,
    mana_cost_component: &CardManaCostText,
    _card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();

    // Calculate position - on the right side of the card top
    let mana_position = Vec2::new(
        layout.mana_cost_x_offset * card_size.x,
        layout.mana_cost_y_offset * card_size.y,
    );

    // Calculate available width for the mana cost
    let available_width = card_size.x * 0.15; // Use 15% of card width for mana cost

    // Use adaptive font sizing based on mana cost complexity
    let font_size = get_adaptive_font_size(
        card_size,
        14.0, // Base size for mana cost
        &mana_cost_component.mana_cost,
        available_width,
        10.0, // Minimum size
    );

    // Load font - we'll use regular font for now
    // In future updates this could be replaced with specialized mana symbol rendering
    let font = asset_server.load("fonts/DejaVuSans.ttf");

    // Create the mana cost entity
    commands
        .spawn((
            Text2d::new(mana_cost_component.mana_cost.clone()),
            Transform::from_translation(Vec3::new(
                mana_position.x,
                mana_position.y,
                0.1, // Slightly above the card
            )),
            GlobalTransform::default(),
            TextFont {
                font,
                font_size,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Right), // Right-justified
            CardTextType::ManaCost,
            TextLayoutInfo {
                alignment: JustifyText::Right,
            },
            Name::new(format!("Mana Cost: {}", mana_cost_component.mana_cost)),
        ))
        .id()
}

/// System implementation that finds cards and creates mana cost text for them
#[allow(dead_code)]
pub fn mana_cost_text_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Card)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, card) in query.iter() {
        // Skip cards with no mana cost
        if card.cost.cost.is_empty() {
            continue;
        }

        // Convert Mana struct to display string
        let mana_cost_string = card.cost.cost.to_string();

        // Get card position and size
        let card_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let layout = get_card_layout();
        let card_size = Vec2::new(layout.card_width, layout.card_height);

        // Create content for mana cost text
        let content = CardManaCostText {
            mana_cost: mana_cost_string,
        };

        // Create the mana cost text
        let text_entity =
            spawn_mana_cost_text(&mut commands, &content, card_pos, card_size, &asset_server);

        // Add as child of the card entity
        commands.entity(entity).add_child(text_entity);
    }
}
