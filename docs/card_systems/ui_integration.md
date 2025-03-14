# UI Integration

This document outlines how the card systems integrate with the user interface in Rummage, ensuring a seamless and intuitive player experience.

## Integration Architecture

The card systems and UI are integrated through a well-defined interface:

- **Card representations**: Visual representations of cards
- **Interaction handlers**: Systems that handle UI interactions with cards
- **Event propagation**: Bidirectional event flow between UI and game logic
- **State synchronization**: Keeping UI and game state in sync

## Card Visualization

Cards are visualized in the UI through:

- **Card renderer**: Renders cards based on their properties
- **State indicators**: Visual indicators for card states (tapped, summoning sick, etc.)
- **Counter visualization**: Display of counters on cards
- **Effect visualization**: Visual effects for active abilities

## Card Interactions

The UI supports various card interactions:

- **Dragging cards**: Moving cards between zones
- **Clicking cards**: Selecting or activating cards
- **Hovering cards**: Displaying detailed information
- **Targeting**: Selecting targets for spells and abilities
- **Stacking**: Managing stacks of cards in zones

## Implementation Details

The integration uses Bevy's entity component system:

```rust
// Example of a system that handles card selection in the UI
fn handle_card_selection(
    mouse_input: Res<Input<MouseButton>>,
    mut selected_card: ResMut<SelectedCard>,
    card_query: Query<(Entity, &GlobalTransform, &Card, &Interaction)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (entity, transform, card, interaction) in card_query.iter() {
            if matches!(interaction, Interaction::Clicked) {
                selected_card.entity = Some(entity);
                // Trigger UI update for selection
            }
        }
    }
}
```

## Zone Representation

Different zones are represented in the UI:

- **Hand**: Cards arranged in a fan or row
- **Battlefield**: Grid-based layout of permanent cards
- **Graveyard**: Stack of cards with access to view
- **Exile**: Similar to graveyard but visually distinct
- **Stack**: Visual representation of spells and abilities waiting to resolve

## Responsiveness

The UI adapts to different situations:

- **Different screen sizes**: Layouts adjust to available space
- **Varying card counts**: Handles different numbers of cards in zones
- **Game state changes**: Updates in response to game state changes

## Accessibility Considerations

The integration includes accessibility features:

- **Color blindness support**: Uses patterns and symbols in addition to colors
- **Screen reader compatibility**: Cards and states have text descriptions
- **Keyboard navigation**: Full functionality without mouse
- **Customizable settings**: Adjustable text size, contrast, etc.

## Performance Optimization

The UI integration is optimized for performance:

- **Culling**: Only render visible cards
- **Level of detail**: Simplified representations for distant or small cards
- **Asset management**: Efficient loading and unloading of card assets
- **Batched rendering**: Group similar cards for efficient rendering

## Related Documentation

- [Card Rendering](rendering/index.md): How cards are visually rendered
- [Game UI System](../game_gui/index.md): The overall UI system
- [Card Selection](../game_gui/interaction/card_selection.md): UI for selecting cards
- [Drag and Drop](../game_gui/interaction/drag_and_drop.md): UI for dragging cards
- [Targeting System](../game_gui/interaction/targeting.md): UI for targeting 