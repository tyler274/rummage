# Abilities

This document outlines the different types of abilities in Magic: The Gathering as implemented in Rummage.

## Types of Abilities

Magic: The Gathering has three main types of abilities:

1. **Activated Abilities**: Abilities a player can activate by paying a cost
2. **Triggered Abilities**: Abilities that trigger automatically when a specific event occurs
3. **Static Abilities**: Abilities that modify the game rules or create continuous effects

## Activated Abilities

Activated abilities are written in the format: "Cost: Effect." Examples include:

- "{T}: Add {G}." (A basic Forest's mana ability)
- "{2}{W}, {T}: Create a 1/1 white Soldier creature token."
- "{B}, Pay 2 life: Draw a card."

### Implementation

In Rummage, activated abilities are implemented as follows:

```rust
pub struct ActivatedAbility {
    pub activation_cost: Cost,
    pub effect: AbilityEffect,
    pub timing_restriction: Option<TimingRestriction>,
    pub target_requirement: Option<TargetRequirement>,
}

pub enum Cost {
    Mana(ManaCost),
    Tap,
    PayLife(u32),
    SacrificeThis,
    SacrificePermanent(PermanentType),
    Discard(DiscardRequirement),
    Exile(ExileRequirement),
    Multiple(Vec<Cost>),
    // Other cost types...
}
```

### Mana Abilities

A special subcategory of activated abilities, mana abilities:
- Produce mana
- Don't target
- Don't use the stack
- Resolve immediately

## Triggered Abilities

Triggered abilities use the words "when," "whenever," or "at." Examples include:

- "When this creature enters the battlefield, draw a card."
- "Whenever a creature dies, gain 1 life."
- "At the beginning of your upkeep, draw a card."

### Implementation

In Rummage, triggered abilities are implemented with:

```rust
pub struct TriggeredAbility {
    pub trigger_condition: TriggerCondition,
    pub effect: AbilityEffect,
    pub target_requirement: Option<TargetRequirement>,
}

pub enum TriggerCondition {
    EntersBattlefield(FilterType),
    LeavesBattlefield(FilterType),
    CastSpell(FilterType),
    BeginningOfPhase(Phase),
    EndOfPhase(Phase),
    CreatureDies(FilterType),
    DamageDealt(DamageFilter),
    // Other trigger conditions...
}
```

### Trigger Types

There are several types of triggers:
- **State triggers**: "When/Whenever [state] is true..."
- **Event triggers**: "When/Whenever [event] occurs..."
- **Phase/step triggers**: "At the beginning of [phase/step]..."

## Static Abilities

Static abilities apply continuously and don't use the stack. Examples include:

- "Creatures you control get +1/+1."
- "Opponent's spells cost {1} more to cast."
- "You have hexproof."

### Implementation

In Rummage, static abilities are implemented as follows:

```rust
pub struct StaticAbility {
    pub effect: StaticEffect,
    pub condition: Option<Condition>,
}

pub enum StaticEffect {
    ModifyPowerToughness(PTModification),
    ModifyCost(CostModification),
    GrantKeyword(KeywordGrant),
    PreventActions(ActionPrevention),
    ReplaceEffect(ReplacementEffect),
    // Other static effects...
}
```

### Layering System

Static abilities that modify characteristics are applied in a specific order defined by the "layer system":
1. Copy effects
2. Control-changing effects
3. Text-changing effects
4. Type-changing effects
5. Color-changing effects
6. Ability-adding/removing effects
7. Power/toughness changing effects

## Keyword Abilities

Keyword abilities are shorthand for common abilities:

- **Flying**: "This creature can only be blocked by creatures with flying or reach."
- **Haste**: "This creature can attack and use {T} abilities as soon as it comes under your control."
- **Deathtouch**: "Any amount of damage this deals to a creature is enough to destroy it."
- **Trample**: "This creature can deal excess combat damage to the player or planeswalker it's attacking."

### Implementation

In Rummage, keyword abilities are implemented as:

```rust
pub enum KeywordAbility {
    Flying,
    Haste,
    Vigilance,
    Trample,
    FirstStrike,
    DoubleStrike,
    Deathtouch,
    Lifelink,
    Reach,
    Hexproof,
    Indestructible,
    // Other keywords...
}
```

## Loyalty Abilities

Planeswalkers have loyalty abilities, a special type of activated ability:

- "+1: Draw a card."
- "-2: Create a 3/3 Beast creature token."
- "-8: You get an emblem with 'Creatures you control get +2/+2.'"

### Implementation

Loyalty abilities are implemented as:

```rust
pub struct LoyaltyAbility {
    pub cost: i32, // Positive or negative loyalty change
    pub effect: AbilityEffect,
    pub target_requirement: Option<TargetRequirement>,
}
```

## Implementation Status

The ability implementation status is:

- ✅ Basic activated abilities
- ✅ Mana abilities
- ✅ Common keyword abilities
- 🔄 Complex triggered abilities
- 🔄 Replacement effects
- 🔄 Layering system
- ✅ Loyalty abilities

## Related Documentation

- [Targeting](../mtg_rules/targeting.md): How abilities target
- [Stack](../mtg_rules/stack.md): How abilities use the stack
- [Mana and Costs](../mtg_rules/mana_costs.md): How ability costs work 