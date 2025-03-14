# Magic: The Gathering Rules Reference

This section provides a comprehensive reference to the Magic: The Gathering rules as implemented in Rummage. It serves as a bridge between the official comprehensive rules and our game engine implementation.

## How Rules Are Organized

The Rummage documentation organizes Magic: The Gathering rules into three layers:

1. **MTG Rules Reference** (this section) - A high-level explanation of the rules and mechanics
2. **MTG Core Rules** - Detailed implementation of fundamental rules shared by all formats
3. **Format-Specific Rules** - Extensions and modifications for specific formats (e.g., Commander)

This layered approach ensures that common rules are documented once in the core layer, while format-specific variations are documented in their respective format sections.

## Core Rules vs. Format Rules

Understanding the distinction between core rules and format rules is essential:

- **Core Rules**: Universal mechanics that apply to all Magic: The Gathering games (turn structure, stack, zones, etc.)
- **Format Rules**: Additional rules and modifications specific to a format (Commander damage, partner commanders, etc.)

In Rummage, both are implemented as composable ECS systems, allowing shared core systems with format-specific extensions.

## Implementation Methodology

Our rules implementation follows a methodology designed for correctness, testability, and extensibility:

1. **Rule Extraction**: Rules are extracted from the [Comprehensive Rules](MagicCompRules%2020250207.txt)
2. **System Design**: Rules are modeled as composable Bevy ECS systems
3. **State Representation**: Game state is represented as entities with components
4. **Event-Driven Actions**: Rule applications are triggered by relevant game events
5. **Verification**: Extensive testing ensures comprehensive rules compliance

## Rules Categories

The MTG rules are broken down into the following main categories:

### Game Structure Rules

- [Turn Structure](turn_structure.md) - Phases, steps, and the progression of a turn
- [Stack](stack.md) - How spells and abilities are put onto and resolved from the stack
- [Zones](zones.md) - Game areas where cards can exist (library, hand, battlefield, etc.)
- [State-Based Actions](state_based_actions.md) - Automatic game checks that maintain game integrity

### Card Rules

- [Card Types](card_types.md) - The various types of cards and their characteristics
- [Card States](card_states.md) - Different states a card can be in (tapped, face-down, etc.)
- [Mana Costs](mana_costs.md) - How mana costs work and are calculated

### Gameplay Rules

- [Combat](combat.md) - Rules for attacking, blocking, and combat damage
- [Targeting](targeting.md) - How targets are selected and validated
- [Effects](effects.md) - Different types of effects and how they're applied
- [Keywords](keywords.md) - Standard keyword abilities and their implementations

### Advanced Rules

- [Triggered Abilities](triggered_abilities.md) - How triggered abilities work and are resolved
- [Replacement Effects](replacement_effects.md) - How replacement effects modify events
- [Priority](priority.md) - The system determining when players can take actions
- [Layer System](layer_system.md) - How continuous effects are applied in a specific order

## Format-Specific Rules

Rules specific to particular formats are documented in their respective sections:

- [Commander Format Rules](../formats/commander/index.md)
- [Two-Headed Giant Rules](../formats/two_headed_giant/index.md)
- [Planechase Rules](../formats/planechase/index.md)

## Implementation Examples

Throughout the rules documentation, you'll find code examples showing how the rules are implemented in Rummage:

```rust
// Example: A system implementing state-based actions
pub fn check_state_based_actions(
    mut commands: Commands,
    mut creatures: Query<(Entity, &Creature, &Health)>,
    mut players: Query<(Entity, &Player)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Check for creatures with lethal damage
    for (entity, creature, health) in creatures.iter() {
        if health.damage >= creature.toughness {
            // Creature has lethal damage, destroy it
            commands.entity(entity).insert(Destroyed);
            game_events.send(GameEvent::CreatureDestroyed { entity });
        }
    }
    
    // Check for players with zero or less life
    for (entity, player) in players.iter() {
        if player.life <= 0 {
            game_events.send(GameEvent::PlayerLost { 
                player: entity,
                reason: LossReason::ZeroLife,
            });
        }
    }
    
    // Other state-based actions...
}
```

## Rules Interactions

Magic: The Gathering is known for its complex rule interactions. The documentation explains how different rule systems interact:

- How the stack interacts with state-based actions
- How replacement effects modify zone changes
- How continuous effects are applied in layers
- How priority flows during complex game scenarios

## Testing Rules Correctness

The Rummage engine extensively tests rules correctness:

- Unit tests for individual rule applications
- Integration tests for interactions between rule systems
- Scenario tests for complex game states
- Regression tests for previously identified issues

For more details on how rules are tested, see the [Testing Overview](../testing/index.md).

## Rules Implementation Resources

- [Comprehensive Rules PDF](https://media.wizards.com/2023/downloads/MagicCompRules%2020230414.pdf) - The official comprehensive rules document
- [MTG Wiki](https://mtg.fandom.com/wiki/Magic:_The_Gathering_Wiki) - Community-maintained rules explanations
- [Scryfall](https://scryfall.com/) - Card database with official rulings
- [MTG Salvation](https://www.mtgsalvation.com/) - Community discussion of rules interactions

## How to Use This Documentation

- For a high-level overview of a rule, start with the relevant page in this MTG Rules section
- For implementation details, follow the links to the MTG Core Rules section
- For format-specific rules, check the format's dedicated rules documentation
- For code examples, look at the implementation snippets provided throughout

---

Next: [Comprehensive Rules Overview](comprehensive_rules.md)
