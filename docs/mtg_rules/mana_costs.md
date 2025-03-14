# Mana and Costs

This document provides an overview of the rules for mana and costs in Magic: The Gathering. For implementation details in Rummage, see the relevant code in the [mana.rs](https://github.com/tyler274/rummage/blob/main/src/mana.rs) file.

## Mana Types

Magic: The Gathering has six types of mana:

1. **White** (W)
2. **Blue** (U)
3. **Black** (B)
4. **Red** (R)
5. **Green** (G)
6. **Colorless** (C)

Additionally, there is the concept of generic mana, represented by numbers (e.g., {1}, {2}), which can be paid with any type of mana.

## Mana Costs

A spell's mana cost appears in the upper right corner of the card and consists of mana symbols. For example:

- {2}{U} means "two generic mana and one blue mana"
- {W}{W} means "two white mana"
- {X}{R}{R} means "X generic mana and two red mana" where X is chosen by the player

## Mana Pool

Players have a mana pool where mana is stored until spent or until a phase or step ends. Mana in a player's mana pool can be spent to pay costs or may be lost when a phase or step ends.

## Mana Sources

Mana is produced by mana abilities, which come from various sources:

- **Land abilities**: Basic lands and many nonbasic lands
- **Creature abilities**: Mana dorks and other creatures
- **Artifact abilities**: Mana rocks and other artifacts
- **Enchantment abilities**: Various enchantments
- **Spells**: Some instants and sorceries produce mana

## Additional Costs

Many spells and abilities have additional costs beyond their mana cost:

- **Tap costs**: {T} means "tap this permanent"
- **Life costs**: "Pay N life"
- **Sacrifice costs**: "Sacrifice a creature"
- **Discard costs**: "Discard a card"
- **Exile costs**: "Exile a card from your graveyard"

## Alternative Costs

Some spells can be cast for alternative costs:

- **Flashback**: Cast from graveyard for a different cost
- **Overload**: Cast with a different effect for a different cost
- **Foretell**: Cast from exile for a different cost
- **Evoke**: Cast with a sacrifice trigger for a reduced cost

## Cost Reduction

Various effects can reduce costs:

- "Spells you cast cost {1} less to cast"
- "This spell costs {1} less to cast for each artifact you control"
- "The first creature spell you cast each turn costs {2} less to cast"

## Cost Increases

Similarly, effects can increase costs:

- "Spells your opponents cast cost {1} more to cast"
- "Activated abilities cost {2} more to activate"
- "Creature spells with flying cost {1} more to cast"

## Special Mana Types

Beyond the basic types, there are special types of mana:

- **Snow mana** ({S}): Produced by snow permanents
- **Phyrexian mana** ({W/P}, {U/P}, etc.): Can be paid with either colored mana or 2 life
- **Hybrid mana** ({W/U}, {B/R}, etc.): Can be paid with either of two colors
- **Colorless-specific mana** ({C}): Must be paid with colorless mana, not any color

## Mana Conversion

Mana conversion for costs follows these rules:

1. Colored mana requirements must be paid with the exact color
2. Generic mana can be paid with any type of mana
3. Colorless-specific requirements must be paid with colorless mana
4. Special mana symbols follow their own rules

## Related Documentation

For more information on how mana and costs are implemented in Rummage, see:

- [Casting Spells](../mtg_core/casting_spells.md): How mana is used to cast spells
- [Abilities](../mtg_core/abilities.md): How mana abilities work
- [Stack](../mtg_core/stack/index.md): How mana abilities bypass the stack 