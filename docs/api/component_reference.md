# Component Reference

This document provides a comprehensive reference of the various components used in Rummage to represent game entities and their properties.

## Overview

Components in Bevy ECS are small, reusable pieces of data that are attached to entities. Rummage uses components to represent various aspects of Magic: The Gathering cards, players, and game elements.

## Card Components

Components that represent aspects of Magic: The Gathering cards.

### Core Card Components

```rust
/// The name of a card
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CardName {
    pub name: String,
}

/// The mana cost of a card
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CardCost {
    pub cost: Mana,
}

/// The type information of a card (creature, instant, etc.)
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CardTypeInfo {
    pub types: CardTypes,
}

/// The card's rules text
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CardRulesText {
    pub rules_text: String,
}

/// The specific details of a card (power/toughness for creatures, etc.)
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CardDetailsComponent {
    pub details: CardDetails,
}

/// The keyword abilities of a card
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CardKeywords {
    pub keywords: KeywordAbilities,
}
```

### Card State Components

```rust
/// Indicates a tapped card
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Tapped;

/// Indicates a card with summoning sickness
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SummoningSickness;

/// Indicates a card that is attacking
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Attacker {
    pub attacking_player: Entity,
}

/// Indicates a card that is blocking
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Blocker {
    pub blocking: Entity,
}

/// Damage marked on a card
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DamageMarked {
    pub amount: u32,
}

/// Counters on a card
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Counters {
    pub counter_map: HashMap<CounterType, u32>,
}

/// Indicates a card is a token
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Token;
```

### Zone Components

```rust
/// Indicates which game zone a card is in
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Zone {
    pub zone_type: ZoneType,
    pub owner: Entity,
    pub position: Option<usize>,
}

/// Represents a game zone (library, graveyard, etc.)
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ZoneContainer {
    pub zone_type: ZoneType,
    pub owner: Entity,
    pub contents: Vec<Entity>,
}
```

## Player Components

Components that represent aspects of players.

### Core Player Components

```rust
/// Core player component
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Player {
    pub id: usize,
    pub life_total: i32,
    pub is_active: bool,
}

/// Player's mana pool
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ManaPool {
    pub mana: Mana,
}

/// Player's hand size
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct HandSize {
    pub current: usize,
    pub maximum: usize,
}
```

### Commander-Specific Player Components

```rust
/// Commander damage received by a player
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CommanderDamage {
    pub damage_map: HashMap<Entity, u32>,
}

/// Commander color identity restrictions
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ColorIdentity {
    pub colors: HashSet<Color>,
}
```

## Game State Components

Components that represent aspects of the game state.

### Turn and Phase Components

```rust
/// Current game turn
#[derive(Resource, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct GameTurn {
    pub number: u32,
    pub active_player: Entity,
}

/// Current game phase
#[derive(Resource, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct GamePhase {
    pub phase: Phase,
    pub step: Option<Step>,
}

/// Priority holder
#[derive(Resource, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct Priority {
    pub player: Entity,
    pub passed_players: HashSet<Entity>,
}
```

### Stack Components

```rust
/// The game stack (for spells and abilities)
#[derive(Resource, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct Stack {
    pub items: Vec<StackItem>,
}

/// An item on the stack
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct StackItem {
    pub id: Uuid,
    pub source: Entity,
    pub controller: Entity,
    pub item_type: StackItemType,
    pub targets: Vec<Entity>,
    pub effects: Vec<Effect>,
}
```

## UI Components

Components used for the user interface representation.

### Card Visualization

```rust
/// Visual representation of a card
#[derive(Component, Debug, Clone)]
pub struct CardVisual {
    pub entity: Entity,
    pub card_face: Handle<Image>,
    pub is_facedown: bool,
    pub visual_state: CardVisualState,
}

/// UI states for cards
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardVisualState {
    Normal,
    Selected,
    Targeted,
    Highlighted,
    Disabled,
}

/// Interactive card component
#[derive(Component, Debug, Clone)]
pub struct Interactable {
    pub enabled: bool,
    pub interaction_type: InteractionType,
}
```

### Layout Components

```rust
/// Battlefield position
#[derive(Component, Debug, Clone)]
pub struct BattlefieldPosition {
    pub row: usize,
    pub column: usize,
    pub rotation: f32,
}

/// Hand position
#[derive(Component, Debug, Clone)]
pub struct HandPosition {
    pub index: usize,
    pub total: usize,
}
```

## Network Components

Components used for network synchronization.

```rust
/// Component for entities that should be synchronized over the network
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct NetworkSynchronized {
    pub id: Uuid,
    pub version: u32,
    pub owner: Option<u64>, // Network ID of the owning client
}

/// Action performed by a player that needs network broadcasting
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct PlayerAction {
    pub player: Entity,
    pub action_type: ActionType,
    pub targets: Vec<Entity>,
    pub timestamp: f64,
}
```

## System Components

Internal components used by the game engine.

```rust
/// Marker for entities that should be included in snapshots
#[derive(Component, Debug, Clone)]
pub struct Snapshotable;

/// Temporary component for marking entities that need processing
#[derive(Component, Debug, Clone)]
pub struct NeedsProcessing;

/// Component for tracking when an entity was created
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Created {
    pub timestamp: f64,
    pub turn: u32,
}
```

## Component Integration

Components are used together to represent complex game objects:

```rust
// Example of a full creature card entity with its components
commands.spawn((
    // Core card information
    Card::new(
        "Grizzly Bears",
        Mana::new(1, 0, 0, 0, 1, 0), // 1G
        CardTypes::new_creature(vec!["Bear".to_string()]),
        CardDetails::new_creature(2, 2),
        "", // No rules text
    ),
    // State information
    Zone {
        zone_type: ZoneType::Battlefield,
        owner: player_entity,
        position: None,
    },
    // Visual representation
    CardVisual {
        entity: Entity::PLACEHOLDER,
        card_face: card_image_handle,
        is_facedown: false,
        visual_state: CardVisualState::Normal,
    },
    // Battlefield positioning
    BattlefieldPosition {
        row: 0,
        column: 0,
        rotation: 0.0,
    },
    // System markers
    Snapshotable,
    NetworkSynchronized {
        id: Uuid::new_v4(),
        version: 0,
        owner: Some(player_network_id),
    },
));
```

## Component Registration

Components must be registered with the Bevy type registry to be used with reflection, serialization, and UI:

```rust
fn register_components(app: &mut App) {
    app.register_type::<CardName>()
       .register_type::<CardCost>()
       .register_type::<CardTypeInfo>()
       .register_type::<CardRulesText>()
       .register_type::<CardDetailsComponent>()
       .register_type::<CardKeywords>()
       .register_type::<Tapped>()
       .register_type::<SummoningSickness>()
       .register_type::<Attacker>()
       .register_type::<Blocker>()
       .register_type::<DamageMarked>()
       .register_type::<Counters>()
       .register_type::<Token>()
       .register_type::<Zone>()
       .register_type::<ZoneContainer>()
       .register_type::<Player>()
       .register_type::<ManaPool>()
       .register_type::<HandSize>()
       .register_type::<CommanderDamage>()
       .register_type::<ColorIdentity>()
       .register_type::<NetworkSynchronized>()
       .register_type::<PlayerAction>()
       .register_type::<Created>();
} 