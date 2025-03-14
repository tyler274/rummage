# Magic: The Gathering Rules

This section documents the implementation of Magic: The Gathering rules in Rummage.

## Rules Implementation

The MTG rules are implemented according to the official Magic: The Gathering Comprehensive Rules. We strive to accurately represent all game mechanics while providing a clear and maintainable codebase.

## Core Mechanics

- [Turn Structure](turn_structure.md): Phases and steps of a turn
- [Stack](stack.md): How spells and abilities are processed
- [Targeting](targeting.md): Rules for selecting targets
- [Card States](card_states.md): Different states cards can have
- [Mana and Costs](mana_costs.md): How mana works and costs are paid
- [Combat](combat.md): Attack, blocking, and damage rules

## Special Mechanics

- [Triggered Abilities](triggered_abilities.md): Events that cause abilities to trigger
- [Replacement Effects](replacement_effects.md): Effects that modify how events occur
- [State-Based Actions](state_based_actions.md): Automatic game actions

## Edge Cases

Magic: The Gathering has many complex interactions and edge cases. We document these scenarios and how they are handled in the engine.

## Rules References

For the official rules reference, see the [Magic: The Gathering Comprehensive Rules](https://magic.wizards.com/en/rules).
