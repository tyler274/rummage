# Special Rules

This section covers special rules and mechanics unique to the Commander format.

## Contents

- [Multiplayer Politics](multiplayer_politics.md) - Voting, deals, and social mechanics
- [Partner Commanders](partner_commanders.md) - Commander partners and related mechanics
- [Commander Ninjutsu](commander_ninjutsu.md) - The Commander Ninjutsu mechanic
- [Commander Death Triggers](commander_death.md) - How commander death and zone changes work
- [Commander-Specific Cards](special_cards.md) - Cards designed specifically for Commander

## Commander-Specific Rules

The special rules section covers Commander-specific rules that are unique to the format:

### Partners and Backgrounds

Partner mechanics allow players to have two commanders. There are several forms:

- **Universal Partners**: Any two commanders with "Partner" can be paired
- **Partner With**: Specific cards that can only partner with their named counterpart
- **Background**: Legendary creatures that "can have a Background" as a second commander
- **Friends Forever**: A variant of Partner that allows pairing any two "Friends Forever" cards

See [Partner Commanders](partner_commanders.md) for implementation details.

### Commander Ninjutsu

Commander Ninjutsu is a variant of the Ninjutsu mechanic that allows a commander to be put onto the battlefield from the command zone by returning an unblocked attacker to hand. This mechanic is currently only found on one card: Yuriko, the Tiger's Shadow.

See [Commander Ninjutsu](commander_ninjutsu.md) for implementation details.

### Commander Death and Zone Changes

In Commander, when a commander would change zones, its owner can choose to move it to the command zone instead. Special rules govern how this interacts with "dies" triggers and other zone-change abilities.

See [Commander Death Triggers](commander_death.md) for implementation details.

### Commander-Specific Cards

Many cards have been designed specifically for the Commander format, including:

- Cards that reference the command zone directly
- Cards with mechanics only found in Commander products (Myriad, Lieutenant, etc.)
- Cards that affect the Commander tax
- Cards designed for multiplayer political play

See [Commander-Specific Cards](special_cards.md) for implementation details.

### Multiplayer Politics

Commander is often played as a multiplayer format, and certain mechanics are designed for multiplayer interactions:

- Voting mechanics
- Deal-making abilities
- Group effects that affect all players
- Incentives and deterrents for attacking

See [Multiplayer Politics](multiplayer_politics.md) and [Politics Testing](politics_testing.md) for implementation details.

## Related Mechanics

Some Commander-specific mechanics interact with other systems:

- [Commander Damage](../combat/commander_damage.md) - For tracking commander combat damage
- [Command Zone](../zones/command_zone.md) - For commander zone transitions
- [Commander Tax](../player_mechanics/commander_tax.md) - For additional costs to cast commanders
- [Color Identity](../player_mechanics/color_identity.md) - For deck construction restrictions

## Testing Special Rules

The [tests](tests/) directory contains test cases for verifying the correct implementation of special Commander rules and edge case handling. 