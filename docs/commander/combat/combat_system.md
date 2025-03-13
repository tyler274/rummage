# Combat System

## Overview

The Combat System is a critical component of the Commander game engine, handling all aspects of creature combat including attacks, blocks, damage assignment, and combat-specific abilities. This system is especially complex in Commander due to the multiplayer nature of the format and the special rules regarding commander damage.

## Core Components

The combat system consists of several interconnected components:

1. **Combat Phase Management** - Handling the flow through combat steps
2. **Attack Declaration System** - Managing which creatures attack and who they attack
3. **Block Declaration System** - Managing how defending players assign blockers
4. **Damage Assignment System** - Calculating and applying combat damage
5. **Commander Damage Tracking** - Monitoring and accumulating commander damage
6. **Combat Triggers** - Handling abilities that trigger during combat

## System Architecture

The combat system uses a modular design pattern to separate concerns:

```rust
// Core combat system resource
#[derive(Resource)]
pub struct CombatSystem {
    pub active_combat_step: Option<CombatStep>,
    pub attackers: HashMap<Entity, AttackData>,
    pub blockers: HashMap<Entity, BlockData>,
    pub combat_triggers: Vec<CombatTrigger>,
    pub damage_assignment_order: Vec<Entity>,
    pub combat_history: VecDeque<CombatEvent>,
}

// Data for an attacking creature
#[derive(Debug, Clone)]
pub struct AttackData {
    pub attacker: Entity,
    pub defender: Entity, // Can be a player or planeswalker
    pub is_commander: bool,
    pub requirements: Vec<AttackRequirement>,
    pub restrictions: Vec<AttackRestriction>,
}

// Data for a blocking creature
#[derive(Debug, Clone)]
pub struct BlockData {
    pub blocker: Entity,
    pub blocked_attackers: Vec<Entity>,
    pub requirements: Vec<BlockRequirement>,
    pub restrictions: Vec<BlockRestriction>,
}

// Combat event for history tracking
#[derive(Debug, Clone)]
pub enum CombatEvent {
    BeginCombat { turn: u32, active_player: Entity },
    AttackDeclared { attacker: Entity, defender: Entity },
    BlockDeclared { blocker: Entity, attacker: Entity },
    DamageDealt { source: Entity, target: Entity, amount: u32, is_commander_damage: bool },
    CombatEnded { turn: u32 },
}
```

## Integration with Other Systems

The combat system interfaces with several other game systems:

- **Turn Manager** - For phase control and player ordering
- **Stack System** - For handling combat-triggered abilities 
- **Damage System** - For applying damage and handling prevention/redirection effects
- **Game State System** - For tracking changes to the game state during combat
- **Player System** - For player state changes (life totals, commander damage)

## Documentation Structure

This documentation is organized into several parts:

- [Combat Phases](combat_phases.md) - Detailed implementation of each combat step
- [Commander Damage](commander_damage.md) - Special handling of commander damage
- [Multiplayer Combat](multiplayer_combat.md) - Combat in a multiplayer environment
- [Combat Abilities](combat_abilities.md) - Implementation of combat-specific abilities
- [Combat Verification](combat_verification.md) - Testing and verification approach

Each section contains detailed information about implementation, edge cases, and testing strategies for that aspect of the combat system. 