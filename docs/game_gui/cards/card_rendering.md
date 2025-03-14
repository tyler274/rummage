# Card Rendering

The card rendering system is responsible for visualizing Magic the Gathering cards in the game UI.

## Card Components

Cards are rendered using several visual components:

- **Card Frame**: The border and background of the card, varying by card type
- **Art Box**: The image area displaying card artwork
- **Text Box**: The area containing card rules text
- **Type Line**: The line showing card types and subtypes
- **Mana Cost**: Symbols representing the card's casting cost
- **Name Bar**: The area displaying the card's name
- **Stats Box**: For creatures, the power/toughness display

## Rendering Pipeline

Cards are rendered using Bevy's 2D rendering pipeline:

1. Card data is loaded from the game engine
2. Visual components are constructed as Bevy entities
3. Sprites and text elements are positioned according to the card layout
4. Materials and shaders are applied for visual effects

## Optimizations

The rendering system uses several optimizations to maintain performance:

- **Texture Atlases**: Card symbols are packed into texture atlases
- **Dynamic LOD**: Cards further from the camera show simplified versions
- **Batched Rendering**: Similar cards use instanced rendering where possible
- **Caching**: Frequently used card layouts are cached

## Integration

Card rendering integrates with several other systems:

- [Card States](card_states.md): Visual representation changes based on card state
- [Drag and Drop](../interaction/drag_and_drop.md): Visual feedback during dragging
- [Card Animations](card_animations.md): Smooth transitions between visual states

## Example Usage

```rust
// Create a card render entity
commands.spawn((
    SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    },
    CardRender {
        card_entity: card_entity,
        frame_type: FrameType::Creature,
        foil: false,
    },
));
```

For information on how cards integrate with the gameplay systems, see [Card Integration](../card_integration.md). 