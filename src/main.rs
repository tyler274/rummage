mod card;
mod cards;
mod mana;
mod player;

use bevy::prelude::*;
use bevy::math::Rect;
use bevy::window::WindowResized;
// Cursor remember you need to use Text2d, because text2dbundle is deprecated
use bevy::text::{JustifyText, Text2d, TextColor, TextFont, TextLayout};
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
    offset: Vec2,
}

#[derive(Component)]
struct CardTextContent {
    text: String,
    text_type: CardTextType,
}

#[derive(Component)]
struct SpawnedText;

#[derive(Component)]
struct InHand;

fn hello_world() {
    println!("hello world!");
    println!("Mana default color is: {:?}", Mana::default());
}

fn setup_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 1.0;
    projection.near = -1000.0;
    projection.far = 1000.0;

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        projection,
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
            let vertical_size = 600.0; // Match the spawn_hand height
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
            text_type: CardTextType::Name,
        },
    )).set_parent(card_entity);

    commands.spawn((
        CardTextContent {
            text: cost.to_string(),
            text_type: CardTextType::Cost,
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
            text_type: CardTextType::Type,
        },
    )).set_parent(card_entity);

    if let Some(p) = power {
        commands.spawn((
            CardTextContent {
                text: format!("{}/{}", p, toughness.unwrap()),
                text_type: CardTextType::PowerToughness,
            },
        )).set_parent(card_entity);
    }

    card_entity
}

fn spawn_hand(mut commands: Commands, window: Query<&Window>) {
    // Safely get window
    let Ok(_window) = window.get_single() else { return };
    let window_height = 600.0; // Reduced height for better visibility
    let card_spacing = 120.0; // Reduced spacing
    let start_x = -200.0; // Moved cards more towards center
    let base_y = -window_height * 0.25; // Position cards 25% up from the bottom

    // Spawn a hand of iconic MTG cards
    let cards = vec![
        // Lightning Bolt - Red Instant
        spawn_card(
            &mut commands,
            "Lightning Bolt".to_string(),
            1,
            CardType::INSTANT,
            None,
            None,
            None,
            vec!["Deal 3 damage to any target".to_string()],
            Vec2::new(start_x, base_y),
        ),
        // Black Lotus - Artifact
        spawn_card(
            &mut commands,
            "Black Lotus".to_string(),
            0,
            CardType::ARTIFACT,
            None,
            None,
            None,
            vec!["Add three mana of any one color".to_string()],
            Vec2::new(start_x + card_spacing, base_y),
        ),
        // Tarmogoyf - Green Creature
        spawn_card(
            &mut commands,
            "Tarmogoyf".to_string(),
            2,
            CardType::CREATURE,
            Some(CreatureType::DRAGON),
            Some(2),
            Some(3),
            vec!["*/* where * is the number of card types in all graveyards".to_string()],
            Vec2::new(start_x + card_spacing * 2.0, base_y),
        ),
        // Snapcaster Mage - Blue Creature
        spawn_card(
            &mut commands,
            "Snapcaster Mage".to_string(),
            2,
            CardType::CREATURE,
            Some(CreatureType::WIZARD),
            Some(2),
            Some(1),
            vec!["Flash".to_string(), "When this enters the battlefield, target instant or sorcery card in your graveyard gains flashback".to_string()],
            Vec2::new(start_x + card_spacing * 3.0, base_y),
        ),
        // Dark Confidant - Black Creature
        spawn_card(
            &mut commands,
            "Dark Confidant".to_string(),
            2,
            CardType::CREATURE,
            Some(CreatureType::HUMAN),
            Some(2),
            Some(1),
            vec!["At the beginning of your upkeep, reveal the top card of your library and put that card into your hand. You lose life equal to its converted mana cost.".to_string()],
            Vec2::new(start_x + card_spacing * 4.0, base_y),
        ),
    ];

    // Add InHand component to each card
    for entity in cards {
        commands.entity(entity).insert(InHand);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CardTextType {
    Name,
    Cost,
    Type,
    PowerToughness,
}

// Add debug visualization for card bounds
fn spawn_debug_bounds(commands: &mut Commands, card_pos: Vec2, card_size: Vec2, text_pos: Vec2) {
    // Card center marker
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0), // Red
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        Transform::from_xyz(card_pos.x, card_pos.y, 100.0),
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));

    // Text position marker
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0), // Green
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        Transform::from_xyz(text_pos.x, text_pos.y, 100.0),
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

// Cursor remember you need to use Text2d, because Text2dBundle is deprecated in Bevy 0.15.x
fn spawn_card_text(
    mut commands: Commands,
    text_content_query: Query<(Entity, &CardTextContent, &Parent), (Without<SpawnedText>, With<CardTextContent>)>,
    card_query: Query<(&Transform, &Sprite), With<Card>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (content_entity, content, parent) in text_content_query.iter() {
        let parent_entity = parent.get();
        
        if let Ok((card_transform, sprite)) = card_query.get(parent_entity) {
            let card_size = sprite.custom_size.unwrap_or(Vec2::new(100.0, 140.0));
            
            // Calculate relative offsets from card center
            let (offset, font_size, alignment) = match content.text_type {
                CardTextType::Name => (
                    Vec3::new(0.0, card_size.y * 0.35, 1.0),
                    16.0,
                    JustifyText::Center
                ),
                CardTextType::Cost => (
                    Vec3::new(-card_size.x * 0.35, card_size.y * 0.35, 1.0),
                    16.0,
                    JustifyText::Left
                ),
                CardTextType::Type => (
                    Vec3::new(0.0, 0.0, 1.0),
                    14.0,
                    JustifyText::Center
                ),
                CardTextType::PowerToughness => (
                    Vec3::new(-card_size.x * 0.35, -card_size.y * 0.35, 1.0),
                    16.0,
                    JustifyText::Left
                ),
            };

            // Create text entity with relative transform
            let text_entity = commands.spawn((
                Text2d::new(content.text.clone()),
                TextFont {
                    font: font.clone(),
                    font_size,
                    ..default()
                },
                TextColor(if content.text_type == CardTextType::Type {
                    Color::srgb(0.3, 0.3, 0.3)
                } else {
                    Color::BLACK
                }),
                TextLayout::new_with_justify(alignment),
                Transform::from_translation(offset),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                CardText { offset: offset.truncate() },
                SpawnedText,
            )).id();

            // Set up parent-child relationship
            commands.entity(parent_entity).add_child(text_entity);
            commands.entity(content_entity).insert(SpawnedText);

            // Add debug visualization
            spawn_debug_bounds(&mut commands, 
                card_transform.translation.truncate(),
                card_size,
                card_transform.translation.truncate() + offset.truncate()
            );
        }
    }
}

fn handle_drag_and_text(
    mut card_query: Query<(Entity, &mut Draggable, &mut Transform), With<Card>>,
    mouse_button: Res<bevy::input::ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
) {
    let Ok(window) = windows.get_single() else { return };
    let Ok((camera, camera_transform)) = camera.get_single() else { return };

    let card_width = 100.0;
    let card_height = card_width * 1.4;

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            if mouse_button.just_pressed(MouseButton::Left) {
                let mut max_z_index: f32 = 0.0;
                for (_, draggable, _) in card_query.iter() {
                    max_z_index = max_z_index.max(draggable.z_index);
                }

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

                if let Some((target_entity, _)) = topmost_card {
                    if let Ok((_, mut draggable, transform)) = card_query.get_mut(target_entity) {
                        draggable.dragging = true;
                        draggable.drag_offset = world_position - transform.translation.truncate();
                        draggable.z_index = max_z_index + 1.0;
                    }
                }
            }

            if mouse_button.just_released(MouseButton::Left) {
                for (_, mut draggable, _) in card_query.iter_mut() {
                    draggable.dragging = false;
                }
            }

            for (entity, draggable, mut card_transform) in card_query.iter_mut() {
                if draggable.dragging {
                    let new_pos = world_position - draggable.drag_offset;
                    card_transform.translation = new_pos.extend(draggable.z_index);

                    // Debug marker for card position
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.0, 0.0, 1.0),
                            custom_size: Some(Vec2::new(3.0, 3.0)),
                            ..default()
                        },
                        Transform::from_xyz(new_pos.x, new_pos.y, 100.0),
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ));
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
                resolution: (1280.0, 720.0).into(),
                title: "Rummage".to_string(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (
            hello_world,
            setup_camera,
            spawn_hand,
            spawn_card_text.after(spawn_hand),
        ))
        .add_systems(Update, (
            handle_drag_and_text,
            handle_window_resize,
            handle_exit,
        ))
        .run();
}
