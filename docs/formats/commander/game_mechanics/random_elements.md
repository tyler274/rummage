# Random Elements in Commander

This document outlines how random elements in Magic: The Gathering are implemented in the Commander format within Rummage.

## Types of Random Elements

Magic: The Gathering contains several types of random elements that require implementation in a digital engine:

| Element | Description | Examples |
|---------|-------------|----------|
| Coin Flips | Binary random outcome | Krark's Thumb, Mana Crypt |
| Die Rolls | Random number generation | Delina, Wild Mage; Chaos effects |
| Random Card Selection | Selecting cards at random | Goblin Lore, Chaos Warp |
| Random Target Selection | Choosing targets randomly | Possibility Storm, Knowledge Pool |
| Random Card Generation | Creating cards not in the original deck | Booster Tutor, Garth One-Eye |

## Implementation Approach

In Rummage, we implement random elements using a deterministic RNG system that:

1. Maintains synchronization across networked games
2. Provides verifiable randomness that can be audited
3. Seeds random generators consistently across client instances
4. Allows for replaying game states with identical outcomes

### Code Example: Deterministic RNG

```rust
use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Resource)]
pub struct GameRng(ChaCha8Rng);

impl GameRng {
    pub fn new_seeded(seed: u64) -> Self {
        Self(ChaCha8Rng::seed_from_u64(seed))
    }
    
    pub fn flip_coin(&mut self) -> bool {
        self.0.gen_bool(0.5)
    }
    
    pub fn roll_die(&mut self, sides: u32) -> u32 {
        self.0.gen_range(1..=sides)
    }
    
    pub fn select_random_index(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }
        self.0.gen_range(0..max)
    }
}

// System that handles coin flips
fn handle_coin_flip(
    mut commands: Commands,
    mut rng: ResMut<GameRng>,
    mut coin_flip_events: EventReader<CoinFlipEvent>,
) {
    for event in coin_flip_events.read() {
        let result = rng.flip_coin();
        commands.spawn(CoinFlipResult {
            source: event.source,
            result,
            affected_entities: event.affected_entities.clone(),
        });
    }
}
```

## Synchronization with Multiplayer

In multiplayer Commander games, random outcomes must be identical for all players. To achieve this:

1. The host acts as the source of truth for RNG seed
2. RNG state is included in the synchronized game state
3. Random events are processed deterministically
4. Network desync detection checks RNG state

## Testing Random Elements

Random elements require special testing approaches:

1. **Seeded Tests**: Using known seeds to produce predictable outcomes
2. **Distribution Tests**: Verifying statistical properties over many trials
3. **Edge Case Tests**: Testing boundary conditions and extreme outcomes
4. **Determinism Tests**: Ensuring identical outcomes given the same seed

### Example Test

```rust
#[test]
fn test_coin_flip_determinism() {
    // Create two RNGs with the same seed
    let mut rng1 = GameRng::new_seeded(12345);
    let mut rng2 = GameRng::new_seeded(12345);
    
    // They should produce identical sequences
    for _ in 0..100 {
        assert_eq!(rng1.flip_coin(), rng2.flip_coin());
    }
}
```

## Commander-Specific Concerns

In Commander, random elements interact with multiplayer politics in unique ways:

1. Perception of fairness in random outcomes
2. Impact of random effects on multiple players simultaneously
3. Using random outcomes as leverage in political negotiations
4. Varying player attitudes toward randomness and luck

Our implementation supports these dynamics by:

- Providing clear visualization of random processes
- Implementing rules for modifying random outcomes (like Krark's Thumb)
- Supporting multiplayer-specific random effects
- Allowing customization of random elements in house rules

## Summary

Random elements in Commander are implemented using deterministic, network-synchronized systems that maintain game integrity while supporting the unique social and political dynamics of the format. 