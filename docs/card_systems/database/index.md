# Card Database

The card database is the central repository of all card information in Rummage. It stores the canonical data for every card in the supported Magic: The Gathering sets.

## Database Structure

The card database is organized to efficiently store and retrieve card data:

- **Card entries**: Core card data including name, cost, types, etc.
- **Card versions**: Variations of cards across different sets
- **Rules text**: Formatted and parsed rules text for each card
- **Card attributes**: Power, toughness, loyalty, etc.
- **Related cards**: Connections between cards (tokens, meld pairs, etc.)

## Data Sources

Card data is sourced from:

- Official Wizards of the Coast databases
- Community-maintained data repositories
- Hand-curated rules interpretations

## Implementation

The database is implemented using a combination of:

- **Static data files**: Core card information stored in structured formats
- **Runtime components**: In-memory representation with efficient lookups
- **Serialization**: Conversion between storage and runtime formats

## Loading and Caching

The database implements efficient loading and caching:

- **Lazy loading**: Only load cards as needed
- **Set-based loading**: Load cards by set for organized play
- **Caching**: Keep frequently used cards in memory

## Card Lookup

Cards can be searched by various criteria:

- **Name**: Exact or fuzzy name matching
- **Mana cost**: Specific mana cost or converted mana cost
- **Type**: Card types, subtypes, and supertypes
- **Text**: Full-text search of rules text
- **Format legality**: Cards legal in specific formats

## Extensibility

The database is designed for extensibility:

- **New sets**: Simple process to add new card sets
- **Custom cards**: Support for user-created cards
- **Format updates**: Easy to update format legality

## Related Documentation

- [Data Structure](data_structure.md): Detailed structure of card data
- [Card Attributes](card_attributes.md): Attributes tracked for each card
- [Card Effects](../effects/index.md): How card effects are implemented
- [Card Rendering](../rendering/index.md): How cards are displayed 