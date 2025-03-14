# Commander-Specific Rules

This page provides a high-level reference for the official Commander format rules. For detailed implementation specifics, see the [Commander Format](../formats/commander/index.md) documentation.

## What Is Commander?

Commander (formerly known as Elder Dragon Highlander or EDH) is a multiplayer format for Magic: The Gathering created by players and embraced by Wizards of the Coast as an official format. It emphasizes social gameplay and creative deck building around a chosen legendary creature.

## Official Commander Rules

The Commander format is governed by a specific set of rules maintained by the Commander Rules Committee:

### Deck Construction Rules

- Players choose a legendary creature as their "commander"
- A deck contains exactly 100 cards, including the commander
- Except for basic lands, no two cards in the deck may have the same English name
- A card's color identity must be within the color identity of the commander
- Color identity includes colored mana symbols in costs and rules text

### Game Play Rules

- Players begin with 40 life
- Commanders begin the game in the "command zone"
- While a commander is in the command zone, it may be cast, subject to normal timing restrictions
- Each time a player casts their commander from the command zone, it costs an additional {2} for each previous time they've cast it
- If a commander would be exiled or put into a hand, graveyard, or library, its owner may choose to move it to the command zone instead
- A player that has been dealt 21 or more combat damage by the same commander loses the game

## Rule Implementation References

For detailed implementation of these rules in Rummage, refer to these sections:

- [Color Identity](../formats/commander/player_mechanics/color_identity.md) - Implementation of color identity rules
- [Command Zone](../formats/commander/zones/command_zone.md) - How the command zone functions
- [Commander Tax](../formats/commander/player_mechanics/commander_tax.md) - Implementation of additional commander casting costs
- [Commander Damage](../formats/commander/combat/commander_damage.md) - Tracking and applying commander combat damage
- [Zone Transitions](../formats/commander/zones/zone_transitions.md) - Commander movement between zones

## Commander Variants

Rummage supports these common Commander variants:

### Partner Commanders

Some legendary creatures have the "Partner" ability, allowing a deck to have two commanders. See [Partner Commanders](../formats/commander/special_rules/partner_commanders.md) for implementation details.

### Commander Ninjutsu

A special ability that allows commanders to be put onto the battlefield from the command zone. See [Commander Ninjutsu](../formats/commander/special_rules/commander_ninjutsu.md) for implementation details.

### Brawl

A variant with 60-card decks, only using Standard-legal cards and starting at 25 life.

### Commander Death Triggers

Special rules for how commander death and zone changes work. See [Commander Death Triggers](../formats/commander/special_rules/commander_death.md) for implementation details.

## Commander-Specific Cards

Many cards have been designed specifically for the Commander format. See [Commander-Specific Cards](../formats/commander/special_rules/special_cards.md) for details on how these cards are implemented.

## References

- [Official Commander Rules](https://mtgcommander.net/index.php/rules/)
- [Commander Format on MTG Wiki](https://mtg.fandom.com/wiki/Commander_(format))
- [Wizards of the Coast Commander Page](https://magic.wizards.com/en/formats/commander) 