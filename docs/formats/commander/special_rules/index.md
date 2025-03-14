# Special Rules

This section covers special rules and mechanics unique to the Commander format.

## Contents

- [Color Identity](color_identity.md) - Implementation of color identity restrictions
- [Special Cards](special_cards.md) - Commander-specific card mechanics
- [Partner and Background](partner_background.md) - Special commander partner mechanics
- [Multiplayer Politics](multiplayer_politics.md) - Voting, deals, and social mechanics

## Commander-Specific Rules

The special rules section covers Commander-specific rules not covered in other sections:

- Color identity implementation for deck validation
- Partner commander mechanics
- Background mechanic from Baldur's Gate
- Commander-specific card types and abilities
- Handling of banned and restricted cards
- Multiplayer political mechanics (voting, deal-making, etc.)

These special rules are what make Commander unique among Magic: The Gathering formats and require specific implementation details to capture the essence of the format.

## Related Mechanics

Some Commander-specific mechanics interact with other systems:

- [Random Mechanics](../game_mechanics/random_mechanics.md) - For coin flips and dice rolls in Commander cards
- [Commander Damage](../combat/commander_damage.md) - For tracking commander combat damage
- [Command Zone](../zones/command_zone.md) - For commander zone transitions

## Testing

The [tests](tests/) directory contains test cases for verifying the correct implementation of special Commander rules and edge case handling. 