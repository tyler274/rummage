# Commander-Specific Rules

This page details the specific rules of the Commander format and their implementation in the Rummage game engine.

## What Is Commander?

Commander (formerly known as Elder Dragon Highlander or EDH) is a multiplayer format for Magic: The Gathering created by players and embraced by Wizards of the Coast as an official format. It emphasizes social gameplay and creative deck building around a chosen legendary creature.

## Official Commander Rules

The Commander format is governed by a specific set of rules maintained by the Commander Rules Committee:

1. **Deck Construction**
   - Players choose a legendary creature as their "commander"
   - A deck contains exactly 100 cards, including the commander
   - Except for basic lands, no two cards in the deck may have the same English name
   - A card's color identity must be within the color identity of the commander
   - A deck may not contain cards with color identities outside those of its commander
   - Color identity includes colored mana symbols in costs and rules text

2. **Game Play**
   - Players begin with 40 life
   - Commanders begin the game in the "command zone"
   - While a commander is in the command zone, it may be cast, subject to normal timing restrictions
   - Each time a player casts their commander from the command zone, it costs an additional {2} for each previous time they've cast it
   - If a commander would be exiled or put into a hand, graveyard, or library, its owner may choose to move it to the command zone instead
   - A player that has been dealt 21 or more combat damage by the same commander loses the game

## Implementation Details

### Color Identity

The Color Identity system is implemented as follows:

```rust
// Simplified color identity implementation
pub struct ColorIdentity {
    white: bool,
    blue: bool,
    black: bool,
    red: bool,
    green: bool,
}

impl ColorIdentity {
    // Create from card data
    pub fn from_card(card: &Card) -> Self {
        // Extract color identity from card's cost and text
        // ...
    }
    
    // Check if a card fits within commander's color identity
    pub fn contains(&self, other: &ColorIdentity) -> bool {
        // ...
    }
}
```

### Command Zone

The Command Zone is implemented as a special game zone:

```rust
// Simplified command zone implementation
pub struct CommandZone {
    commanders: Vec<Entity>,
    cast_count: HashMap<Entity, u32>,
}

impl CommandZone {
    // Return additional cost to cast a commander
    pub fn get_commander_tax(&self, commander: Entity) -> u32 {
        self.cast_count.get(&commander).copied().unwrap_or(0) * 2
    }
    
    // Track commander being cast
    pub fn commander_cast(&mut self, commander: Entity) {
        *self.cast_count.entry(commander).or_insert(0) += 1;
    }
}
```

### Commander Damage

Commander damage is tracked per-player, per-commander:

```rust
// Simplified commander damage tracking
pub struct CommanderDamageTracker {
    // Maps (damaged_player, commander) to amount of damage
    damage: HashMap<(Entity, Entity), u32>,
}

impl CommanderDamageTracker {
    pub fn add_damage(&mut self, player: Entity, commander: Entity, amount: u32) {
        let entry = self.damage.entry((player, commander)).or_insert(0);
        *entry += amount;
        
        // Check for game loss due to commander damage
        if *entry >= 21 {
            // Trigger player loss event
        }
    }
}
```

## Commander Variants

Rummage supports these common Commander variants:

### Partner Commanders

Some legendary creatures have the "Partner" ability, allowing a deck to have two commanders:

```rust
// Simplified partner implementation
pub fn validate_partner_commanders(commander1: &Card, commander2: &Card) -> bool {
    if !commander1.has_ability("Partner") || !commander2.has_ability("Partner") {
        return false;
    }
    
    // Additional checks for Partner With ability...
    
    true
}
```

### Commander Ninjutsu

A special ability that allows commanders to be put onto the battlefield from the command zone:

```rust
pub fn can_use_commander_ninjutsu(card: &Card, game_state: &GameState) -> bool {
    // Check if card has Commander Ninjutsu ability
    if !card.has_ability_with_keyword("Commander Ninjutsu") {
        return false;
    }
    
    // Check if controller has an unblocked attacking creature
    // ...
    
    true
}
```

### Brawl

A variant with 60-card decks, only using Standard-legal cards and starting at 25 life:

```rust
pub fn initialize_brawl_game(players: &[Entity], world: &mut World) {
    for player in players {
        // Set life total to 25 for Brawl
        let mut life = world.get_mut::<Life>(*player).unwrap();
        life.current = 25;
        
        // Additional Brawl setup...
    }
}
```

## Testing Commander Rules

Commander-specific rules are thoroughly tested:

1. **Deck Validation Tests**: Ensure decks conform to Commander deck building rules
2. **Commander Casting Tests**: Verify correct tax application and timing restrictions
3. **Commander Damage Tests**: Ensure correct tracking and application of commander damage
4. **Color Identity Tests**: Validate color identity calculations and restrictions
5. **Zone Transfer Tests**: Test commander zone transfer choices

## References

- [Official Commander Rules](https://mtgcommander.net/index.php/rules/)
- [Commander Format on MTG Wiki](https://mtg.fandom.com/wiki/Commander_(format))
- [Wizards of the Coast Commander Page](https://magic.wizards.com/en/formats/commander) 