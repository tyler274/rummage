# ECS Implementation of MTG Rules

This document explains how Magic: The Gathering rules are implemented using Bevy's Entity Component System (ECS) architecture in Rummage.

## Table of Contents

1. [Introduction](#introduction)
2. [Entity Representations](#entity-representations)
3. [Component Design](#component-design)
4. [System Organization](#system-organization)
5. [Event-Driven Mechanics](#event-driven-mechanics)
6. [Example Implementations](#example-implementations)

## Introduction

The Entity Component System (ECS) architecture is particularly well-suited for implementing a complex rule system like Magic: The Gathering. It allows us to:

- Separate game data (components) from game logic (systems)
- Process game mechanics in parallel where possible
- Model relationships between game elements naturally
- Extend functionality without modifying existing code

Rummage leverages Bevy's ECS to create a maintainable, performant implementation of MTG rules that can be easily tested and extended.

## Entity Representations

In our implementation, we represent game concepts as entities with appropriate components:

### Cards

Cards are entities with components like:
```rust
#[derive(Component)]
struct Card {
    id: String,
    oracle_id: Uuid,
}

#[derive(Component)]
struct CardName(String);

#[derive(Component)]
enum CardType {
    Creature,
    Instant,
    Sorcery,
    Artifact,
    Enchantment,
    Land,
    Planeswalker,
}

#[derive(Component)]
struct ManaCost {
    white: u8,
    blue: u8,
    black: u8,
    red: u8,
    green: u8,
    colorless: u8,
    generic: u8,
}

// For creatures
#[derive(Component)]
struct Power(i32);

#[derive(Component)]
struct Toughness(i32);
```

### Players

Players are entities with components like:
```rust
#[derive(Component)]
struct Player {
    id: Uuid,
    name: String,
}

#[derive(Component)]
struct Life(i32);

#[derive(Component)]
struct ManaPool {
    white: u8,
    blue: u8,
    black: u8,
    red: u8,
    green: u8,
    colorless: u8,
}
```

### Zones

Game zones are implemented as entities with special components:
```rust
#[derive(Component)]
enum ZoneType {
    Battlefield,
    Hand,
    Library,
    Graveyard,
    Stack,
    Exile,
    Command,
}

#[derive(Component)]
struct ZoneContents {
    entities: Vec<Entity>,
}

#[derive(Component)]
struct BelongsToPlayer(Entity);
```

## Component Design

Our component design follows these principles:

1. **Single Responsibility**: Each component represents a single aspect of a game entity
2. **Data-Oriented**: Components store data, not behavior
3. **Minimalist**: Only include necessary data to reduce memory usage
4. **Composition**: Complex entities are built by combining simple components

For example, a creature permanent on the battlefield would have components like:
- `Card` - Basic card information
- `CardName` - The card's name
- `CardType` - Type information (Creature)
- `Power` - Current power value
- `Toughness` - Current toughness value
- `Zone` - Indicates it's on the battlefield
- `Tapped` or `Untapped` - Current tap state
- Various ability components

## System Organization

Systems are organized by game mechanics and rule implementations:

### Turn Structure

Systems to handle turn progression:
```rust
fn begin_turn_system(/* ... */) {
    // Logic for beginning of turn
}

fn untap_step_system(/* ... */) {
    // Untap permanents
}

fn upkeep_step_system(/* ... */) {
    // Handle upkeep triggers
}

fn draw_step_system(/* ... */) {
    // Draw card for active player
}
```

### State-Based Actions

Systems to check and apply state-based actions:
```rust
fn check_creature_death(
    mut commands: Commands,
    creatures: Query<(Entity, &Toughness), With<Creature>>,
) {
    for (entity, toughness) in &creatures {
        if toughness.0 <= 0 {
            // Move creature to graveyard
            commands.entity(entity).remove::<ZoneType>();
            commands.entity(entity).insert(ZoneType::Graveyard);
            // ...other necessary changes
        }
    }
}

fn check_player_loss(
    mut commands: Commands,
    players: Query<(Entity, &Life, &Player)>,
    mut game_over_events: EventWriter<GameOverEvent>,
) {
    for (entity, life, player) in &players {
        if life.0 <= 0 {
            commands.entity(entity).insert(PlayerLost);
            game_over_events.send(GameOverEvent {
                player: entity,
                reason: LossReason::ZeroLife,
            });
        }
    }
}
```

### Spell Resolution

Systems to handle casting and resolving spells:
```rust
fn add_to_stack_system(
    mut commands: Commands,
    mut cast_events: EventReader<CastSpellEvent>,
    mut stack: ResMut<Stack>,
) {
    for event in cast_events.iter() {
        // Create stack object entity
        let stack_object = commands.spawn((
            StackObject,
            StackSource(event.source),
            event.targets.clone(),
            // Other stack object components
        )).id();
        
        // Add to stack
        stack.push(stack_object);
    }
}

fn resolve_top_of_stack(
    mut commands: Commands,
    mut stack: ResMut<Stack>,
    stack_objects: Query<(Entity, &StackObject)>,
    // Other necessary queries
) {
    if let Some(top_object) = stack.pop() {
        // Implement resolution logic based on object type
        // ...
    }
}
```

## Event-Driven Mechanics

MTG's reactive nature is modeled using Bevy's event system:

### Game Events

```rust
#[derive(Event)]
struct DrawCardEvent {
    player: Entity,
    amount: usize,
}

#[derive(Event)]
struct DamageEvent {
    source: Entity,
    target: Entity,
    amount: u32,
    is_combat_damage: bool,
}

#[derive(Event)]
struct ZoneChangeEvent {
    entity: Entity,
    from: ZoneType,
    to: ZoneType,
}
```

### Event Handling

```rust
fn handle_damage_events(
    mut damage_events: EventReader<DamageEvent>,
    mut creatures: Query<(Entity, &mut Toughness), With<Creature>>,
    mut players: Query<(Entity, &mut Life), With<Player>>,
) {
    for event in damage_events.iter() {
        // Handle creature damage
        if let Ok((_, mut toughness)) = creatures.get_mut(event.target) {
            toughness.0 -= event.amount as i32;
        } 
        // Handle player damage
        else if let Ok((_, mut life)) = players.get_mut(event.target) {
            life.0 -= event.amount as i32;
        }
    }
}
```

## Example Implementations

### Card Draw

```rust
fn draw_card_system(
    mut commands: Commands,
    mut players: Query<(Entity, &Player)>,
    mut libraries: Query<(&ZoneContents, &BelongsToPlayer), With<LibraryZone>>,
    mut hands: Query<&mut ZoneContents, (With<HandZone>, Without<LibraryZone>)>,
    mut draw_events: EventReader<DrawCardEvent>,
    mut zone_change_events: EventWriter<ZoneChangeEvent>,
) {
    for event in draw_events.iter() {
        let player_entity = event.player;
        
        // Find player's library
        for (library_contents, belongs_to) in &libraries {
            if belongs_to.0 == player_entity {
                // Find player's hand
                for mut hand_contents in &mut hands {
                    // Draw cards if possible
                    for _ in 0..event.amount {
                        if let Some(&card) = library_contents.entities.last() {
                            // Remove from library
                            commands.entity(card).remove::<ZoneType>();
                            commands.entity(card).insert(ZoneType::Hand);
                            
                            // Add to hand
                            hand_contents.entities.push(card);
                            
                            // Emit zone change event
                            zone_change_events.send(ZoneChangeEvent {
                                entity: card,
                                from: ZoneType::Library,
                                to: ZoneType::Hand,
                            });
                        }
                    }
                }
            }
        }
    }
}
```

### Combat Phase

```rust
fn begin_combat_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    active_player: Res<ActivePlayer>,
) {
    // Change phase
    game_state.current_phase = Phase::Combat;
    game_state.current_step = Step::BeginCombat;
    
    // Give active player priority
    commands.insert_resource(PlayerWithPriority(active_player.0));
    
    // Trigger "at beginning of combat" abilities
    // ...
}

fn declare_attackers_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut attack_declarations: EventReader<DeclareAttackerEvent>,
    creatures: Query<Entity, (With<Creature>, With<CanAttack>)>,
) {
    if game_state.current_step == Step::DeclareAttackers {
        for event in attack_declarations.iter() {
            // Validate attacker
            if creatures.contains(event.attacker) {
                // Mark as attacking
                commands.entity(event.attacker).insert(Attacking(event.defending_player));
                
                // Tap creature if needed
                commands.entity(event.attacker).insert(Tapped);
            }
        }
    }
}
```

This ECS approach provides a clean separation of concerns while maintaining the complex interactions that make Magic: The Gathering such a rich game. For more detailed implementations of specific rules, refer to the corresponding sections in the [MTG Core Rules](index.md) documentation.

---

Next: [Turn Structure](turn_structure/index.md) 