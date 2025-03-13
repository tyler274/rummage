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

Bevy's Entity Component System (ECS) architecture provides an ideal foundation for implementing Magic: The Gathering's complex rule system. This architecture offers several key advantages:

- **Data-Logic Separation**: Game data (components) remains separate from game logic (systems)
- **Parallelism**: Game mechanics can process in parallel where possible
- **Composition**: Entities are composed of reusable components rather than inheritance hierarchies
- **Extensibility**: New functionality can be added without modifying existing code

These benefits directly address the challenges of implementing MTG's intricate, interconnected rule system in a maintainable, testable, and performant way.

## Entity Representations

In Rummage, we model the core MTG game elements as entities with specific components:

### Cards

Cards are represented as entities with components describing their characteristics:

```rust
// Core card identity
#[derive(Component)]
struct Card {
    id: String,
    oracle_id: Uuid,
}

// Name component (separate for query efficiency)
#[derive(Component)]
struct CardName(String);

// Card type component
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

// Mana cost component
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

// Creature-specific components
#[derive(Component)]
struct Power(i32);

#[derive(Component)]
struct Toughness(i32);
```

### Players

Players are entities with components tracking game state:

```rust
// Core player identity
#[derive(Component)]
struct Player {
    id: Uuid,
    name: String,
}

// Life total component
#[derive(Component)]
struct Life(i32);

// Mana pool component
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

Game zones are implemented as entities with specialized components:

```rust
// Zone type identifier
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

// Container for entities in a zone
#[derive(Component)]
struct ZoneContents {
    entities: Vec<Entity>,
}

// Zone ownership
#[derive(Component)]
struct BelongsToPlayer(Entity);
```

## Component Design

Our component design adheres to these core principles:

1. **Single Responsibility**: Each component represents one specific aspect of a game entity
2. **Data-Oriented**: Components store data only, not behavior
3. **Minimalist**: Components include only necessary data to minimize memory usage
4. **Composable**: Complex entities are built by combining simple components

This approach enables flexible entity composition. For example, a creature permanent on the battlefield might have:

```rust
// A creature permanent's components
Entity {
    Card { id: "c4a81753", oracle_id: "..." },
    CardName("Llanowar Elves"),
    CardType::Creature,
    Power(1),
    Toughness(1),
    InZone(ZoneType::Battlefield),
    UntappedState,
    TapForManaAbility { color: Green },
    CreatureType(vec!["Elf", "Druid"]),
    ControlledBy(player_entity),
    // Additional components for abilities, counters, etc.
}
```

## System Organization

Systems implement game rules and mechanics, organized into logical categories:

### Turn Structure Systems

Systems that handle the progression of game turns:

```rust
// Beginning of turn system
fn begin_turn_system(
    mut turn_state: ResMut<TurnState>,
    mut events: EventWriter<BeginTurnEvent>,
    query: Query<Entity, With<ActivePlayer>>,
) {
    if let Ok(active_player) = query.get_single() {
        events.send(BeginTurnEvent { player: active_player });
        turn_state.phase = Phase::Beginning;
        turn_state.step = Step::Untap;
    }
}

// Untap step system
fn untap_step_system(
    turn_state: Res<TurnState>,
    active_player: Query<Entity, With<ActivePlayer>>,
    mut permanents: Query<(Entity, &ControlledBy, &mut UntappedState)>,
) {
    // Skip if not in untap step
    if turn_state.phase != Phase::Beginning || turn_state.step != Step::Untap {
        return;
    }
    
    // Get active player
    let active_player = match active_player.get_single() {
        Ok(player) => player,
        Err(_) => return,
    };
    
    // Untap permanents controlled by active player
    for (_, controlled_by, mut untapped) in &mut permanents {
        if controlled_by.0 == active_player {
            *untapped = UntappedState::Untapped;
        }
    }
}
```

### State-Based Actions

Systems that check and apply state-based effects:

```rust
// System for creatures with 0 or less toughness
fn check_creature_death(
    mut commands: Commands,
    creatures: Query<(Entity, &Toughness, &InZone), With<CardType::Creature>>,
    mut zone_events: EventWriter<ZoneChangeEvent>,
) {
    for (entity, toughness, zone) in &creatures {
        if toughness.0 <= 0 && zone.0 == ZoneType::Battlefield {
            // Move creature to graveyard
            zone_events.send(ZoneChangeEvent {
                entity,
                from: ZoneType::Battlefield,
                to: ZoneType::Graveyard,
                cause: ZoneChangeCause::StateBased,
            });
        }
    }
}

// System for players with 0 or less life
fn check_player_loss(
    players: Query<(Entity, &Life, &Player)>,
    mut game_events: EventWriter<GameEvent>,
) {
    for (entity, life, player) in &players {
        if life.0 <= 0 {
            game_events.send(GameEvent::PlayerLost {
                player: entity,
                reason: LossReason::ZeroLife,
            });
        }
    }
}
```

### Spell Resolution

Systems for spell casting and resolution:

```rust
// Adding spells to the stack
fn cast_spell_system(
    mut commands: Commands,
    mut cast_events: EventReader<CastSpellEvent>,
    mut stack: ResMut<Stack>,
) {
    for event in cast_events.iter() {
        // Create stack object entity
        let stack_object = commands.spawn((
            StackObject,
            SourceCard(event.card),
            ControlledBy(event.controller),
            event.targets.clone(),
            // Additional stack object components
        )).id();
        
        // Add to stack
        stack.objects.push(stack_object);
    }
}

// Resolving the top object on the stack
fn resolve_top_of_stack(
    mut commands: Commands,
    mut stack: ResMut<Stack>,
    objects: Query<(Entity, &StackObject, &SourceCard, &ControlledBy)>,
    cards: Query<&CardType>,
    mut resolution_events: EventWriter<StackResolutionEvent>,
) {
    if let Some(top_object) = stack.objects.pop() {
        if let Ok((entity, _, source_card, controller)) = objects.get(top_object) {
            // Determine appropriate resolution based on card type
            if let Ok(card_type) = cards.get(source_card.0) {
                resolution_events.send(StackResolutionEvent {
                    stack_object: entity,
                    source: source_card.0,
                    controller: controller.0,
                    card_type: card_type.clone(),
                });
            }
        }
    }
}
```

## Event-Driven Mechanics

MTG's reactive mechanics are implemented using Bevy's event system:

### Game Events

```rust
// Card draw event
#[derive(Event)]
struct DrawCardEvent {
    player: Entity,
    amount: usize,
}

// Damage event
#[derive(Event)]
struct DamageEvent {
    source: Entity,
    target: Entity,
    amount: u32,
    is_combat_damage: bool,
}

// Zone change event
#[derive(Event)]
struct ZoneChangeEvent {
    entity: Entity,
    from: ZoneType,
    to: ZoneType,
    cause: ZoneChangeCause,
}
```

### Event Handlers

Systems that respond to these events:

```rust
// Handle card drawing
fn handle_draw_event(
    mut events: EventReader<DrawCardEvent>,
    player_hands: Query<(Entity, &ZoneType, &BelongsToPlayer)>,
    player_libraries: Query<(Entity, &ZoneType, &BelongsToPlayer, &ZoneContents)>,
    mut commands: Commands,
    mut zone_events: EventWriter<ZoneChangeEvent>,
) {
    for event in events.iter() {
        // Find player's library and hand
        // Move top N cards from library to hand
        // ...
    }
}

// Handle damage
fn handle_damage_event(
    mut events: EventReader<DamageEvent>,
    mut players: Query<(Entity, &mut Life)>,
    mut creatures: Query<(Entity, &mut Toughness)>,
    mut damage_taken: Query<&mut DamageTaken>,
) {
    for event in events.iter() {
        // Apply damage based on target type
        // ...
    }
}
```

## Example Implementations

This section provides complete examples of how complex MTG mechanics are implemented using the ECS architecture.

### Creature Combat Example

Here's how the combat system is implemented:

```rust
// Declare attackers
fn declare_attackers_system(
    mut commands: Commands,
    turn_state: Res<TurnState>,
    mut attack_declarations: EventReader<DeclareAttackerEvent>,
    creatures: Query<(Entity, &ControlledBy, &UntappedState), With<CardType::Creature>>,
    active_player: Query<Entity, With<ActivePlayer>>,
    mut phase_events: EventWriter<PhaseChangeEvent>,
) {
    // Validate phase
    if turn_state.phase != Phase::Combat || turn_state.step != Step::DeclareAttackers {
        return;
    }
    
    // Process attack declarations
    for event in attack_declarations.iter() {
        if let Ok((entity, controller, untapped_state)) = creatures.get(event.creature) {
            // Check if creature is controlled by active player and untapped
            if let Ok(active) = active_player.get_single() {
                if controller.0 == active && matches!(untapped_state, UntappedState::Untapped) {
                    // Mark as attacking
                    commands.entity(entity).insert(Attacking { 
                        defending_player: event.target,
                        // Additional attack data
                    });
                    
                    // Tap the creature
                    commands.entity(entity).insert(UntappedState::Tapped);
                }
            }
        }
    }
    
    // When all declarations are done, advance to next step
    phase_events.send(PhaseChangeEvent {
        new_phase: Phase::Combat,
        new_step: Step::DeclareBlockers,
    });
}
```

### Card Drawing and Libraries

Implementation of card drawing:

```rust
// Draw card system
fn draw_card_system(
    mut events: EventReader<DrawCardEvent>,
    players: Query<&Player>,
    libraries: Query<(Entity, &ZoneContents, &ZoneType, &BelongsToPlayer)>,
    hands: Query<(Entity, &ZoneType, &BelongsToPlayer)>,
    mut zone_events: EventWriter<ZoneChangeEvent>,
    mut game_events: EventWriter<GameEvent>,
) {
    for event in events.iter() {
        // Find player's library
        let player_library = libraries.iter()
            .find(|(_, _, zone_type, belongs_to)| {
                **zone_type == ZoneType::Library && belongs_to.0 == event.player
            });
        
        if let Some((library_entity, contents, _, _)) = player_library {
            // Check if player can draw enough cards
            if contents.entities.len() < event.amount {
                // Not enough cards - player loses
                game_events.send(GameEvent::PlayerLost {
                    player: event.player,
                    reason: LossReason::EmptyLibrary,
                });
                continue;
            }
            
            // Find player's hand
            let player_hand = hands.iter()
                .find(|(_, zone_type, belongs_to)| {
                    **zone_type == ZoneType::Hand && belongs_to.0 == event.player
                });
            
            if let Some((hand_entity, _, _)) = player_hand {
                // Move cards from library to hand
                for _ in 0..event.amount {
                    if let Some(&card) = contents.entities.last() {
                        zone_events.send(ZoneChangeEvent {
                            entity: card,
                            from: ZoneType::Library,
                            to: ZoneType::Hand,
                            cause: ZoneChangeCause::Draw,
                        });
                    }
                }
            }
        }
    }
}
```

## Connecting ECS to MTG Rules

This ECS implementation maps directly to the MTG Comprehensive Rules:

1. **Entities**: Correspond to objects in the MTG rules (cards, permanents, players)
2. **Components**: Represent characteristics and states defined in the rules
3. **Systems**: Implement rules procedures and state transitions
4. **Events**: Model the discrete game events that trigger rule applications

This mapping ensures that the implementation accurately reflects the official rules while leveraging the performance and flexibility benefits of the ECS architecture.

---

For more specific implementations, see:
- [Turn Structure](turn_structure/index.md)
- [Zones](zones/index.md)
- [Stack](stack/index.md)
- [State-Based Actions](state_actions/index.md)
- [Combat](combat/index.md) 