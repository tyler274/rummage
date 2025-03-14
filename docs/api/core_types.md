# Core Types

This document provides a reference for the fundamental data types used in Rummage.

## Game State Types

### Phase and Step Types

```rust
/// The phases of a Magic: The Gathering turn
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum Phase {
    /// Beginning phase
    Beginning,
    /// First main phase
    PreCombatMain,
    /// Combat phase
    Combat,
    /// Second main phase
    PostCombatMain,
    /// Ending phase
    Ending,
}

/// The steps within phases of a Magic: The Gathering turn
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum Step {
    /// Beginning phase - Untap step
    Untap,
    /// Beginning phase - Upkeep step
    Upkeep,
    /// Beginning phase - Draw step
    Draw,
    /// Combat phase - Beginning of combat step
    BeginningOfCombat,
    /// Combat phase - Declare attackers step
    DeclareAttackers,
    /// Combat phase - Declare blockers step
    DeclareBlockers,
    /// Combat phase - First strike damage step (only if needed)
    FirstStrikeDamage,
    /// Combat phase - Combat damage step
    CombatDamage,
    /// Combat phase - End of combat step
    EndOfCombat,
    /// Ending phase - End step
    End,
    /// Ending phase - Cleanup step
    Cleanup,
}
```

### Zone Types

```rust
/// Game zones in Magic: The Gathering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum ZoneType {
    /// The library (deck) zone
    Library,
    /// The hand zone
    Hand,
    /// The battlefield zone (permanents in play)
    Battlefield,
    /// The graveyard zone (discard pile)
    Graveyard,
    /// The stack zone (spells and abilities being cast/activated)
    Stack,
    /// The exile zone (removed from game)
    Exile,
    /// The command zone (for commanders, emblems, etc.)
    Command,
    /// The sideboard zone (unused in Commander)
    Sideboard,
}
```

## Card Types

### Card Type System

```rust
/// Types of Magic: The Gathering cards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub struct CardTypes {
    /// Card super types (Legendary, Basic, etc.)
    pub super_types: HashSet<String>,
    /// Card types (Creature, Instant, etc.)
    pub card_types: HashSet<String>,
    /// Card sub types (Human, Wizard, Equipment, etc.)
    pub sub_types: HashSet<String>,
}

impl CardTypes {
    /// Constant for the Creature type
    pub const TYPE_CREATURE: Self = Self { 
        card_types: HashSet::from(["Creature".to_string()]),
        super_types: HashSet::new(),
        sub_types: HashSet::new(),
    };
    
    /// Create a new creature type
    pub fn new_creature(creature_types: Vec<String>) -> Self {
        Self {
            card_types: HashSet::from(["Creature".to_string()]),
            super_types: HashSet::new(),
            sub_types: HashSet::from_iter(creature_types),
        }
    }
    
    /// Check if this is a creature
    pub fn is_creature(&self) -> bool {
        self.card_types.contains("Creature")
    }
    
    /// Get creature types
    pub fn get_creature_types(&self) -> Vec<String> {
        if !self.is_creature() {
            return Vec::new();
        }
        self.sub_types.iter().cloned().collect()
    }
    
    // Similar methods for other card types
}
```

### Card Details

```rust
/// Details specific to different card types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum CardDetails {
    /// Creature card details
    Creature(CreatureCard),
    /// Planeswalker card details
    Planeswalker { loyalty: i32 },
    /// Instant card details
    Instant(SpellCard),
    /// Sorcery card details
    Sorcery(SpellCard),
    /// Enchantment card details
    Enchantment(EnchantmentCard),
    /// Artifact card details
    Artifact(ArtifactCard),
    /// Land card details
    Land(LandCard),
    /// Other card types
    Other,
}

/// Creature card details
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub struct CreatureCard {
    /// Creature's power
    pub power: i32,
    /// Creature's toughness
    pub toughness: i32,
    /// Creature's type
    pub creature_type: CreatureType,
}
```

## Mana System

### Mana Types

```rust
/// Representation of mana in Magic: The Gathering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub struct Mana {
    /// Generic/colorless mana
    pub generic: u32,
    /// White mana
    pub white: u32,
    /// Blue mana
    pub blue: u32,
    /// Black mana
    pub black: u32,
    /// Red mana
    pub red: u32,
    /// Green mana
    pub green: u32,
}

impl Mana {
    /// Create a new Mana value
    pub fn new(generic: u32, white: u32, blue: u32, black: u32, red: u32, green: u32) -> Self {
        Self {
            generic,
            white,
            blue,
            black,
            red,
            green,
        }
    }
    
    /// Calculate the converted mana cost (total mana value)
    pub fn converted_mana_cost(&self) -> u32 {
        self.generic + self.white + self.blue + self.black + self.red + self.green
    }
    
    /// Get the amount of a specific color
    pub fn colored_mana_cost(&self, color: Color) -> u32 {
        match color {
            Color::Generic => self.generic,
            Color::White => self.white,
            Color::Blue => self.blue,
            Color::Black => self.black,
            Color::Red => self.red,
            Color::Green => self.green,
        }
    }
}

/// Color in Magic: The Gathering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum Color {
    /// Generic/colorless mana
    Generic,
    /// White mana
    White,
    /// Blue mana
    Blue,
    /// Black mana
    Black,
    /// Red mana
    Red,
    /// Green mana
    Green,
}
```

## Keyword Abilities

```rust
/// Keyword abilities in Magic: The Gathering
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum KeywordAbility {
    /// Flying
    Flying,
    /// First Strike
    FirstStrike,
    /// Double Strike
    DoubleStrike,
    /// Deathtouch
    Deathtouch,
    /// Lifelink
    Lifelink,
    /// Haste
    Haste,
    /// Hexproof
    Hexproof,
    /// Indestructible
    Indestructible,
    /// Menace
    Menace,
    /// Protection
    Protection,
    /// Reach
    Reach,
    /// Trample
    Trample,
    /// Vigilance
    Vigilance,
    // And many more...
}

/// Container for a card's keyword abilities
#[derive(Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct KeywordAbilities {
    /// Set of abilities this card has
    pub abilities: HashSet<KeywordAbility>,
    /// Values for abilities that need them (e.g., "Protection from black")
    pub ability_values: HashMap<KeywordAbility, String>,
}

impl KeywordAbilities {
    /// Parse keywords from rules text
    pub fn from_rules_text(rules_text: &str) -> Self {
        // Implementation that parses keywords from rules text
        let mut abilities = HashSet::new();
        let mut ability_values = HashMap::new();
        
        // Example parsing logic
        if rules_text.contains("Flying") {
            abilities.insert(KeywordAbility::Flying);
        }
        
        if rules_text.contains("First strike") {
            abilities.insert(KeywordAbility::FirstStrike);
        }
        
        // Handle Protection keyword with value
        if let Some(protection_text) = rules_text.find("Protection ") {
            abilities.insert(KeywordAbility::Protection);
            // Extract the protection value (e.g., "from black")
            // and store it in ability_values
        }
        
        Self { abilities, ability_values }
    }
}
```

## Stack and Effects

```rust
/// An item on the stack
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct StackItem {
    /// Unique identifier
    pub id: Uuid,
    /// Source entity of the stack item
    pub source: Entity,
    /// Controller of the stack item
    pub controller: Entity,
    /// Type of stack item
    pub item_type: StackItemType,
    /// Target entities
    pub targets: Vec<Entity>,
    /// Effects to apply when resolved
    pub effects: Vec<Effect>,
}

/// Types of stack items
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum StackItemType {
    /// A spell being cast
    Spell,
    /// An activated ability
    ActivatedAbility,
    /// A triggered ability
    TriggeredAbility,
}

/// An effect that can be applied to the game state
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum Effect {
    /// Deal damage to target(s)
    DealDamage {
        /// Amount of damage
        amount: u32,
        /// Source of the damage
        source: Entity,
    },
    /// Draw cards
    DrawCards {
        /// Number of cards to draw
        count: u32,
        /// Player who draws the cards
        player: Entity,
    },
    /// Add mana to a player's mana pool
    AddMana {
        /// Mana to add
        mana: Mana,
        /// Player to receive the mana
        player: Entity,
    },
    /// Destroy target permanent(s)
    DestroyPermanent {
        /// Whether the permanent can be regenerated
        can_regenerate: bool,
    },
    /// Exile target(s)
    Exile {
        /// Return zone if temporary
        return_zone: Option<ZoneType>,
        /// When to return (e.g., end of turn)
        return_condition: Option<ReturnCondition>,
    },
    // And many more effect types...
}
```

## Game Actions

```rust
/// Actions that players can take
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum PlayerAction {
    /// Play a land
    PlayLand {
        /// The land card to play
        card: Entity,
    },
    /// Cast a spell
    CastSpell {
        /// The spell to cast
        card: Entity,
        /// Targets for the spell
        targets: Vec<Entity>,
        /// Mana to spend
        mana: Mana,
    },
    /// Activate an ability
    ActivateAbility {
        /// Source of the ability
        source: Entity,
        /// Index of the ability on the source
        ability_index: usize,
        /// Targets for the ability
        targets: Vec<Entity>,
        /// Mana to spend
        mana: Option<Mana>,
    },
    /// Attack with creatures
    DeclareAttackers {
        /// Attacking creatures
        attackers: Vec<(Entity, Entity)>, // (Attacker, Defender)
    },
    /// Block with creatures
    DeclareBlockers {
        /// Blocking assignments
        blockers: Vec<(Entity, Entity)>, // (Blocker, Attacker)
    },
    /// Pass priority
    PassPriority,
    /// Mulligan
    Mulligan,
    /// Keep hand
    KeepHand,
    /// Concede game
    Concede,
}
```

## Network Types

```rust
/// Network action to synchronize across clients
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct NetworkAction {
    /// Player who initiated the action
    pub player_id: u64,
    /// The action taken
    pub action: PlayerAction,
    /// Timestamp for ordering
    pub timestamp: f64,
    /// Sequence number for this client
    pub sequence: u32,
}

/// Rollback information for network synchronization
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct RollbackInfo {
    /// Snapshot ID to roll back to
    pub snapshot_id: Uuid,
    /// Reason for rollback
    pub reason: RollbackReason,
    /// New actions to apply after rollback
    pub actions: Vec<NetworkAction>,
}

/// Reasons for a rollback
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum RollbackReason {
    /// State desynchronization detected
    StateMismatch,
    /// Random number generator desynchronization
    RngMismatch,
    /// Network latency compensation
    LatencyCompensation,
    /// Action validation failure
    InvalidAction,
}
```

## Event Types

```rust
/// Event triggered when a phase changes
#[derive(Event, Debug, Clone)]
pub struct PhaseChangeEvent {
    /// The new phase
    pub new_phase: Phase,
    /// The new step (if applicable)
    pub new_step: Option<Step>,
    /// The previous phase
    pub old_phase: Phase,
    /// The previous step (if applicable)
    pub old_step: Option<Step>,
}

/// Event triggered when damage is dealt
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    /// Entity receiving damage
    pub target: Entity,
    /// Entity dealing damage
    pub source: Entity,
    /// Amount of damage
    pub amount: u32,
    /// Whether the damage is combat damage
    pub is_combat_damage: bool,
}

/// Event triggered when a card changes zones
#[derive(Event, Debug, Clone)]
pub struct ZoneChangeEvent {
    /// The card that changed zones
    pub card: Entity,
    /// The source zone
    pub from: ZoneType,
    /// The destination zone
    pub to: ZoneType,
    /// The entity that owns the destination zone
    pub to_owner: Entity,
    /// Position in the destination zone (if applicable)
    pub position: Option<usize>,
}

/// Event triggered when a snapshot should be taken
#[derive(Event, Debug, Clone)]
pub enum SnapshotEvent {
    /// Take a new snapshot
    Take,
    /// Apply a specific snapshot
    Apply(Uuid),
    /// Save the current state to disk
    Save(String),
    /// Load a state from disk
    Load(String),
}
```

## Integration Types

```rust
/// Configuration for the game engine
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Number of players
    pub player_count: usize,
    /// Starting life total
    pub starting_life: u32,
    /// Whether commander damage is enabled
    pub enable_commander_damage: bool,
    /// Starting hand size
    pub starting_hand_size: usize,
    /// Maximum hand size
    pub maximum_hand_size: usize,
    /// Number of cards to draw per turn
    pub draw_per_turn: usize,
    /// Random seed for deterministic gameplay
    pub random_seed: Option<u64>,
}

/// Configuration for snapshots
#[derive(Resource, Debug, Clone)]
pub struct SnapshotConfig {
    /// Whether to automatically take snapshots on turn change
    pub auto_snapshot_on_turn: bool,
    /// Whether to automatically take snapshots on phase change
    pub auto_snapshot_on_phase: bool,
    /// Maximum number of snapshots to process per frame
    pub max_snapshots_per_frame: usize,
    /// Compression level for snapshots (0-9)
    pub compression_level: u8,
}
``` 