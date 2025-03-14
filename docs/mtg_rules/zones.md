# Game Zones

This document provides an overview of game zones in Magic: The Gathering. For the detailed implementation in Rummage, please see [Zones Implementation](../mtg_core/zones/index.md).

## Overview

Magic: The Gathering divides the game into distinct areas called zones. Each zone has specific rules governing how cards can enter, leave, and interact while in that zone.

## Standard Game Zones

The standard game includes the following zones:

### Library

- The player's deck of cards
- Cards are face-down and in a random order
- Players draw from the top of their library
- Running out of cards in the library can cause a player to lose the game

### Hand

- Cards held by a player
- Normally hidden from opponents
- Maximum hand size (usually seven) is enforced during the cleanup step
- Cards in hand can be cast or played according to their type and timing restrictions

### Battlefield

- Permanents (lands, creatures, artifacts, enchantments, planeswalkers) exist on the battlefield
- Cards on the battlefield are typically face-up (exceptions exist for morphed/manifested cards)
- Cards on the battlefield can tap, untap, attack, block, and be affected by other cards
- Positioning on the battlefield can matter for certain cards and effects

### Graveyard

- Discard pile for destroyed, sacrificed, or discarded cards
- Cards are face-up and can be examined by any player
- Order of cards in the graveyard matters for some effects
- Some abilities allow cards to be played or returned from the graveyard

### Stack

- Holds spells and abilities that have been cast or activated but haven't resolved yet
- Operates as a last-in, first-out (LIFO) data structure
- Spells and abilities resolve one at a time from top to bottom
- Players can respond to objects on the stack by adding more spells or abilities

### Exile

- Cards removed from the game
- Cards are typically face-up unless specified otherwise
- Generally, cards in exile cannot interact with the game
- Some cards can return cards from exile or interact with exiled cards

### Command

- Special zone for format-specific cards or game elements
- In Commander, this zone holds commander cards
- In other formats, it may hold emblems, planes, schemes, etc.
- Cards in the command zone have special rules for how they can be cast or used

## Zone Transitions

Cards can move between zones according to specific rules:

1. **Casting a Spell**: Card moves from hand to the stack
2. **Spell Resolution**: Card moves from stack to battlefield (permanents) or graveyard (instants/sorceries)
3. **Destroying/Sacrificing**: Permanent moves from battlefield to graveyard
4. **Drawing**: Card moves from library to hand
5. **Discarding**: Card moves from hand to graveyard
6. **Exiling**: Card moves from any zone to exile

## Zone Change Rules

When a card changes zones:

- It becomes a new object with no memory of its previous existence
- Counters, attachments, and continuous effects that previously affected it no longer apply
- Exceptions exist for abilities that specifically track the object across zone changes

## Format-Specific Zone Rules

Different formats may modify zone rules:

- **Commander**: The command zone holds commander cards, which can be cast from there
- **Planechase**: The planar deck and active plane exist in the command zone
- **Archenemy**: Scheme cards reside in the command zone

## Related Documentation

For more information about zones in Rummage, see:

- [Zones Implementation](../mtg_core/zones/index.md): Detailed implementation in the core engine
- [Commander Command Zone](../formats/commander/zones/command_zone.md): Commander-specific implementation
- [Zone Transitions](../formats/commander/zones/zone_transitions.md): How cards move between zones
- [Stack Implementation](../mtg_core/stack/index.md): Detailed stack zone implementation 