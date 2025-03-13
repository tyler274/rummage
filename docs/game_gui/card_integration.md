# Card Systems Integration

This document provides a detailed explanation of how the Game UI system integrates with the Card Systems in Rummage.

## Table of Contents

1. [Overview](#overview)
2. [Visualization Pipeline](#visualization-pipeline)
3. [Interaction Translation](#interaction-translation)
4. [Special Card Rendering](#special-card-rendering)
5. [Implementation Examples](#implementation-examples)

## Overview

The Game UI and Card Systems modules work together to create a seamless player experience. The Card Systems module provides the data model and game logic for cards, while the Game UI module translates this data into visual elements and player interactions.

This integration follows these key principles:

1. **Separation of Concerns**: Card data and logic remain in the Card Systems module, while visualization and interaction are handled by the Game UI
2. **Event-Driven Communication**: Changes in card state trigger events that the UI responds to
3. **Bidirectional Flow**: Player interactions with the UI generate events that affect the underlying card systems
4. **Visual Consistency**: Card rendering maintains a consistent visual language across different card types and states

## Visualization Pipeline

The visualization pipeline transforms card data from the Card Systems module into visual elements in the Game UI:

### Card Entity Mapping

Each card entity in the game engine has a corresponding visual entity in the UI:

```rust
fn create_card_visuals(
    mut commands: Commands,
    card_query: Query<(Entity, &Card, &CardName, &CardType), Added<Card>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, card, name, card_type) in &card_query {
        // Create visual entity linked to the card entity
        let visual_entity = commands.spawn((
            // Visual components
            CardVisual { card_entity: entity },
            Sprite {
                custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                ..default()
            },
            SpriteSheetBundle {
                texture: asset_server.load(&get_card_texture_path(card)),
                ..default()
            },
            // UI interaction components
            Interactable,
            Draggable,
            // Transform for positioning
            Transform::default(),
            GlobalTransform::default(),
        )).id();
        
        // Link the card entity to its visual representation
        commands.entity(entity).insert(VisualRepresentation(visual_entity));
    }
}
```

### State Change Propagation

When card state changes in the game engine, these changes are reflected in the UI:

```rust
fn update_card_visuals(
    mut commands: Commands,
    card_query: Query<(Entity, &VisualRepresentation, &ZoneType, Option<&Tapped>), Changed<ZoneType>>,
    mut visual_query: Query<(&mut Transform, &mut Visibility)>,
) {
    for (card_entity, visual_rep, zone, tapped) in &card_query {
        if let Ok((mut transform, mut visibility)) = visual_query.get_mut(visual_rep.0) {
            // Update position based on zone
            match zone {
                ZoneType::Battlefield => {
                    visibility.is_visible = true;
                    // Position on battlefield
                },
                ZoneType::Hand => {
                    visibility.is_visible = true;
                    // Position in hand
                },
                ZoneType::Library | ZoneType::Graveyard | ZoneType::Exile => {
                    // Position in appropriate zone
                },
                _ => {
                    visibility.is_visible = false;
                }
            }
            
            // Update rotation based on tapped state
            if tapped.is_some() {
                transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
            } else {
                transform.rotation = Quat::default();
            }
        }
    }
}
```

### Visual Effects

The UI adds visual effects to represent game actions:

```rust
fn add_cast_visual_effect(
    mut commands: Commands,
    mut cast_events: EventReader<CastSpellEvent>,
    card_query: Query<&VisualRepresentation>,
) {
    for event in cast_events.iter() {
        if let Ok(visual_rep) = card_query.get(event.card) {
            // Add casting visual effect to the card
            commands.entity(visual_rep.0).insert(CastingEffect {
                duration: 0.5,
                elapsed: 0.0,
            });
        }
    }
}
```

## Interaction Translation

The UI translates player interactions into game actions:

### Drag and Drop

```rust
fn handle_card_drag_end(
    mut commands: Commands,
    mut drag_events: EventReader<DragEndEvent>,
    card_visuals: Query<&CardVisual>,
    zones: Query<(Entity, &ZoneType, &DropTarget)>,
    mut zone_transfer_events: EventWriter<ZoneTransferEvent>,
) {
    for event in drag_events.iter() {
        if let Ok(card_visual) = card_visuals.get(event.entity) {
            // Find the zone the card was dropped on
            for (zone_entity, zone_type, drop_target) in &zones {
                if drop_target.contains(event.position) {
                    // Send zone transfer event
                    zone_transfer_events.send(ZoneTransferEvent {
                        card: card_visual.card_entity,
                        target_zone: zone_entity,
                        target_zone_type: *zone_type,
                    });
                    break;
                }
            }
        }
    }
}
```

### Card Selection

```rust
fn handle_card_selection(
    mut commands: Commands,
    mut click_events: EventReader<ClickEvent>,
    card_visuals: Query<&CardVisual>,
    mut selection_events: EventWriter<CardSelectionEvent>,
) {
    for event in click_events.iter() {
        if let Ok(card_visual) = card_visuals.get(event.entity) {
            // Send selection event
            selection_events.send(CardSelectionEvent {
                card: card_visual.card_entity,
                selection_type: if event.button == MouseButton::Left {
                    SelectionType::Primary
                } else {
                    SelectionType::Secondary
                },
            });
        }
    }
}
```

### Context Menus

```rust
fn show_card_context_menu(
    mut commands: Commands,
    mut right_click_events: EventReader<RightClickEvent>,
    card_visuals: Query<&CardVisual>,
    cards: Query<(&Card, &CardType, &ZoneType)>,
) {
    for event in right_click_events.iter() {
        if let Ok(card_visual) = card_visuals.get(event.entity) {
            if let Ok((card, card_type, zone)) = cards.get(card_visual.card_entity) {
                // Create context menu based on card type and zone
                let menu_entity = commands.spawn((
                    ContextMenu {
                        card: card_visual.card_entity,
                        position: event.screen_position,
                    },
                    // UI components for the menu
                )).id();
                
                // Add appropriate actions based on card type and zone
                add_context_menu_actions(
                    &mut commands, 
                    menu_entity, 
                    card_visual.card_entity, 
                    card_type, 
                    zone
                );
            }
        }
    }
}
```

## Special Card Rendering

Some cards require special rendering treatment:

### Modal Cards

```rust
fn show_modal_card_options(
    mut commands: Commands,
    mut cast_events: EventReader<CastModalSpellEvent>,
    cards: Query<&ModalOptions>,
) {
    for event in cast_events.iter() {
        if let Ok(modal_options) = cards.get(event.card) {
            // Create modal selection UI
            let modal_ui = commands.spawn((
                ModalSelectionUI {
                    card: event.card,
                    options: modal_options.options.clone(),
                },
                // UI components
            )).id();
            
            // Add option buttons
            for (i, option) in modal_options.options.iter().enumerate() {
                commands.spawn((
                    ModalOptionButton {
                        parent: modal_ui,
                        option_index: i,
                    },
                    // UI components
                    // Text component with option description
                ));
            }
        }
    }
}
```

### Split Cards

```rust
fn render_split_card(
    mut commands: Commands,
    split_cards: Query<(Entity, &SplitCard, &VisualRepresentation)>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, split_card, visual_rep) in &split_cards {
        // Load both halves of the split card
        let left_texture = asset_server.load(&split_card.left_half_texture);
        let right_texture = asset_server.load(&split_card.right_half_texture);
        
        // Create a special sprite that combines both halves
        commands.entity(visual_rep.0).insert(SplitCardVisual {
            left_half: left_texture,
            right_half: right_texture,
        });
    }
}
```

## Implementation Examples

### Card Hover Preview

```rust
fn card_hover_preview(
    mut commands: Commands,
    hover_events: EventReader<HoverEvent>,
    card_visuals: Query<&CardVisual>,
    cards: Query<(&Card, &CardName)>,
    mut preview_query: Query<Entity, With<CardPreview>>,
) {
    // Remove existing previews
    for preview_entity in &preview_query {
        commands.entity(preview_entity).despawn_recursive();
    }
    
    // Create new preview for hovered card
    for event in hover_events.iter() {
        if let Ok(card_visual) = card_visuals.get(event.entity) {
            if let Ok((card, name)) = cards.get(card_visual.card_entity) {
                commands.spawn((
                    CardPreview,
                    // Large card image
                    ImageBundle {
                        image: UiImage::new(asset_server.load(&get_card_preview_texture(card))),
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(event.screen_position.x + 20.0),
                                top: Val::Px(event.screen_position.y - 200.0),
                                ..default()
                            },
                            size: Size::new(Val::Px(240.0), Val::Px(336.0)),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        }
    }
}
```

### Battlefield Layout

```rust
fn organize_battlefield_cards(
    mut commands: Commands,
    players: Query<(Entity, &Player)>,
    battlefield: Query<Entity, With<BattlefieldZone>>,
    cards_in_battlefield: Query<(Entity, &VisualRepresentation), (With<Card>, With<ZoneType>)>,
    mut card_visuals: Query<&mut Transform>,
) {
    if let Ok(battlefield_entity) = battlefield.get_single() {
        // Group cards by controller
        let mut player_cards: HashMap<Entity, Vec<Entity>> = HashMap::new();
        
        for (card_entity, visual_rep) in &cards_in_battlefield {
            if let Some(controller) = get_card_controller(card_entity) {
                player_cards.entry(controller)
                    .or_insert_with(Vec::new)
                    .push(visual_rep.0);
            }
        }
        
        // Position each player's cards in their battlefield section
        for (player_entity, player) in &players {
            if let Some(visual_entities) = player_cards.get(&player_entity) {
                let player_section = get_player_battlefield_section(player_entity, &players);
                
                for (i, &visual_entity) in visual_entities.iter().enumerate() {
                    if let Ok(mut transform) = card_visuals.get_mut(visual_entity) {
                        // Calculate position within player's section
                        let row = i / CARDS_PER_ROW;
                        let col = i % CARDS_PER_ROW;
                        
                        transform.translation = Vec3::new(
                            player_section.min.x + col as f32 * (CARD_WIDTH + CARD_SPACING),
                            player_section.min.y + row as f32 * (CARD_HEIGHT + CARD_SPACING),
                            0.0,
                        );
                    }
                }
            }
        }
    }
}
```

These examples demonstrate the tight integration between the Game UI and Card Systems modules, showing how they work together to create a cohesive player experience while maintaining a clean separation of concerns.

---

For more information on the Card Systems module, see the [Card Systems documentation](../card_systems/index.md). 