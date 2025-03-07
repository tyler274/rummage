mod card;
mod cards;
mod mana;
mod player;

use bevy::prelude::*;
use bevy::math::Rect;
use bevy::window::WindowResized;
use mana::Mana;
use card::{Card, CardType, CreatureCard, CreatureType};
use bevy::app::AppExit;

#[derive(Component)]
struct Draggable {
    dragging: bool,
    drag_offset: Vec2,
    z_index: f32,
}

#[derive(Component)]
struct CardText {
    offset: Vec2,  // Store the initial offset from the card
    content: String, // Store the text content
}

#[derive(Component)]
struct CardTextContent {
    text: String,
    position: Vec2,
}

#[derive(Component)]
struct SpawnedText;

#[derive(Component)]
struct InHand {
    #[allow(dead_code)]
    index: usize,
}

fn hello_world() {
    println!("hello world!");
    println!("Mana default color is: {:?}", Mana::default());
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        },
        Transform::default(),
    ));
}

fn handle_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut projection_query: Query<&mut OrthographicProjection>,
    mut windows: Query<&mut Window>,
) {
    let Ok(mut window) = windows.get_single_mut() else { return };
    
    for event in resize_reader.read() {
        // Force a window redraw to clear any artifacts
        window.present_mode = bevy::window::PresentMode::AutoVsync;
        
        if let Ok(mut projection) = projection_query.get_single_mut() {
            // Keep a fixed vertical size and scale the horizontal size with aspect ratio
            let vertical_size = 800.0;
            let aspect_ratio = event.width / event.height;
            let horizontal_size = vertical_size * aspect_ratio;
            
            projection.area = Rect::new(
                -horizontal_size / 2.0,
                -vertical_size / 2.0,
                horizontal_size / 2.0,
                vertical_size / 2.0,
            );
            projection.scale = 1.0;
        }
    }
}

fn creature_type_to_string(creature_type: &CreatureType) -> String {
    if creature_type.contains(CreatureType::DRAGON) {
        "Dragon".to_string()
    } else if creature_type.contains(CreatureType::WIZARD) {
        "Wizard".to_string()
    } else if creature_type.contains(CreatureType::HUMAN) {
        "Human".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn spawn_card(
    commands: &mut Commands,
    name: String,
    cost: u64,
    card_type: CardType,
    creature_type: Option<CreatureType>,
    power: Option<u64>,
    toughness: Option<u64>,
    abilities: Vec<String>,
    position: Vec2,
) -> Entity {
    let card_width = 100.0;
    let card_height = card_width * 1.4; // Standard card ratio

    // Create a card entity with all required components
    let card_entity = commands.spawn((
        // Visual components
        Sprite {
            color: Color::srgb(0.8, 0.8, 0.8),
            custom_size: Some(Vec2::new(card_width, card_height)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        // Game components
        Card {
            name: name.clone(),
            cost,
            id: 1,
        },
        card_type.clone(),
        if let Some(p) = power {
            CreatureCard {
                power: p,
                toughness: toughness.unwrap(),
                abilities,
            }
        } else {
            CreatureCard {
                power: 0,
                toughness: 0,
                abilities: vec![],
            }
        },
        creature_type.clone().unwrap_or(CreatureType::DRAGON),
        // Dragging component
        Draggable {
            dragging: false,
            drag_offset: Vec2::ZERO,
            z_index: 0.0,
        },
    )).id();

    // Spawn text content components with scaled positions
    commands.spawn((
        CardTextContent {
            text: name,
            position: Vec2::new(-card_width * 0.45, card_height * 0.45),
        },
    )).set_parent(card_entity);

    commands.spawn((
        CardTextContent {
            text: cost.to_string(),
            position: Vec2::new(card_width * 0.45, card_height * 0.45),
        },
    )).set_parent(card_entity);

    let type_text = match card_type {
        CardType::CREATURE => {
            if let Some(creature_type) = &creature_type {
                format!("Creature - {}", creature_type_to_string(creature_type))
            } else {
                "Creature".to_string()
            }
        }
        CardType::INSTANT => "Instant".to_string(),
        CardType::ENCHANTMENT => "Enchantment".to_string(),
        CardType::ARTIFACT => "Artifact".to_string(),
        _ => "Unknown".to_string(),
    };

    commands.spawn((
        CardTextContent {
            text: type_text,
            position: Vec2::new(-card_width * 0.45, 0.0),
        },
    )).set_parent(card_entity);

    if let Some(p) = power {
        commands.spawn((
            CardTextContent {
                text: format!("{}/{}", p, toughness.unwrap()),
                position: Vec2::new(card_width * 0.45, -card_height * 0.45),
            },
        )).set_parent(card_entity);
    }

    card_entity
}

fn spawn_hand(mut commands: Commands, window: Query<&Window>) {
    // Safely get window
    let Ok(_window) = window.get_single() else { return };
    let window_height = 800.0; // Use fixed height for consistent card positioning
    let card_spacing = 150.0;
    let start_x = -300.0;
    let base_y = -window_height * 0.3; // Position cards 30% up from the bottom

    // Spawn a hand of iconic MTG cards
    let cards = vec![
        // Lightning Bolt - Red Instant
        (spawn_card(
            &mut commands,
            "Lightning Bolt".to_string(),
            1,
            CardType::INSTANT,
            None,
            None,
            None,
            vec!["Deal 3 damage to any target".to_string()],
            Vec2::new(start_x, base_y),
        ), 0),
        // Black Lotus - Artifact
        (spawn_card(
            &mut commands,
            "Black Lotus".to_string(),
            0,
            CardType::ARTIFACT,
            None,
            None,
            None,
            vec!["Add three mana of any one color".to_string()],
            Vec2::new(start_x + card_spacing, base_y),
        ), 1),
        // Tarmogoyf - Green Creature
        (spawn_card(
            &mut commands,
            "Tarmogoyf".to_string(),
            2,
            CardType::CREATURE,
            Some(CreatureType::DRAGON),
            Some(2),
            Some(3),
            vec!["*/* where * is the number of card types in all graveyards".to_string()],
            Vec2::new(start_x + card_spacing * 2.0, base_y),
        ), 2),
        // Snapcaster Mage - Blue Creature
        (spawn_card(
            &mut commands,
            "Snapcaster Mage".to_string(),
            2,
            CardType::CREATURE,
            Some(CreatureType::WIZARD),
            Some(2),
            Some(1),
            vec!["Flash".to_string(), "When this enters the battlefield, target instant or sorcery card in your graveyard gains flashback".to_string()],
            Vec2::new(start_x + card_spacing * 3.0, base_y),
        ), 3),
        // Dark Confidant - Black Creature
        (spawn_card(
            &mut commands,
            "Dark Confidant".to_string(),
            2,
            CardType::CREATURE,
            Some(CreatureType::HUMAN),
            Some(2),
            Some(1),
            vec!["At the beginning of your upkeep, reveal the top card of your library and put that card into your hand. You lose life equal to its converted mana cost.".to_string()],
            Vec2::new(start_x + card_spacing * 4.0, base_y),
        ), 4),
    ];

    // Add InHand component to each card
    for (entity, index) in cards {
        commands.entity(entity).insert(InHand { index });
    }
}

fn spawn_card_text(
    mut commands: Commands,
    text_content_query: Query<(Entity, &CardTextContent, &Parent), Without<SpawnedText>>,
    draggable_query: Query<&Draggable>,
) {
    for (content_entity, content, parent) in text_content_query.iter() {
        // Get the parent card's z-index, defaulting to 0.0 if not found
        let parent_z_index = draggable_query.get(parent.get()).map_or(0.0, |d| d.z_index);
        let parent_entity = parent.get();
        
        // Store the initial offset in the CardText component
        let offset = content.position;

        // Adjust scale based on text type
        let scale = if offset.y > 0.0 {
            0.3  // Title and mana cost
        } else if offset.y < 0.0 {
            0.4  // Power/Toughness
        } else {
            0.25  // Type line and abilities
        };

        // Create a sprite for text representation
        commands.spawn((
            // Core sprite components
            Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(content.text.len() as f32 * 10.0 * scale, 20.0 * scale)),
                ..default()
            },
            Transform::from_xyz(content.position.x, content.position.y, parent_z_index + 0.1),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            // Custom components
            CardText { 
                offset,
                content: content.text.clone(),
            },
            SpawnedText,
        )).set_parent(parent_entity);

        // Remove the CardTextContent entity since we don't need it anymore
        commands.entity(content_entity).despawn();
    }
}

fn handle_drag_and_text(
    mut card_query: Query<(Entity, &mut Draggable, &mut Transform)>,
    mut text_query: Query<(&mut Transform, &Parent, &CardText), (With<CardText>, Without<Draggable>)>,
    mouse_button: Res<bevy::input::ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    // Safely get window and camera
    let Ok(window) = windows.get_single() else { return };
    let Ok((camera, camera_transform)) = camera.get_single() else { return };

    let card_width = 100.0;
    let card_height = card_width * 1.4;

    // Get mouse position in world coordinates
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Handle mouse down
            if mouse_button.just_pressed(MouseButton::Left) {
                // Find the highest z-index currently in use
                let mut max_z_index: f32 = 0.0;
                for (_, draggable, _) in card_query.iter() {
                    max_z_index = max_z_index.max(draggable.z_index);
                }

                // First pass: find the topmost card under the cursor
                let mut topmost_card: Option<(Entity, f32)> = None;
                for (entity, draggable, transform) in card_query.iter() {
                    let card_bounds = Rect::from_center_size(
                        transform.translation.truncate(),
                        Vec2::new(card_width, card_height),
                    );
                    if card_bounds.contains(world_position) {
                        if let Some((_, current_z)) = topmost_card {
                            if draggable.z_index > current_z {
                                topmost_card = Some((entity, draggable.z_index));
                            }
                        } else {
                            topmost_card = Some((entity, draggable.z_index));
                        }
                    }
                }

                // Second pass: if we found a card, make it draggable
                if let Some((target_entity, _)) = topmost_card {
                    if let Ok((_, mut draggable, transform)) = card_query.get_mut(target_entity) {
                        draggable.dragging = true;
                        draggable.drag_offset = world_position - transform.translation.truncate();
                        draggable.z_index = max_z_index + 1.0;
                    }

                    // Update text z-indices
                    for (mut text_transform, text_parent, _) in text_query.iter_mut() {
                        if text_parent.get() == target_entity {
                            text_transform.translation.z = max_z_index + 1.1;
                        }
                    }
                }
            }

            // Handle mouse up
            if mouse_button.just_released(MouseButton::Left) {
                for (_, mut draggable, _) in card_query.iter_mut() {
                    draggable.dragging = false;
                }
            }

            // Handle dragging
            for (entity, draggable, mut transform) in card_query.iter_mut() {
                if draggable.dragging {
                    let new_pos = world_position - draggable.drag_offset;
                    transform.translation = new_pos.extend(draggable.z_index);

                    // Update text positions using stored offsets
                    for (mut text_transform, text_parent, card_text) in text_query.iter_mut() {
                        if text_parent.get() == entity {
                            text_transform.translation = (new_pos + card_text.offset).extend(draggable.z_index + 0.1);
                        }
                    }
                }
            }
        }
    }
}

fn handle_exit(
    mut exit_events: EventReader<AppExit>,
) {
    for _exit_event in exit_events.read() {
        println!("Received exit event, cleaning up...");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800.0, 600.0).into(),
                title: "Rummage".to_string(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (
            setup_camera,
            spawn_hand,
            spawn_card_text.after(spawn_hand),
        ))
        .add_systems(Update, (
            hello_world,
            handle_drag_and_text,
            handle_window_resize,
            handle_exit,
        ))
        .run();
}
