# Life Total Management in Commander

This document covers the implementation of life total management in the Commander format within Rummage.

## Commander Life Total Rules

In the Commander format, players have the following life total rules:

1. Starting life total is 40 (compared to 20 in standard Magic)
2. A player loses if their life total is 0 or less
3. A player can gain life above their starting total with no upper limit
4. Life totals are tracked for each player throughout the game
5. There is no life loss from drawing from an empty library (unlike in standard Magic)

Additionally, Commander has a unique "Commander damage" rule:
- A player loses if they've been dealt 21 or more combat damage by a single commander

## Implementation

### Life Total Component

```rust
/// Component for tracking player life totals
#[derive(Component, Debug, Clone, Reflect)]
pub struct LifeTotal {
    /// Current life value
    pub current: i32,
    /// Starting life value
    pub starting: i32,
    /// Damage taken from each commander (entity ID -> damage amount)
    pub commander_damage: HashMap<Entity, i32>,
}

impl Default for LifeTotal {
    fn default() -> Self {
        Self {
            current: 40, // Commander starts at 40 life
            starting: 40,
            commander_damage: HashMap::new(),
        }
    }
}
```

### Life Change Events

```rust
#[derive(Event, Debug, Clone)]
pub enum LifeChangeEvent {
    Gain(Entity, i32),     // (Player entity, amount)
    Loss(Entity, i32),     // (Player entity, amount)
    Set(Entity, i32),      // (Player entity, new value)
    CommanderDamage {
        target: Entity,    // Target player
        source: Entity,    // Commander entity
        amount: i32,       // Damage amount
    },
}
```

### Life Total System

```rust
/// System that handles life total changes
pub fn handle_life_changes(
    mut players: Query<(Entity, &mut LifeTotal)>,
    mut life_events: EventReader<LifeChangeEvent>,
    mut game_events: EventWriter<GameEvent>,
) {
    let mut changed_players = HashSet::new();
    
    // Process all life change events
    for event in life_events.read() {
        match event {
            LifeChangeEvent::Gain(entity, amount) => {
                if let Ok((_, mut life)) = players.get_mut(*entity) {
                    life.current += amount;
                    changed_players.insert(*entity);
                }
            },
            LifeChangeEvent::Loss(entity, amount) => {
                if let Ok((_, mut life)) = players.get_mut(*entity) {
                    life.current -= amount;
                    changed_players.insert(*entity);
                }
            },
            LifeChangeEvent::Set(entity, value) => {
                if let Ok((_, mut life)) = players.get_mut(*entity) {
                    life.current = *value;
                    changed_players.insert(*entity);
                }
            },
            LifeChangeEvent::CommanderDamage { target, source, amount } => {
                if let Ok((_, mut life)) = players.get_mut(*target) {
                    let current_damage = life.commander_damage.entry(*source).or_insert(0);
                    *current_damage += amount;
                    life.current -= amount;
                    changed_players.insert(*target);
                }
            }
        }
    }
    
    // Check for game loss conditions
    for entity in changed_players {
        if let Ok((player_entity, life)) = players.get(entity) {
            // Check for zero or less life
            if life.current <= 0 {
                game_events.send(GameEvent::PlayerLost {
                    player: player_entity,
                    reason: LossReason::LifeTotal,
                });
            }
            
            // Check for commander damage
            for (cmdr, damage) in &life.commander_damage {
                if *damage >= 21 {
                    game_events.send(GameEvent::PlayerLost {
                        player: player_entity,
                        reason: LossReason::CommanderDamage(*cmdr),
                    });
                    break;
                }
            }
        }
    }
}
```

## Life Gain/Loss Display

In the UI, life total changes are displayed with:

1. Animated counters that show the direction and amount of life change
2. Color-coded visual feedback (green for gain, red for loss)
3. Persistent life total display for all players
4. Visual warning when a player is at low life
5. Commander damage trackers for each opponent's commander

## Life Total Interactions

Various cards and effects can interact with life totals:

1. **Life gain/loss effects**: Direct modification of life totals
2. **Life total setting effects**: Cards that set life to a specific value
3. **Life swapping effects**: Cards that exchange life totals between players
4. **Damage redirection**: Effects that redirect damage from one player to another
5. **Damage prevention**: Effects that prevent damage that would be dealt

## Testing Life Total Management

We test life total functionality with:

1. **Unit tests**: Verifying the baseline functionality
2. **Integration tests**: Testing interactions with damage effects
3. **Edge case tests**: Testing boundary conditions (very high/low life totals)
4. **Visual tests**: Verifying the UI correctly displays life changes

### Example Test

```rust
#[test]
fn test_commander_damage_loss() {
    // Create a test app with required systems
    let mut app = App::new();
    app.add_systems(Update, handle_life_changes)
        .add_event::<LifeChangeEvent>()
        .add_event::<GameEvent>();
        
    // Create player entities
    let player = app.world.spawn(LifeTotal::default()).id();
    let commander = app.world.spawn_empty().id();
    
    // Deal 21 commander damage
    app.world.send_event(LifeChangeEvent::CommanderDamage {
        target: player,
        source: commander,
        amount: 21,
    });
    
    // Run systems
    app.update();
    
    // Check if loss event was sent
    let events = app.world.resource::<Events<GameEvent>>();
    let mut reader = events.get_reader();
    let mut found_loss = false;
    
    for event in reader.read(&events) {
        if let GameEvent::PlayerLost { player: p, reason: LossReason::CommanderDamage(cmdr) } = event {
            if *p == player && *cmdr == commander {
                found_loss = true;
                break;
            }
        }
    }
    
    assert!(found_loss, "Player should lose to commander damage");
}
```

## Summary

Life total management in Commander is implemented with a flexible system that:

1. Correctly applies the Commander-specific starting life total of 40
2. Tracks commander damage separately from regular life changes
3. Implements all standard and Commander-specific loss conditions
4. Provides clear visual feedback through the UI
5. Supports all card interactions with life totals 