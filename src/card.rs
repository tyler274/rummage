use bevy::{
    prelude::*,
    input::mouse::MouseButton,
};
use bitflags::bitflags;
use crate::mana::Mana;

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct CreatureType: u32 {
        const NONE = 0;
        const HUMAN = 1 << 0;
        const WIZARD = 1 << 1;
        const DRAGON = 1 << 2;
        const ANGEL = 1 << 3;
        const DEMON = 1 << 4;
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub name: String,
    pub cost: Mana,
    pub card_type: CardType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardType {
    Creature(CreatureCard),
    // Add other card types here as needed
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureCard {
    pub power: i32,
    pub toughness: i32,
    pub creature_type: CreatureType,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct CreatureOnField {
    pub power_modifier: i64,
    pub toughness_modifier: i64,
    pub battle_damage: u64,
    pub token: bool,
}

#[derive(Component)]
pub struct Draggable {
    pub dragging: bool,
    pub drag_offset: Vec2,
    pub z_index: f32,
}

#[derive(Component)]
pub struct CardTextContent {
    pub text: String,
    pub text_type: CardTextType,
}

#[derive(Component)]
pub struct SpawnedText;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CardTextType {
    Name,
    Cost,
    Type,
    PowerToughness,
}

#[derive(Resource)]
pub struct DebugConfig {
    pub show_text_positions: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            show_text_positions: false,
        }
    }
}

pub fn spawn_hand(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let card_width = 100.0;
    let card_height = card_width * 1.4;
    let spacing = 20.0;
    let num_cards = 5;
    let total_width = (num_cards as f32 * card_width) + ((num_cards - 1) as f32 * spacing);
    let start_x = -total_width / 2.0;
    let y = -250.0;

    // Define iconic MTG cards
    let cards = vec![
        Card {
            name: "Serra Angel".to_string(),
            cost: Mana::new(3, 2, 0, 0, 0, 0), // 3W2
            card_type: CardType::Creature(CreatureCard {
                power: 4,
                toughness: 4,
                creature_type: CreatureType::HUMAN | CreatureType::ANGEL,
            }),
        },
        Card {
            name: "Shivan Dragon".to_string(),
            cost: Mana::new(4, 0, 0, 0, 2, 0), // 4RR
            card_type: CardType::Creature(CreatureCard {
                power: 5,
                toughness: 5,
                creature_type: CreatureType::DRAGON,
            }),
        },
        Card {
            name: "Jace's Archivist".to_string(),
            cost: Mana::new(1, 0, 2, 0, 0, 0), // 1UU
            card_type: CardType::Creature(CreatureCard {
                power: 2,
                toughness: 2,
                creature_type: CreatureType::HUMAN | CreatureType::WIZARD,
            }),
        },
        Card {
            name: "Prodigal Sorcerer".to_string(),
            cost: Mana::new(2, 0, 0, 1, 0, 0), // 2U
            card_type: CardType::Creature(CreatureCard {
                power: 1,
                toughness: 1,
                creature_type: CreatureType::HUMAN | CreatureType::WIZARD,
            }),
        },
        Card {
            name: "Dragon Mage".to_string(),
            cost: Mana::new(5, 0, 0, 0, 2, 0), // 5RR - corrected parameter order for red mana
            card_type: CardType::Creature(CreatureCard {
                power: 5,
                toughness: 5,
                creature_type: CreatureType::DRAGON | CreatureType::WIZARD,
            }),
        },
    ];

    for (i, card) in cards.into_iter().enumerate() {
        let x = start_x + (i as f32 * (card_width + spacing));
        let z = i as f32;

        // Create a card entity with required components
        let card_entity = commands
            .spawn((
                Sprite {
                    custom_size: Some(Vec2::new(card_width, card_height)),
                    color: Color::srgb(0.8, 0.8, 0.8), // Light gray color
                    ..default()
                },
                Transform::from_xyz(x, y, z),
                card.clone(),
                Draggable {
                    dragging: false,
                    drag_offset: Vec2::ZERO,
                    z_index: z,
                },
            ))
            .insert(GlobalTransform::default())
            .insert(Visibility::Visible)
            .insert(InheritedVisibility::default())
            .insert(ViewVisibility::default())
            .id();

        // Spawn text content entities as children
        commands.spawn((
            CardTextContent {
                text: card.name.clone(),
                text_type: CardTextType::Name,
            },
        )).set_parent(card_entity);

        commands.spawn((
            CardTextContent {
                text: card.cost.to_string(),
                text_type: CardTextType::Cost,
            },
        )).set_parent(card_entity);

        // Safely handle card type
        if let CardType::Creature(creature) = &card.card_type {
            commands.spawn((
                CardTextContent {
                    text: creature_type_to_string(&creature.creature_type),
                    text_type: CardTextType::Type,
                },
            )).set_parent(card_entity);

            commands.spawn((
                CardTextContent {
                    text: format!("{}/{}", creature.power, creature.toughness),
                    text_type: CardTextType::PowerToughness,
                },
            )).set_parent(card_entity);
        }
    }
}

pub fn handle_card_dragging(
    mut card_query: Query<(Entity, &mut Transform, &mut Draggable, &GlobalTransform), With<Card>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // Safely get window and camera
    let Ok(window) = windows.get_single() else {
        return; // No window available
    };
    
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return; // No camera available
    };

    if let Some(cursor_pos) = window.cursor_position() {
        // Convert cursor position to world coordinates
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Handle mouse press - start dragging
            if mouse_button.just_pressed(MouseButton::Left) {
                let mut highest_z = f32::NEG_INFINITY;
                let mut top_card = None;

                // First pass: find the card with highest z-index at cursor position
                for (entity, _, draggable, global_transform) in card_query.iter() {
                    let card_pos = global_transform.translation().truncate();
                    let card_size = Vec2::new(100.0, 140.0);
                    
                    // Check if the cursor is within the card bounds
                    if world_pos.x >= card_pos.x - card_size.x / 2.0
                        && world_pos.x <= card_pos.x + card_size.x / 2.0
                        && world_pos.y >= card_pos.y - card_size.y / 2.0
                        && world_pos.y <= card_pos.y + card_size.y / 2.0
                    {
                        if draggable.z_index > highest_z {
                            highest_z = draggable.z_index;
                            top_card = Some(entity);
                        }
                    }
                }

                // Second pass: start dragging only the top card
                if let Some(top_entity) = top_card {
                    // Find the highest z-index among all cards
                    let mut max_z = highest_z;
                    for (_, _, draggable, _) in card_query.iter() {
                        max_z = max_z.max(draggable.z_index);
                    }

                    for (entity, mut transform, mut draggable, global_transform) in card_query.iter_mut() {
                        if entity == top_entity {
                            let card_pos = global_transform.translation().truncate();
                            draggable.dragging = true;
                            draggable.drag_offset = card_pos - world_pos;
                            // Set the dragged card's z-index higher than all others
                            draggable.z_index = max_z + 1.0;
                            transform.translation.z = max_z + 1.0;
                        }
                    }
                }
            }

            // Handle mouse release - stop dragging and update z-index
            if mouse_button.just_released(MouseButton::Left) {
                let mut max_z = f32::NEG_INFINITY;
                
                // First find the highest z-index
                for (_, _, draggable, _) in card_query.iter() {
                    max_z = max_z.max(draggable.z_index);
                }

                // Then update the previously dragged card
                for (_, _, mut draggable, _) in card_query.iter_mut() {
                    if draggable.dragging {
                        draggable.dragging = false;
                        draggable.z_index = max_z + 1.0; // Place it on top
                    }
                }
            }

            // Update position of dragged cards
            for (_, mut transform, draggable, _) in card_query.iter_mut() {
                if draggable.dragging {
                    let new_pos = world_pos + draggable.drag_offset;
                    transform.translation.x = new_pos.x;
                    transform.translation.y = new_pos.y;
                    // Maintain the z-index we set when dragging started
                    transform.translation.z = draggable.z_index;
                }
            }
        }
    }
}

pub fn creature_type_to_string(creature_type: &CreatureType) -> String {
    let mut types = Vec::new();
    
    if creature_type.contains(CreatureType::DRAGON) {
        types.push("Dragon");
    }
    if creature_type.contains(CreatureType::WIZARD) {
        types.push("Wizard");
    }
    if creature_type.contains(CreatureType::HUMAN) {
        types.push("Human");
    }
    if creature_type.contains(CreatureType::ANGEL) {
        types.push("Angel");
    }
    if creature_type.contains(CreatureType::DEMON) {
        types.push("Demon");
    }
    
    if types.is_empty() {
        "Unknown".to_string()
    } else {
        types.join(" ")
    }
}

pub fn debug_render_text_positions(
    mut gizmos: Gizmos,
    card_query: Query<(&Transform, &Card), With<Card>>,
    config: Res<DebugConfig>,
) {
    if !config.show_text_positions {
        return;
    }

    for (transform, _) in card_query.iter() {
        let card_pos = transform.translation.truncate();
        let card_width = 100.0;
        let card_height = card_width * 1.4;

        // Note: Using Color::srgb instead of Color::rgb as rgb is deprecated
        
        // Name position (top left) - red dot
        let name_pos = card_pos + Vec2::new(-card_width * 0.25, card_height * 0.35);
        gizmos.circle_2d(name_pos, 3.0, Color::srgb(1.0, 0.0, 0.0));

        // Mana cost position (top right) - blue dot
        let cost_pos = card_pos + Vec2::new(card_width * 0.35, card_height * 0.35);
        gizmos.circle_2d(cost_pos, 3.0, Color::srgb(0.0, 0.0, 1.0));

        // Type position (middle center) - green dot
        let type_pos = card_pos + Vec2::new(0.0, card_height * 0.1);
        gizmos.circle_2d(type_pos, 3.0, Color::srgb(0.0, 1.0, 0.0));

        // Power/Toughness position (bottom right) - yellow dot
        let pt_pos = card_pos + Vec2::new(card_width * 0.35, -card_height * 0.35);
        gizmos.circle_2d(pt_pos, 3.0, Color::srgb(1.0, 1.0, 0.0));
    }
}
