# Magic: The Gathering Comprehensive Rules

This page provides an overview of the Magic: The Gathering Comprehensive Rules and how they are implemented in the Rummage game engine.

## Introduction

The Magic: The Gathering Comprehensive Rules are the complete ruleset for the game, covering all possible interactions and edge cases. The current version used in Rummage is dated February 7, 2025, and can be found in [MagicCompRules 20250207.txt](MagicCompRules%2020250207.txt).

## Implementation Approach

Rummage implements the Comprehensive Rules through a modular system of components and systems. Each section of the rules is mapped to specific game logic:

1. **Game Concepts (Rules 100-199)**
   - Core game logic, turn structure, and win conditions
   - Starting the game, ending the game
   - Colors, mana, and basic card types

2. **Parts of a Card (Rules 200-299)**
   - Card data structures
   - Card characteristics and attributes
   - Card types and subtypes

3. **Card Types (Rules 300-399)**
   - Type-specific behaviors (creatures, artifacts, etc.)
   - Type-changing effects
   - Supertype rules (legendary, basic, etc.)

4. **Zones (Rules 400-499)**
   - Zone implementation
   - Movement between zones
   - Zone-specific rules

5. **Turn Structure (Rules 500-599)**
   - Phase and step management
   - Beginning, combat, and ending phases
   - Extra turns and additional phases

6. **Spells, Abilities, and Effects (Rules 600-699)**
   - Spell casting
   - Ability implementation
   - Effect resolution

7. **Additional Rules (Rules 700-799)**
   - Actions and special actions
   - State-based actions
   - Commander-specific rules

## Testing Approach

Each section of the Comprehensive Rules is covered by specific test cases to ensure compliance:

- **Rule-Specific Tests**: Each rule with significant game impact has dedicated unit tests
- **Interaction Tests**: Tests for complex interactions between different rules
- **Edge-Case Tests**: Tests for unusual rule applications and corner cases
- **Oracle Rulings**: Tests based on official Wizards of the Coast rulings

## Key Implementation Challenges

1. **Rule Interdependencies**: Many rules reference or depend on other rules, requiring careful implementation order
2. **State-Based Actions**: Continuous checking of game state conditions (rule 704)
3. **Layering System**: Implementation of continuous effects in the correct order (rule 613)
4. **Replacement Effects**: Handling multiple replacement effects that could apply to the same event (rule 616)

## Implementation Status

The table below summarizes the implementation status of major rule sections:

| Rule Section | Description | Status | Notes |
|--------------|-------------|--------|-------|
| 100-199 | Game Concepts | ✅ | Core game flow implemented |
| 200-299 | Parts of a Card | ✅ | Card model complete |
| 300-309 | Card Types | ✅ | All card types supported |
| 400-499 | Zones | ✅ | All zones implemented |
| 500-599 | Turn Structure | ✅ | Complete turn sequence |
| 600-609 | Spells | ✅ | Spell casting fully supported |
| 610-613 | Effects | 🔄 | Complex continuous effects in progress |
| 614-616 | Replacement Effects | 🔄 | Being implemented |
| 700-799 | Additional Rules | 🔄 | Specialized rules in development |

Legend:
- ✅ Implemented and tested
- 🔄 In progress
- ⚠️ Planned but not yet implemented

## Example Rule Implementation

Here's a simplified example of how a rule is implemented in the Rummage codebase:

```rust
// Implementing Rule 302.6: "A creature's activated ability with the tap symbol 
// in its activation cost can't be activated unless the creature has been under 
// its controller's control continuously since their most recent turn began."
pub fn can_use_tap_ability(
    creature: &Creature,
    game_state: &GameState
) -> bool {
    // Check if creature has summoning sickness
    if !creature.has_haste && creature.turns_under_current_control < 1 {
        return false;
    }
    // More checks as needed...
    true
}
```

## References

- [Official Magic: The Gathering Rules](https://magic.wizards.com/en/rules)
- [Commander Rules Committee](https://mtgcommander.net/index.php/rules/)
- [Wizards of the Coast FAQ](https://magic.wizards.com/en/formats/commander) 