# Random Mechanics Test Cases

## Overview

This document outlines test cases for Magic: The Gathering cards and mechanics involving randomness, including coin flips, dice rolls, and random selections. When implementing these mechanics, it's crucial to ensure deterministic testing while maintaining fair randomness in actual gameplay.

## Test Principles

1. **Deterministic Testing**: All random operations must be mockable for testing
2. **Seed Control**: Tests should use fixed seeds for reproducibility
3. **Distribution Validation**: Test that random operations have the expected distribution over many trials
4. **User Interface**: Test that random events are properly communicated to players

## Coin Flip Test Cases

### Basic Coin Flip Mechanics

```rust
#[test]
fn test_basic_coin_flip() {
    // Create a test game with mocked RNG using a predetermined seed
    let mut game = TestGame::new_with_seed(12345);
    
    // Add Krark's Thumb to player 1's battlefield
    // "If you would flip a coin, instead flip two coins and ignore one of them."
    game.add_card_to_battlefield(1, "Krark's Thumb");
    
    // Add Chance Encounter to player 1's battlefield
    // "Whenever you win a coin flip, put a luck counter on Chance Encounter.
    // At the beginning of your upkeep, if Chance Encounter has ten or more luck counters on it, you win the game."
    game.add_card_to_battlefield(1, "Chance Encounter");
    
    // Cast Stitch in Time
    // "Flip a coin. If you win the flip, take an extra turn after this one."
    game.cast_spell(1, "Stitch in Time");
    
    // Verify the coin flip sequence was triggered
    assert!(game.event_occurred(GameEvent::CoinFlipStarted));
    
    // With our test seed, we know the first flip should be a win
    // and the second flip would be a loss, but Krark's Thumb lets us ignore one
    assert_eq!(game.get_extra_turns_count(1), 1);
    
    // A luck counter should be added to Chance Encounter
    assert_eq!(game.get_counters(Card::by_name("Chance Encounter"), CounterType::Luck), 1);
}
```

### Commander with Coin Flip Abilities

```rust
#[test]
fn test_okaun_zndrsplt_interaction() {
    // Create a test game with mocked RNG
    let mut game = TestGame::new_with_seed(42);
    
    // Add Okaun, Eye of Chaos and Zndrsplt, Eye of Wisdom as commanders for player 1
    // Okaun: "Whenever you win a coin flip, double Okaun's power and toughness until end of turn."
    // Zndrsplt: "Whenever you win a coin flip, draw a card."
    game.add_commander(1, "Okaun, Eye of Chaos");
    game.add_commander(1, "Zndrsplt, Eye of Wisdom");
    
    // Cast Frenetic Efreet
    // "{0}: Flip a coin. If you win the flip, Frenetic Efreet phases out.
    // If you lose the flip, sacrifice Frenetic Efreet."
    game.cast_spell(1, "Frenetic Efreet");
    
    // Activate ability three times, with predetermined results: win, lose, win
    game.set_next_coin_flips([true, false, true]);
    
    for _ in 0..3 {
        game.activate_ability(Card::by_name("Frenetic Efreet"), 0);
        game.resolve_stack();
    }
    
    // Verify Okaun's power/toughness was doubled twice (for two wins)
    // Base P/T is 3/3, doubled twice = 12/12
    let okaun = game.get_battlefield_card(1, "Okaun, Eye of Chaos");
    assert_eq!(game.get_power(okaun), 12);
    assert_eq!(game.get_toughness(okaun), 12);
    
    // Verify player drew 2 cards from Zndrsplt's ability
    assert_eq!(game.get_hand_size(1), 2);
    
    // Verify Frenetic Efreet was sacrificed on the loss
    assert!(game.in_graveyard(Card::by_name("Frenetic Efreet")));
}
```

### Edge Cases

```rust
#[test]
fn test_coin_flip_replacement_effects() {
    let mut game = TestGame::new_with_seed(7777);
    
    // Add Krark's Thumb to player 1's battlefield
    game.add_card_to_battlefield(1, "Krark's Thumb");
    
    // Add Goblin Archaeologist to battlefield
    // "{T}: Flip a coin. If you win the flip, destroy target artifact.
    // If you lose the flip, sacrifice Goblin Archaeologist."
    game.add_card_to_battlefield(1, "Goblin Archaeologist");
    
    // Add a target artifact
    game.add_card_to_battlefield(2, "Sol Ring");
    
    // Add Chance Encounter
    game.add_card_to_battlefield(1, "Chance Encounter");
    
    // Add Tavern Scoundrel
    // "Whenever you win one or more coin flips, create a Treasure token."
    game.add_card_to_battlefield(1, "Tavern Scoundrel");
    
    // Setup test for replacement effect - ensure both flips are a "win"
    game.set_next_coin_flips([true, true]);
    
    // Activate Goblin Archaeologist targeting Sol Ring
    game.activate_ability_with_target(
        Card::by_name("Goblin Archaeologist"), 
        0, 
        Card::by_name("Sol Ring")
    );
    game.resolve_stack();
    
    // Verify Sol Ring was destroyed
    assert!(game.in_graveyard(Card::by_name("Sol Ring")));
    
    // Verify Goblin Archaeologist is still on the battlefield
    assert!(game.on_battlefield(Card::by_name("Goblin Archaeologist")));
    
    // Verify we got only ONE Treasure token despite flipping two coins
    // (Tavern Scoundrel triggers on "one or more" wins, not per win)
    assert_eq!(game.count_permanents_by_type(1, "Treasure"), 1);
    
    // Verify Chance Encounter got a luck counter
    assert_eq!(game.get_counters(Card::by_name("Chance Encounter"), CounterType::Luck), 1);
}
```

## Dice Rolling Test Cases

### Basic Dice Rolling

```rust
#[test]
fn test_basic_dice_rolling() {
    let mut game = TestGame::new_with_seed(42);
    
    // Add Delina, Wild Mage to battlefield
    // "Whenever Delina, Wild Mage attacks, roll a d20. If you roll a 15 or higher, 
    // create a token that's a copy of target creature you control..."
    game.add_card_to_battlefield(1, "Delina, Wild Mage");
    
    // Add a target creature
    game.add_card_to_battlefield(1, "Llanowar Elves");
    
    // Set up attack
    game.begin_combat();
    game.declare_attacker(Card::by_name("Delina, Wild Mage"), 2);
    
    // Mock the d20 roll to be 17
    game.set_next_dice_roll(20, 17);
    
    // Resolve Delina's triggered ability, targeting Llanowar Elves
    game.choose_target_for_ability(
        Card::by_name("Delina, Wild Mage"), 
        Card::by_name("Llanowar Elves")
    );
    game.resolve_stack();
    
    // Verify a token copy was created
    assert_eq!(game.count_permanents_by_name(1, "Llanowar Elves"), 2);
    
    // Verify the token has haste (part of Delina's ability)
    let token = game.get_tokens_by_name(1, "Llanowar Elves")[0];
    assert!(game.has_ability(token, Ability::Haste));
}
```

### Commander With Dice Rolling Abilities

```rust
#[test]
fn test_farideh_commander() {
    let mut game = TestGame::new_with_seed(42);
    
    // Add Farideh, Devil's Chosen as commander for player 1
    // "Whenever you roll one or more dice, Farideh, Devil's Chosen gains flying and menace until end of turn.
    // Whenever you roll a natural 20 on a d20, put a +1/+1 counter on Farideh."
    game.add_commander(1, "Farideh, Devil's Chosen");
    
    // Cast Pixie Guide
    // "If you would roll a d20, instead roll two d20s and use the higher roll."
    game.cast_spell(1, "Pixie Guide");
    
    // Cast Light of Hope
    // "Roll a d20. If you roll a 1-9, create a 3/3 Angel creature token with flying.
    // If you roll a 10-19, you gain 3 life for each creature you control.
    // If you roll a 20, until end of turn, creatures you control gain +2/+2 and acquire flying."
    game.cast_spell(1, "Light of Hope");
    
    // Set dice rolls to 8 and 20 (Pixie Guide will use the 20)
    game.set_next_dice_rolls(20, [8, 20]);
    game.resolve_stack();
    
    // Verify the abilities triggered by the natural 20
    // 1. Creatures get +2/+2 and flying from Light of Hope
    // 2. Farideh gets a +1/+1 counter from her ability
    // 3. Farideh gets flying and menace from her first ability
    
    // Check Farideh's power/toughness (base 3/3 + 1 counter + 2 from Light of Hope = 6/6)
    let farideh = game.get_battlefield_card(1, "Farideh, Devil's Chosen");
    assert_eq!(game.get_power(farideh), 6);
    assert_eq!(game.get_toughness(farideh), 6);
    assert_eq!(game.get_counters(farideh, CounterType::PlusOnePlusOne), 1);
    
    // Check abilities
    assert!(game.has_ability(farideh, Ability::Flying));
    assert!(game.has_ability(farideh, Ability::Menace));
    
    // Pixie Guide should also have +2/+2 and flying from Light of Hope
    let pixie = game.get_battlefield_card(1, "Pixie Guide");
    assert_eq!(game.get_power(pixie), game.get_base_power(pixie) + 2);
    assert!(game.has_ability(pixie, Ability::Flying));
}
```

## Random Selection Test Cases

### Random Discard

```rust
#[test]
fn test_random_discard() {
    let mut game = TestGame::new_with_seed(42);
    
    // Set up player 2's hand with known cards
    let cards = ["Island", "Lightning Bolt", "Dark Ritual", "Divination", "Doom Blade"];
    for card in cards {
        game.add_card_to_hand(2, card);
    }
    
    // Cast Hypnotic Specter with player 1
    game.add_card_to_battlefield(1, "Hypnotic Specter");
    
    // Deal damage to trigger discard
    game.deal_damage(Card::by_name("Hypnotic Specter"), Player(2), 1);
    
    // Set up the random discard to choose Lightning Bolt
    game.set_next_random_card_index(1); // 0-indexed, so this is the second card
    game.resolve_stack();
    
    // Verify Lightning Bolt was discarded
    assert!(game.in_graveyard(Card::by_name("Lightning Bolt")));
    assert_eq!(game.get_hand_size(2), 4);
}
```

### Random Target Selection

```rust
#[test]
fn test_random_target_selection() {
    let mut game = TestGame::new_with_seed(42);
    
    // Setup battlefield with multiple creatures
    game.add_card_to_battlefield(2, "Grizzly Bears");
    game.add_card_to_battlefield(2, "Hill Giant");
    game.add_card_to_battlefield(2, "Shivan Dragon");
    
    // Cast Confusion in the Ranks with player 1
    // "Whenever a permanent enters the battlefield, its controller chooses target permanent 
    // another player controls that shares a type with it. Exchange control of those permanents."
    game.add_card_to_battlefield(1, "Confusion in the Ranks");
    
    // Cast Mogg Fanatic (creature)
    game.cast_spell(1, "Mogg Fanatic");
    
    // Mogg Fanatic enters, triggering Confusion in the Ranks
    // Set the random target to be Hill Giant (index 1 of the 3 valid targets)
    game.set_next_random_target_index(1);
    game.resolve_stack();
    
    // Verify Hill Giant is now controlled by player 1
    assert_eq!(game.get_controller(Card::by_name("Hill Giant")), Player(1));
    
    // Verify Mogg Fanatic is now controlled by player 2
    assert_eq!(game.get_controller(Card::by_name("Mogg Fanatic")), Player(2));
}
```

## Integration with Commander Rules

### Commander Damage with Chaotic Strike

```rust
#[test]
fn test_commander_damage_with_random_double_strike() {
    let mut game = TestGame::new_with_seed(42);
    
    // Setup Player 1's commander
    game.add_commander(1, "Gisela, Blade of Goldnight");
    
    // Cast Chaos Warp targeting an unimportant permanent
    // "The owner of target permanent shuffles it into their library, 
    // then reveals the top card of their library. If it's a permanent card, 
    // they put it onto the battlefield."
    game.add_card_to_hand(1, "Chaos Warp");
    game.cast_spell_with_target(1, "Chaos Warp", Card::by_name("Forest"));
    
    // Set up the random reveal to be Berserker's Onslaught
    // "Attacking creatures you control have double strike."
    game.set_next_reveal_card("Berserker's Onslaught");
    game.resolve_stack();
    
    // Move to combat
    game.begin_combat();
    game.declare_attacker(Card::by_name("Gisela, Blade of Goldnight"), 2);
    
    // Resolve combat damage
    game.resolve_combat_damage();
    
    // Verify commander damage - should be doubled from Berserker's Onslaught
    // Gisela is a 5/5 flying first strike, so should deal 10 commander damage with double strike
    assert_eq!(game.get_commander_damage(2, Card::by_name("Gisela, Blade of Goldnight")), 10);
}
```

## Mock Framework for Testing Random Events

The test cases above reference various functions for mocking random events. Here's an example implementation for the test framework:

```rust
impl TestGame {
    /// Create a new test game with a fixed seed for reproducible randomness
    pub fn new_with_seed(seed: u64) -> Self {
        let mut game = Self::new();
        game.set_rng_seed(seed);
        game
    }
    
    /// Set the results of the next coin flips
    pub fn set_next_coin_flips(&mut self, results: impl Into<Vec<bool>>) {
        self.coin_flip_results = results.into();
    }
    
    /// Set the result of a single upcoming coin flip
    pub fn set_next_coin_flip(&mut self, result: bool) {
        self.coin_flip_results = vec![result];
    }
    
    /// Set the results of the next dice rolls of a specific size (e.g., d20)
    pub fn set_next_dice_rolls(&mut self, sides: u32, results: impl Into<Vec<u32>>) {
        self.dice_roll_results.insert(sides, results.into());
    }
    
    /// Set the result of a single upcoming dice roll
    pub fn set_next_dice_roll(&mut self, sides: u32, result: u32) {
        self.dice_roll_results.insert(sides, vec![result]);
    }
    
    /// Set which card will be chosen in a random selection (by index)
    pub fn set_next_random_card_index(&mut self, index: usize) {
        self.random_card_indices = vec![index];
    }
    
    /// Set which target will be chosen in a random selection (by index)
    pub fn set_next_random_target_index(&mut self, index: usize) {
        self.random_target_indices = vec![index];
    }
    
    /// Set the next card to be revealed from a library
    pub fn set_next_reveal_card(&mut self, card_name: &str) {
        self.next_reveal_cards.push(card_name.to_string());
    }
}
```

## Implementation Notes

1. All random operations should be abstracted through an `RngService` to allow for deterministic testing.
2. The `RngService` should be injectable and replaceable with a mock version for tests.
3. Test cases should validate both mechanical correctness and expected probabilities.
4. The UI should clearly communicate random events to players, with appropriate animations.

## Future Considerations

1. Testing network lag effects on synchronization of random events in multiplayer games
2. Validating fairness of random number generation over large sample sizes
3. Implementing proper security measures to prevent cheating in networked games 