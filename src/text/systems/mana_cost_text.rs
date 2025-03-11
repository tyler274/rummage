use crate::card::Card;
use crate::text::{
    components::{CardTextBundle, CardTextType, TextLayoutInfo},
    utils::{calculate_text_position, calculate_text_size, get_card_font_size, get_card_layout},
};
use bevy::prelude::*;

/// Creates a text entity for mana cost
pub fn create_mana_cost_text(
    commands: &mut Commands,
    content: &crate::text::components::CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Skip cards with no mana cost
    if content.mana_cost.is_empty() {
        return commands.spawn_empty().id();
    }

    // Load mana symbol font
    let mana_font = asset_server.load("fonts/Mana.ttf");
    let layout = get_card_layout();

    // Font size for mana symbols
    let font_size = 25.0;

    // Format mana cost for the Mana font
    // The font requires the mana symbols to remain in their braced format {W}, {U}, etc.
    let mana_text = content.mana_cost.clone();

    // Position the mana cost correctly
    let mana_position = Vec2::new(
        layout.mana_cost_x_offset * card_size.x,
        layout.mana_cost_y_offset * card_size.y,
    );

    // Create the text entity with high z-index
    commands
        .spawn(CardTextBundle {
            text_2d: Text2d::new(mana_text.clone()),
            transform: Transform::from_translation(Vec3::new(
                mana_position.x,
                mana_position.y,
                1.0, // High z-index
            )),
            global_transform: GlobalTransform::default(),
            text_font: TextFont {
                font: mana_font.clone(),
                font_size,
                ..default()
            },
            text_color: TextColor(Color::rgba(0.0, 0.0, 0.0, 1.0)), // Black text
            text_layout: TextLayout::new_with_justify(JustifyText::Right), // Right-justified
            card_text_type: CardTextType::ManaCost,
            text_layout_info: TextLayoutInfo {
                position: card_pos + mana_position,
                size: Vec2::new(card_size.x * 0.3, card_size.y * 0.1),
                alignment: JustifyText::Right,
            },
            name: Name::new(format!("Mana Cost: {}", mana_text)),
        })
        .id()
}

/// System implementation that finds cards and creates mana cost text for them
pub fn mana_cost_text_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Card)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, card) in query.iter() {
        // Skip cards with no mana cost
        if card.cost.is_empty() {
            continue;
        }

        // Convert Mana struct to display string
        let mana_cost_string = card.cost.to_string();

        // Get card position and size
        let card_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let layout = get_card_layout();
        let card_size = Vec2::new(layout.card_width, layout.card_height);

        // Create content for mana cost text
        let content = crate::text::components::CardTextContent {
            name: card.name.clone(),
            mana_cost: mana_cost_string,
            type_line: card.type_line(),
            rules_text: card.rules_text.clone(),
            power_toughness: match &card.card_details {
                crate::card::CardDetails::Creature(creature) => {
                    Some(format!("{}/{}", creature.power, creature.toughness))
                }
                _ => None,
            },
        };

        // Create the mana cost text
        let text_entity =
            create_mana_cost_text(&mut commands, &content, card_pos, card_size, &asset_server);

        // Add as child of the card entity
        commands.entity(entity).add_child(text_entity);
    }
}
