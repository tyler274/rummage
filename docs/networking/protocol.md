# Networking Protocol Specification

This document defines the protocol used for networking in our MTG Commander game engine. It outlines the message formats, synchronization mechanisms, and handling of game-specific concepts.

## Table of Contents

1. [Message Format](#message-format)
2. [Connection Handshake](#connection-handshake)
3. [Authentication](#authentication)
4. [Game State Synchronization](#game-state-synchronization)
5. [Player Actions](#player-actions)
6. [Card Interactions](#card-interactions)
7. [MTG-Specific Handling](#mtg-specific-handling)
8. [Error Handling](#error-handling)

## Message Format

All networked messages follow a standardized format using bevy_replicon's structure:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkMessage<T> {
    /// Message type identifier
    pub message_type: MessageType,
    /// Payload data
    pub payload: T,
    /// Sequence number for ordering
    pub sequence: u64,
    /// Timestamp when the message was created
    pub timestamp: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MessageType {
    /// Game state synchronization
    GameState,
    /// Player action
    Action,
    /// System message
    System,
    /// Chat message
    Chat,
    /// Error message
    Error,
}
```

## Connection Handshake

The connection handshake establishes a client-server relationship and verifies compatibility:

1. **Client Request**: Client sends connection request with version information
2. **Server Validation**: Server validates compatibility and available slots
3. **Connection Accept/Reject**: Server sends acceptance or rejection
4. **Game State Sync**: Server sends initial game state if accepted

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConnectionRequest {
    /// Client version
    pub version: String,
    /// Player name/identifier
    pub player_name: String,
    /// Unique client identifier
    pub client_id: Option<u64>,
    /// Reconnection token for game in progress (if any)
    pub reconnect_token: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConnectionResponse {
    /// Whether the connection was accepted
    pub accepted: bool,
    /// Reason for rejection if applicable
    pub rejection_reason: Option<String>,
    /// Server-assigned client ID
    pub assigned_client_id: Option<u64>,
    /// Session identifier
    pub session_id: Option<String>,
}
```

## Authentication

For secure multiplayer, we implement a simple authentication system:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthRequest {
    /// Username or identifier
    pub username: String,
    /// Password or token
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthResponse {
    /// Whether authentication was successful
    pub success: bool,
    /// Authentication token for future use
    pub token: Option<String>,
    /// Expiration time for the token
    pub expiration: Option<f64>,
}
```

## Game State Synchronization

Game state synchronization happens at multiple levels:

1. **Full State Sync**: Complete game state sent on connection or major changes
2. **Delta Updates**: Only changed components sent during normal gameplay
3. **Targeted Updates**: Some updates only sent to specific clients (for hidden information)

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameStateSync {
    /// Whether this is a full sync or delta
    pub is_full_sync: bool,
    /// Current game turn
    pub turn: u32,
    /// Current game phase
    pub phase: Phase,
    /// Active player entity ID
    pub active_player: u64,
    /// Priority player entity ID
    pub priority_player: Option<u64>,
    /// Entity changes (components)
    pub entity_changes: Vec<EntityChange>,
    /// Game events that occurred
    pub events: Vec<GameEvent>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityChange {
    /// Entity ID
    pub entity_id: u64,
    /// Component changes
    pub components: Vec<ComponentChange>,
    /// Whether the entity was created
    pub is_new: bool,
    /// Whether the entity was destroyed
    pub is_removed: bool,
}
```

## Player Actions

Players send action requests to the server, which validates and processes them:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerActionRequest {
    /// Client ID
    pub client_id: u64,
    /// Action type
    pub action_type: NetworkedActionType,
    /// Target entities
    pub targets: Vec<u64>,
    /// Action parameters
    pub parameters: Option<ActionParameters>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NetworkedActionType {
    /// Play a land card
    PlayLand,
    /// Cast a spell
    CastSpell,
    /// Activate an ability
    ActivateAbility,
    /// Declare attackers
    DeclareAttackers,
    /// Declare blockers
    DeclareBlockers,
    /// Pass priority
    PassPriority,
    /// Make a mulligan decision
    Mulligan,
    /// Choose to put a commander in the command zone
    CommanderZoneChoice,
    /// Stack response
    RespondToStack,
    /// Choose targets
    ChooseTargets,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionParameters {
    /// Mana payment information
    pub mana_payment: Option<Mana>,
    /// Ability index (for cards with multiple abilities)
    pub ability_index: Option<usize>,
    /// X value for spells with X in cost
    pub x_value: Option<u32>,
    /// Selected mode for modal spells
    pub selected_mode: Option<u32>,
    /// Additional costs paid
    pub additional_costs: Option<Vec<AdditionalCost>>,
}
```

## Card Interactions

Card interactions require special handling for targeting, stack effects, and zone changes:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetSelection {
    /// Source card/ability
    pub source: u64,
    /// Selected targets
    pub targets: Vec<Target>,
    /// Whether all required targets have been selected
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Target {
    /// Target entity ID
    pub entity_id: u64,
    /// Target type
    pub target_type: TargetType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TargetType {
    /// Player
    Player,
    /// Creature
    Creature,
    /// Permanent
    Permanent,
    /// Spell on stack
    Spell,
    /// Zone
    Zone,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StackEffect {
    /// Effect ID
    pub id: u64,
    /// Source entity
    pub source: u64,
    /// Controller
    pub controller: u64,
    /// Targets
    pub targets: Vec<Target>,
    /// Effect details
    pub effect: EffectDetails,
}
```

## MTG-Specific Handling

MTG has unique concepts that require special handling:

### Hidden Information

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HiddenInformation {
    /// Zone type containing hidden information
    pub zone: ZoneType,
    /// Owner of the zone
    pub owner: u64,
    /// Hidden card IDs
    pub card_ids: Vec<u64>,
    /// Whether zone contents were reordered
    pub reordered: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RevealedCard {
    /// Card entity ID
    pub card_id: u64,
    /// Zone before reveal
    pub from_zone: ZoneType,
    /// Players who can see the card
    pub visible_to: Vec<u64>,
    /// Reveal source (card/effect that caused reveal)
    pub reveal_source: Option<u64>,
}
```

### Randomization

To prevent cheating, all randomization happens on the server:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RandomizationRequest {
    /// Type of randomization
    pub randomization_type: RandomizationType,
    /// Entities involved
    pub entities: Vec<u64>,
    /// Additional parameters
    pub parameters: Option<RandomizationParameters>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RandomizationType {
    /// Shuffle a library
    ShuffleLibrary,
    /// Flip a coin
    CoinFlip,
    /// Roll a die
    DieRoll,
    /// Select random targets
    RandomTargets,
    /// Random card discard
    RandomDiscard,
}
```

### Priority System

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PriorityUpdate {
    /// Player with priority
    pub priority_player: u64,
    /// Time left to respond (in milliseconds)
    pub time_remaining: Option<u64>,
    /// What's currently on the stack
    pub stack_size: usize,
    /// Current phase/step
    pub current_phase: Phase,
}
```

## Error Handling

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkError {
    /// Error code
    pub code: u32,
    /// Error message
    pub message: String,
    /// Related entity (if applicable)
    pub related_entity: Option<u64>,
    /// Related action (if applicable)
    pub related_action: Option<NetworkedActionType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ErrorCode {
    /// Invalid action
    InvalidAction = 1000,
    /// Invalid target
    InvalidTarget = 1001,
    /// Not your turn
    NotYourTurn = 1002,
    /// Not your priority
    NotYourPriority = 1003,
    /// Invalid mana payment
    InvalidManaPayment = 1004,
    /// Game rule violation
    GameRuleViolation = 1005,
    /// Connection error
    ConnectionError = 2000,
    /// Synchronization error
    SyncError = 2001,
    /// Internal server error
    InternalError = 9000,
}
```

---

This protocol specification provides a comprehensive framework for networked gameplay in our MTG Commander engine. It balances efficiency, security, and game rule enforcement while handling the unique requirements of Magic: The Gathering, such as hidden information, complex targeting, and state-based effects. 