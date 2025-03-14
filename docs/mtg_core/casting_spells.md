# Casting Spells

This document outlines the rules for casting spells in Magic: The Gathering as implemented in Rummage.

## Spell Casting Process

Casting a spell in Magic: The Gathering follows a specific sequence of steps:

1. **Announce the Spell**: The player announces they are casting a spell and places it on the stack.
2. **Choose Modes**: If the spell is modal, the player chooses which mode(s) to use.
3. **Choose Targets**: If the spell requires targets, the player chooses legal targets.
4. **Choose Division of Effects**: For spells that divide their effect, the player chooses how to divide it.
5. **Choose how to pay X**: For spells with X in their cost, the player chooses the value of X.
6. **Determine Total Cost**: Calculate the total cost, including additional or alternative costs.
7. **Activate Mana Abilities**: The player activates mana abilities to generate mana.
8. **Pay Costs**: The player pays all costs in any order.

After these steps, the spell is considered cast and is placed on the stack. It will resolve when all players pass priority in succession.

## Implementation Details

In Rummage, spell casting is implemented using a state machine approach that guides the player through each step:

```rust
pub enum SpellCastingState {
    NotCasting,
    AnnouncingSpell,
    ChoosingModes,
    ChoosingTargets,
    DividingEffects,
    ChoosingX,
    CalculatingCost,
    GeneratingMana,
    PayingCosts,
    Completed,
}

pub struct SpellCasting {
    pub state: SpellCastingState,
    pub spell_entity: Option<Entity>,
    pub casting_player: Entity,
    pub modes_chosen: Vec<u32>,
    pub targets_chosen: Vec<TargetInfo>,
    pub divisions: Vec<u32>,
    pub x_value: u32,
    pub total_cost: ManaCost,
    pub paid_mana: Vec<ManaPayment>,
}
```

## Types of Spells

Magic: The Gathering has several types of spells:

### Permanent Spells

These become permanents on the battlefield when they resolve:
- Creature spells
- Artifact spells
- Enchantment spells
- Planeswalker spells
- Land cards (technically not spells)

### Non-Permanent Spells

These go to the graveyard when they resolve:
- Instant spells
- Sorcery spells

## Timing Restrictions

Different spell types have different timing restrictions:

- **Instant**: Can be cast at any time when a player has priority
- **Sorcery**: Can only be cast during a player's main phase when the stack is empty and they have priority
- **Other spell types**: Generally follow sorcery timing, with exceptions based on abilities

## Special Cases

### Split Cards

Split cards have two halves, and the player chooses which half to cast or may cast both halves with Fuse.

### Modal Spells

Modal spells allow the player to choose from multiple effects, with the number of choices often specified on the card.

### Spells with X in the Cost

The player chooses a value for X, which affects both the spell's cost and its effect.

### Alternative Costs

Some spells offer alternative ways to cast them, such as:
- Flashback
- Overload
- Foretell
- Adventure

## UI Integration

The spell casting process integrates with the UI through:

1. **Card Selection**: Players select a card to begin casting
2. **Target Selection**: Visual interface for selecting targets
3. **Cost Payment**: Interface for choosing which mana to pay
4. **Mode Selection**: Interface for choosing modes for modal spells

For more on the UI implementation, see [Card Selection](../game_gui/interaction/card_selection.md).

## Implementation Status

The spell casting implementation is:

- ✅ Basic spell casting
- ✅ Cost calculation
- ✅ Target selection
- ✅ Mana payment
- 🔄 Alternative costs
- 🔄 Complex modal spells
- ✅ X spells

## Related Documentation

- [Stack](../mtg_rules/stack.md): How spells interact with the stack
- [Mana and Costs](../mtg_rules/mana_costs.md): Details on mana costs and payment
- [Targeting](../mtg_rules/targeting.md): Rules for selecting targets 