# State-Based Actions

This document provides an overview of state-based actions in Magic: The Gathering. For the detailed implementation in Rummage, please see [State-Based Actions](../mtg_core/state_actions/index.md).

## What Are State-Based Actions?

State-based actions are game rules that are continuously checked and automatically applied whenever a player would receive priority. They handle common game situations without requiring explicit triggers.

## Key State-Based Actions

The most common state-based actions include:

### Creature-Related

- A creature with toughness ≤ 0 is put into its owner's graveyard
- A creature with lethal damage marked on it is destroyed
- A creature that has been dealt damage by a source with deathtouch is destroyed
- A creature with toughness greater than 0 and damage marked on it equal to or greater than its toughness has lethal damage

### Player-Related

- A player with 0 or less life loses the game
- A player who attempted to draw from an empty library loses the game
- A player with 10 or more poison counters loses the game
- A player who has attempted to draw more cards than their library contains loses the game

### Card-Related

- If two or more legendary permanents with the same name are on the battlefield, their owners choose one to keep and put the rest into their owners' graveyards
- If two or more planeswalkers with the same planeswalker type are on the battlefield, their controllers choose one to keep and put the rest into their owners' graveyards
- An Aura attached to an illegal object or player, or not attached to an object or player, is put into its owner's graveyard
- An Equipment or Fortification attached to an illegal permanent becomes unattached
- A token that has left the battlefield ceases to exist
- A copy of a spell in a zone other than the stack ceases to exist

## When State-Based Actions Are Checked

State-based actions are checked:

1. Whenever a player would receive priority
2. After a spell or ability resolves
3. After combat damage is dealt
4. During cleanup step

They are NOT checked during the resolution of a spell or ability, or during the process of casting a spell or activating an ability.

## Multiple State-Based Actions

If multiple state-based actions would apply simultaneously:

1. All applicable state-based actions are performed simultaneously
2. The system then checks again to see if any new state-based actions need to be performed
3. This process repeats until no more state-based actions are applicable

## Implementation Note

For the detailed implementation of state-based actions in Rummage, including code examples and integration with other systems, see:

- [State-Based Actions Implementation](../mtg_core/state_actions/index.md): Core implementation details
- [Format-Specific State-Based Actions](../formats/commander/game_mechanics/state_based_actions.md): Commander format extensions 