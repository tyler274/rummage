# Data Structure

This document details the structure of card data in the Rummage database, explaining how card information is organized and stored.

## Card Data Model

The core card data is modeled using a structured format:

### Primary Card Entities

- **CardDefinition**: The fundamental card entry containing all card data
- **CardEdition**: Specific information for a card in a particular set
- **CardFace**: Data for one face of a card (for multi-faced cards)

### Core Card Data

Each card contains these core attributes:

```rust
// Simplified example of core card data structure
pub struct CardDefinition {
    pub oracle_id: Uuid,          // Unique identifier
    pub name: String,             // Card name
    pub mana_cost: Option<String>,// Mana cost string
    pub type_line: String,        // Type line text
    pub oracle_text: String,      // Oracle rules text
    pub colors: Vec<Color>,       // Card colors
    pub color_identity: Vec<Color>,// Color identity
    pub keywords: Vec<String>,    // Keyword abilities
    pub power: Option<String>,    // Power (for creatures)
    pub toughness: Option<String>,// Toughness (for creatures)
    pub loyalty: Option<String>,  // Loyalty (for planeswalkers)
    pub card_faces: Vec<CardFace>,// Multiple faces if applicable
    pub legalities: Legalities,   // Format legalities
    pub reserved: bool,           // On the reserved list?
}
```

## Storage Format

Card data is stored in multiple formats:

- **JSON files**: Static card data stored on disk
- **Binary format**: Optimized runtime representation
- **Database**: For web-based implementations

### JSON Structure

The JSON structure follows a standardized format for interoperability:

```json
{
  "oracle_id": "a3fb7228-e76b-4e96-a40e-20b5fed75685",
  "name": "Lightning Bolt",
  "mana_cost": "{R}",
  "type_line": "Instant",
  "oracle_text": "Lightning Bolt deals 3 damage to any target.",
  "colors": ["R"],
  "color_identity": ["R"],
  "keywords": [],
  "legalities": {
    "standard": "not_legal",
    "modern": "legal",
    "commander": "legal"
  }
}
```

## Parsed Structures

Rules text and other complex fields are parsed into structured data:

### Mana Cost Representation

Mana costs are parsed into a structured format:

```rust
pub struct ManaCost {
    pub generic: u32,            // Generic mana amount
    pub white: u32,              // White mana symbols
    pub blue: u32,               // Blue mana symbols
    pub black: u32,              // Black mana symbols
    pub red: u32,                // Red mana symbols
    pub green: u32,              // Green mana symbols
    pub colorless: u32,          // Colorless mana symbols
    pub phyrexian: Vec<Color>,   // Phyrexian mana symbols
    pub hybrid: Vec<(Color, Color)>, // Hybrid mana pairs
    pub x: bool,                 // Contains X in cost?
}
```

### Rules Text Parsing

Rules text is parsed into an abstract syntax tree:

```rust
pub enum RulesTextNode {
    Text(String),
    Keyword(KeywordAbility),
    TriggeredAbility {
        trigger: Trigger,
        effect: Effect,
    },
    ActivatedAbility {
        cost: Vec<Cost>,
        effect: Effect,
    },
    StaticAbility(StaticEffect),
}
```

## Card Relationships

The database tracks relationships between cards:

- **Token creators**: Cards that create tokens
- **Meld pairs**: Cards that meld together
- **Companions**: Cards with companion relationships
- **Partners**: Cards with partner abilities
- **Flip sides**: Two sides of double-faced cards

## Indexing and Lookup

The data structure includes optimized indexes for:

- **Name lookup**: Fast retrieval by card name
- **Type lookup**: Finding cards by type
- **Text search**: Finding cards with specific rules text
- **Color lookup**: Finding cards by color identity
- **Format lookup**: Finding cards legal in specific formats

## Data Versioning

The database supports versioning of card data:

- **Oracle updates**: Tracking rules text changes
- **Erratas**: Handling card corrections
- **Set releases**: Managing new card additions
- **Format changes**: Updating format legalities

## Related Documentation

- [Card Attributes](card_attributes.md): Detailed information about card attributes
- [Effect Implementation](../effects/index.md): How card effects are implemented from data
- [Card Database](index.md): Overview of the card database system 