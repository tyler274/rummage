# Commander-Specific Zone Mechanics

## Overview

This section covers the implementation of Commander-specific zone mechanics in the Rummage game engine. For core zone mechanics that apply to all formats, see the [MTG Core Zones](../../mtg_core/zones/index.md) documentation.

## Commander Zone Extensions

Commander extends the basic MTG zone system by:

1. **Adding the Command Zone** - A special zone where commanders begin the game
2. **Modifying Zone Transition Rules** - Commanders can optionally move to the command zone instead of other zones
3. **Introducing Commander Tax** - Additional mana cost for casting commanders from the command zone

## Contents

- [Command Zone](command_zone.md) - Implementation of the command zone and commander-specific mechanics
- [Zone Transitions](zone_transitions.md) - Special rules for commander movement between zones
- [Zone Management](zone_management.md) - Extended zone implementation for Commander games

## Key Commander Zone Mechanics

- **Commander Placement**: Commanders start in the command zone before the game begins
- **Zone Replacement**: When a commander would be put into a library, hand, graveyard, or exile, its owner may choose to put it into the command zone instead
- **Commander Tax**: Each time a commander is cast from the command zone, it costs an additional {2} for each previous time it has been cast from the command zone
- **Partner Commanders**: Special handling for decks with two commanders (Partner mechanic)
- **Commander Backgrounds**: Implementation of the Background mechanic for certain commanders

These Commander-specific zone mechanics build upon the core zone system to enable the unique gameplay experience of the Commander format.

---

Next: [Command Zone](command_zone.md) 