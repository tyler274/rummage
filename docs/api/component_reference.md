# Component Reference

This document provides a reference for all the components used in the Rummage game engine. Components are the building blocks of entities in Bevy's ECS architecture.

## Card Components

Components that represent cards and their attributes.

### Core Card Components

| Component | Description |
|-----------|-------------|
| `CardBase` | Core card information including name, types, and text |
| `ManaCost` | The mana cost associated with a card |
| `CardOwner` | Tracks which player owns the card |
| `CardController` | Tracks which player currently controls the card |
| `CardLocation` | Current zone location of the card |
| `CardState` | Current state of the card (tapped, face-down, etc.) |

### Card Type Components

| Component | Description |
|-----------|-------------|
| `Creature` | Marks an entity as a creature with power and toughness |
| `Land` | Marks an entity as a land card |
| `Artifact` | Marks an entity as an artifact |
| `Enchantment` | Marks an entity as an enchantment |
| `Planeswalker` | Marks an entity as a planeswalker with loyalty counters |
| `Instant` | Marks an entity as an instant |
| `Sorcery` | Marks an entity as a sorcery |

### Card Characteristic Components

| Component | Description |
|-----------|-------------|
| `CreatureStats` | Stores creature power, toughness, and damage |
| `PlaneswalkerLoyalty` | Tracks planeswalker loyalty counters |
| `CardSuperTypes` | Lists any super types (Legendary, Basic, etc.) |
| `CardSubTypes` | Lists any subtypes (Human, Wizard, Equipment, etc.) |
| `CardColors` | Colors associated with the card |
| `ColorIdentity` | Color identity of the card (for Commander format) |

## Zone Components

Components related to game zones.

| Component | Description |
|-----------|-------------|
| `InHand` | Marks a card as being in a player's hand |
| `InLibrary` | Marks a card as being in a player's library with position |
| `OnBattlefield` | Marks a card as being on the battlefield |
| `InGraveyard` | Marks a card as being in a graveyard |
| `InExile` | Marks a card as being in exile |
| `InStack` | Marks a card as being on the stack |
| `InCommandZone` | Marks a card as being in the command zone |

## Player Components

Components that represent player state.

| Component | Description |
|-----------|-------------|
| `Player` | Core player information |
| `PlayerLife` | Tracks player's life total |
| `ManaPool` | Tracks player's available mana |
| `CommanderDamage` | Tracks commander damage received |
| `PlayerHand` | Tracks the player's hand |
| `PlayerLibrary` | Tracks the player's library |
| `PlayerGraveyard` | Tracks the player's graveyard |
| `CommanderIdentity` | Tracks the player's commander and its details |

## Turn Components

Components related to turn structure and phases.

| Component | Description |
|-----------|-------------|
| `ActivePlayer` | Marks the player whose turn it is |
| `PriorityHolder` | Marks the player who currently has priority |
| `TurnNumber` | Keeps track of the current turn number |
| `PhaseTracker` | Tracks the current phase/step of the turn |
| `ExtraTurn` | Indicates a player will take an extra turn |

## Combat Components

Components related to combat.

| Component | Description |
|-----------|-------------|
| `Attacking` | Marks a creature as attacking |
| `Blocking` | Marks a creature as blocking |
| `AttackTarget` | Tracks what player or planeswalker is being attacked |
| `BlockTarget` | Tracks what creature is being blocked |
| `CombatDamageAssigned` | Tracks combat damage assignment |
| `FirstStrike` | Indicates a creature has first strike |
| `DoubleStrike` | Indicates a creature has double strike |
| `DamagePrevention` | Tracks damage prevention effects |

## Ability Components

Components related to card abilities.

| Component | Description |
|-----------|-------------|
| `ActivatedAbility` | Defines an activated ability on a card |
| `TriggeredAbility` | Defines a triggered ability on a card |
| `StaticAbility` | Defines a static ability on a card |
| `ReplacementEffect` | Defines a replacement effect |
| `AbilityCost` | The cost associated with an ability |
| `AbilityTarget` | Target requirements for an ability |

## Commander Format Components

Components specific to the Commander format.

| Component | Description |
|-----------|-------------|
| `Commander` | Marks a card as a commander |
| `CommanderTax` | Tracks the additional cost to cast a commander |
| `PartnerCommander` | Indicates a commander has the partner ability |
| `CommanderDamageSource` | Tracks damage dealt by a commander |
| `CommanderDamageReceived` | Tracks commander damage received by a player |

## Effect Components

Components related to ongoing effects.

| Component | Description |
|-----------|-------------|
| `ContinuousEffect` | Represents a continuous effect |
| `EffectDuration` | Duration of an effect (until end of turn, etc.) |
| `CounterModification` | Modifies counters on entities |
| `StatModification` | Modifies power/toughness or other stats |
| `AbilityGranted` | Indicates an ability granted by an effect |

## UI Components

Components related to visual presentation.

| Component | Description |
|-----------|-------------|
| `CardVisual` | Visual representation of a card |
| `Draggable` | Makes an entity draggable in the UI |
| `Droppable` | Makes an entity a valid drop target |
| `Hoverable` | Enables hover effects on an entity |
| `Selectable` | Makes an entity selectable |
| `VisualState` | Tracks the visual state of an entity |

## Network Components

Components related to multiplayer functionality.

| Component | Description |
|-----------|-------------|
| `NetworkIdentifier` | Unique identifier for networked entities |
| `NetworkOwner` | Tracks which player owns this entity in the network |
| `NetworkSynchronized` | Marks components that should sync over network |
| `NetworkReplication` | Controls how an entity is replicated | 