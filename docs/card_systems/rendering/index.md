# Card Rendering

Card rendering is responsible for the visual representation of Magic: The Gathering cards in Rummage. This system handles how cards are displayed in different zones, states, and views.

## Rendering Philosophy

The card rendering system adheres to several key principles:

- **Accuracy**: Cards should resemble their physical counterparts when appropriate
- **Readability**: Card information should be easily readable at various scales
- **Performance**: Rendering should be efficient, even with many cards on screen
- **Flexibility**: The system should adapt to different screen sizes and device capabilities

## Implementation

Card rendering is implemented using Bevy's rendering capabilities:

- **Entity-Component-System**: Cards are entities with rendering components
- **Material system**: Custom shaders for card effects
- **Texture atlases**: Efficient texture management for card art and symbols
- **Text rendering**: High-quality text rendering for card text

## Card Layout

Cards are rendered with several key layout elements:

- **Frame**: The card border and background
- **Art box**: The card's artwork
- **Title bar**: The card's name and mana cost
- **Type line**: Card types, subtypes, and supertypes
- **Text box**: Rules text and flavor text
- **Power/toughness box**: For creatures
- **Loyalty counter**: For planeswalkers
- **Set symbol**: Indicating rarity and expansion

## Art Assets

The rendering system uses various art assets:

- **Card artwork**: The main illustration for each card
- **Frame templates**: Card frames for different card types
- **Mana symbols**: Symbols representing mana costs
- **Set symbols**: Symbols for different expansions
- **UI elements**: Additional interface elements for card interactions

## Special Cases

The system handles special rendering cases such as:

- **Double-faced cards**: Cards with two sides
- **Split cards**: Cards with two halves
- **Adventure cards**: Cards with an adventure component
- **Leveler cards**: Cards with level-up abilities
- **Saga cards**: Cards with chapter abilities
- **Planeswalkers**: Cards with loyalty abilities

## Dynamic Rendering

Cards dynamically adjust their rendering based on:

- **Zone**: Different visual treatment in hand, battlefield, etc.
- **State**: Tapped, attacking, blocking, etc.
- **Modifications**: +1/+1 counters, auras attached, etc.
- **Focus**: Card under mouse hover or selection

## Related Documentation

- [Art Assets](art_assets.md): Details on the art assets used for cards
- [Card Layout](card_layout.md): Specifics of card layout and design
- [UI Integration](../ui_integration.md): How card rendering integrates with the UI
- [Card Database](../database/index.md): Source of card data for rendering 