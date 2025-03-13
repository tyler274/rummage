# System Reference

This document provides a reference for all systems in the Rummage game engine. Systems are functions that operate on components and resources to implement game logic.

## Core Game Systems

### State Management

| System | Description |
|--------|-------------|
| `game_state_condition` | Determines if the game is in an active state |
| `setup_game_engine` | Initializes the game state and components |
| `check_state_based_actions` | Implements MTG's state-based actions |
| `check_win_conditions` | Checks if any player has won or lost |
| `process_game_actions` | Processes game actions from players and effects |

### Turn Systems

| System | Description |
|--------|-------------|
| `handle_turn_start` | Handles logic at the beginning of a player's turn |
| `handle_turn_end` | Handles logic at the end of a player's turn |
| `phase_transition_system` | Manages transitions between game phases |
| `register_turn_systems` | Registers all turn-related systems with the app |
| `beginning_phase_system` | Handles logic for the beginning phase |
| `main_phase_system` | Handles logic for the main phases |
| `ending_phase_system` | Handles logic for the ending phase |

### Stack Systems

| System | Description |
|--------|-------------|
| `priority_system` | Manages priority passing between players |
| `priority_passing_system` | Handles the logic of passing priority |
| `stack_push_system` | Handles adding items to the stack |
| `stack_resolve_system` | Resolves the top item of the stack |
| `handle_stack_item_resolved` | Handles the aftermath of a resolved stack item |

### Zone Systems

| System | Description |
|--------|-------------|
| `zone_change_system` | Handles moving cards between zones |
| `handle_zone_change_events` | Processes zone change events |
| `handle_enters_battlefield` | Handles "enters the battlefield" triggers |
| `handle_leaves_battlefield` | Handles "leaves the battlefield" triggers |
| `library_system` | Manages player libraries |
| `graveyard_system` | Manages player graveyards |
| `exile_system` | Manages the exile zone |
| `command_zone_system` | Manages the command zone |

### Combat Systems

| System | Description |
|--------|-------------|
| `initialize_combat_phase` | Sets up the combat phase |
| `declare_attackers_system` | Handles declaring attackers |
| `declare_blockers_system` | Handles declaring blockers |
| `assign_combat_damage_system` | Assigns combat damage based on attackers and blockers |
| `process_combat_damage_system` | Processes the assigned combat damage |
| `end_combat_system` | Cleans up after combat |
| `handle_declare_attackers_event` | Processes events related to declaring attackers |
| `handle_declare_blockers_event` | Processes events related to declaring blockers |

### Card Systems

| System | Description |
|--------|-------------|
| `cast_spell_system` | Handles casting spells |
| `resolve_spell_system` | Handles resolving spells |
| `activated_ability_system` | Handles activated abilities |
| `triggered_ability_system` | Handles triggered abilities |
| `static_ability_system` | Applies static abilities |
| `continuous_effect_system` | Manages continuous effects |
| `mana_ability_system` | Handles mana abilities |

## Commander Format Systems

| System | Description |
|--------|-------------|
| `commander_damage_system` | Tracks and applies commander damage |
| `commander_cast_system` | Handles casting commanders from the command zone |
| `commander_tax_system` | Applies commander tax |
| `commander_death_system` | Handles commander death triggers |
| `color_identity_system` | Enforces color identity restrictions |
| `partner_commander_system` | Handles partner commanders |

## UI Systems

| System | Description |
|--------|-------------|
| `card_rendering_system` | Renders cards on the UI |
| `battlefield_layout_system` | Manages the layout of cards on the battlefield |
| `hand_layout_system` | Manages the layout of cards in hand |
| `stack_visualization_system` | Visualizes the stack |
| `drag_drop_system` | Handles drag and drop interactions |
| `card_selection_system` | Handles selecting cards |
| `targeting_system` | Handles targeting UI |
| `animation_system` | Manages card animations |

## Networking Systems

| System | Description |
|--------|-------------|
| `network_sync_system` | Synchronizes game state across the network |
| `replicon_sync_system` | Synchronizes using bevy_replicon |
| `action_broadcast_system` | Broadcasts actions to other players |
| `network_state_rollback` | Handles state rollback for network consistency |
| `deterministic_rng_system` | Ensures random number generation is consistent across the network |

## Testing Systems

| System | Description |
|--------|-------------|
| `snapshot_system` | Creates game state snapshots for testing |
| `test_action_system` | Generates actions for testing |
| `validation_system` | Validates game state integrity |
| `verification_system` | Verifies expected outcomes in tests |

## Event Handler Systems

| System | Description |
|--------|-------------|
| `handle_game_events` | General event handling system |
| `handle_snapshot_events` | Processes snapshot creation events |
| `process_pending_snapshots` | Processes pending snapshot operations |
| `event_logging_system` | Logs game events for debugging |

## System Scheduling

The Rummage game engine carefully schedules systems to ensure they run in the correct order. Key system sets include:

1. **Startup Systems** - Run once during initialization
2. **Pre-Update Systems** - Run before the main update
3. **Update Systems** - Main game logic systems
4. **Post-Update Systems** - Run after the main update
5. **Last Systems** - Run at the end of each frame

Systems are organized into the following schedules:

- `GameStartupSchedule` - Initialization
- `GamePreUpdateSchedule` - Pre-update logic
- `GameUpdateSchedule` - Main game logic
- `GamePostUpdateSchedule` - Post-update logic

Within each schedule, systems are further organized by execution order to ensure dependencies are satisfied. 