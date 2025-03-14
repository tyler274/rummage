# Card Attributes

This document details the attributes tracked for each card in the Rummage database. These attributes define a card's characteristics, behavior, and appearance.

## Core Attributes

### Identification

- **Oracle ID**: Unique identifier for the card's Oracle text
- **Name**: The card's name
- **Set Code**: The three or four-letter code for the card's set
- **Collector Number**: The card's number within its set
- **Rarity**: Common, uncommon, rare, mythic rare, or special

### Card Type Information

- **Type Line**: The full type line text
- **Supertypes**: Legendary, Basic, Snow, etc.
- **Card Types**: Creature, Instant, Sorcery, etc.
- **Subtypes**: Human, Wizard, Equipment, Aura, etc.

### Mana and Color

- **Mana Cost**: The card's mana cost
- **Mana Value**: The converted mana cost (total mana)
- **Colors**: The card's colors
- **Color Identity**: Colors in mana cost and rules text
- **Color Indicator**: For cards with no mana cost but have a color

### Rules Text

- **Oracle Text**: The official rules text
- **Flavor Text**: The card's flavor text
- **Keywords**: Keyword abilities (Flying, Trample, etc.)
- **Ability Words**: Words that have no rules meaning (Landfall, etc.)

### Combat Stats

- **Power**: Creature's power (attack strength)
- **Toughness**: Creature's toughness (health)
- **Loyalty**: Starting loyalty for planeswalkers
- **Defense**: Defense value for battles

### Legality

- **Format Legality**: Legal status in various formats
- **Reserved List**: Whether the card is on the Reserved List
- **Banned/Restricted**: Status in various formats

## Special Card Attributes

### Multi-faced Cards

- **Card Faces**: Data for each face of the card
- **Layout**: Card layout type (normal, split, flip, etc.)
- **Face Relationship**: How faces relate to each other

### Tokens

- **Token Information**: Data for tokens created by the card
- **Token Colors**: Colors of created tokens
- **Token Types**: Types of created tokens

### Counters

- **Counter Types**: Types of counters the card uses
- **Counter Placement**: Where counters can be placed
- **Counter Effects**: Effects of counters on the card

## Implementation

Card attributes are implemented as components in the ECS:

```rust
// Example of card type components
#[derive(Component)]
pub struct CardName(pub String);

#[derive(Component)]
pub struct ManaCost {
    pub cost_string: String,
    pub parsed: ParsedManaCost,
}

#[derive(Component)]
pub struct CardTypes {
    pub supertypes: Vec<Supertype>,
    pub card_types: Vec<CardType>,
    pub subtypes: Vec<Subtype>,
}

#[derive(Component)]
pub struct OracleText(pub String);

#[derive(Component)]
pub struct PowerToughness {
    pub power: String,
    pub toughness: String,
}
```

## Attribute Modification

Card attributes can be modified by:

- **Static Effects**: Continuous modifications
- **One-time Effects**: Temporary changes
- **Counters**: Modifications from counters
- **State-Based Actions**: Automatic modifications

## Attribute Access

Systems access card attributes through queries:

```rust
// Example of a system that queries card attributes
fn check_creature_types(
    creatures: Query<(Entity, &CardTypes, &CardName)>,
) {
    for (entity, card_types, name) in creatures.iter() {
        if card_types.card_types.contains(&CardType::Creature) {
            // Process creature card
            let creature_subtypes = &card_types.subtypes;
            // ...
        }
    }
}
```

## Attribute Serialization

Attributes are serialized for:

- **Persistence**: Saving game state
- **Networking**: Transmitting card data
- **UI Display**: Showing card information

## Related Documentation

- [Data Structure](data_structure.md): Overall structure of card data
- [Card Database](index.md): How card data is stored and retrieved
- [Card Rendering](../rendering/index.md): How attributes affect card appearance 