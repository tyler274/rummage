# Glossary

This glossary provides definitions for both Magic: The Gathering game terms and Rummage-specific technical terms.

## A

### Ability
A feature of a card that gives it certain characteristics or allows it to perform certain actions. There are three main types: activated, triggered, and static abilities.

### Activated Ability
An ability that a player can activate by paying its cost (e.g., tapping the card, paying mana).

### Active Player
The player whose turn it is currently.

### APNAP Order
The order in which players make decisions when multiple players need to take actions: Active Player, Non-Active Player.

### Attachment
A card or effect that modifies another card (e.g., Auras, Equipment).

## B

### Battlefield
The zone where permanents (lands, creatures, artifacts, enchantments, and planeswalkers) exist while in play.

### Bevy
The game engine used to build Rummage, featuring an entity-component-system (ECS) architecture.

### Bevy ECS
The entity-component-system architecture used in Bevy, which separates entities (objects), components (data), and systems (logic).

### Bundle
*(Bevy)* A collection of components that are commonly added to an entity together. In Bevy 0.15.x, many bundles are deprecated in favor of individual components.

## C

### Card Type
The classification of a Magic card (e.g., creature, instant, sorcery, land).

### Cast
To play a spell by putting it on the stack and paying its costs.

### Color Identity
The colors in a card's mana cost, color indicator, and rules text. In Commander, a card can only be included in a deck if its color identity is within the commander's color identity.

### Combat Phase
The phase of a turn where creatures can attack and block, consisting of Beginning of Combat, Declare Attackers, Declare Blockers, Combat Damage, and End of Combat steps.

### Commander
1. The legendary creature or planeswalker that leads a Commander deck.
2. A casual multiplayer format of Magic where each player builds a 100-card deck around a legendary creature or planeswalker.

### Commander Tax
The additional cost of {2} that must be paid for each time a commander has been cast from the command zone.

### Component
*(Bevy)* A piece of data that can be attached to an entity, representing a specific aspect of that entity.

### Context
*(Rummage)* An object containing information about the current game state, used when resolving effects.

### Counter (noun)
A marker placed on a card to track various characteristics (e.g., +1/+1 counters, loyalty counters).

### Counter (verb)
To respond to a spell or ability by preventing it from resolving.

## D

### Damage
A reduction in a creature's toughness or a player's life total. Unlike loss of life, damage can be prevented.

### Deck
A collection of cards a player brings to the game. In Commander, decks consist of 99 cards plus a commander.

### Deserialization
*(Rummage)* The process of converting serialized data back into game objects, used for loading games or processing network data.

### Deterministic RNG
*(Rummage)* A random number generator that produces the same sequence of values when initialized with the same seed, crucial for networked gameplay.

### Detrimental
*(Rummage)* When using Bevy's component lifetimes system, a tag indicating that an entity can exist without the marked component.

## E

### Effect
The result of a spell or ability resolving, which may change the game state.

### Entity
*(Bevy)* A unique identifier that can have components attached to it, representing objects in the game.

### Entity Component System (ECS)
*(Bevy)* A software architectural pattern that separates identity (entities), data (components), and behavior (systems).

### Exile
A zone where cards removed from the game are placed.

## F

### Flash
A keyword ability that allows a player to cast a spell any time they could cast an instant.

### Floating Mana
Mana in a player's mana pool that hasn't been spent yet. It disappears at the end of steps and phases.

## G

### Game State
The complete status of the game at a given moment, including all zones, cards, and player information.

### GlobalTransform
*(Bevy)* A component that stores the absolute position, rotation, and scale of an entity in world space.

### Graveyard
The zone where cards go when they're destroyed, sacrificed, discarded, or countered.

## H

### Hand
The zone where players hold cards they've drawn but haven't yet played.

### Haste
A keyword ability that allows a creature to attack and use tap abilities the turn it comes under a player's control.

## I

### Indestructible
A keyword ability that prevents a permanent from being destroyed by damage or effects that say "destroy."

### Instant
A card type that can be cast at any time, even during an opponent's turn.

### IsServer
*(Rummage)* A resource indicating whether the current instance is running as a server or client.

## K

### Keyword Ability
An ability that is represented by a single word or phrase (e.g., Flying, Trample, Deathtouch).

## L

### Library
The zone where a player's deck is kept during the game.

### Life Total
The amount of life a player has. In Commander, players typically start with 40 life.

### Legendary
A supertype that restricts a player to controlling only one copy of a specific legendary permanent at a time.

## M

### Mana
The resource used to cast spells and activate abilities, represented by the five colors (White, Blue, Black, Red, Green) and colorless.

### Mana Cost
The amount and type of mana required to cast a spell, shown in the upper right corner of a card.

### Mana Pool
A holding area where mana exists from the time it's generated until it's spent or lost.

### Marker Component
*(Bevy)* A component with no data that is used to tag entities for queries or to indicate a state.

### Mulligan
The act of drawing a new hand at the start of the game, with one fewer card each time.

## N

### NetworkConfig
*(Rummage)* A resource containing configuration for networking, such as ports and connection settings.

### Non-Active Player
Any player whose turn it is not.

## P

### Permanent
A card or token on the battlefield: creatures, artifacts, enchantments, lands, and planeswalkers.

### PendingSnapshots
*(Rummage)* A resource that tracks snapshots waiting to be processed.

### Phase
A segment of a turn. Each turn consists of five phases: Beginning, Precombat Main, Combat, Postcombat Main, and Ending.

### Plugin
*(Bevy)* A module that adds specific functionality to the game, typically containing resources, components, and systems.

### Priority
The right to take game actions, which passes between players throughout the turn.

## Q

### Query
*(Bevy)* A way to access entities and their components in systems, optionally filtered by component types.

## R

### Replicate
*(Rummage/Replicon)* A component that marks an entity for network replication.

### Replicon
*(Rummage)* The networking library used for multiplayer functionality.

### Resource
*(Bevy)* Global data not tied to any specific entity, accessed by systems.

### Response
Playing a spell or ability after another spell or ability has been put on the stack but before it resolves.

## S

### Serialization
*(Rummage)* The process of converting game objects into a format that can be stored or transmitted.

### Snapshot
*(Rummage)* A serialized representation of the game state at a specific point in time.

### SnapshotEvent
*(Rummage)* An event that triggers taking, applying, saving, or loading a game snapshot.

### Snapshotable
*(Rummage)* A marker component indicating that an entity should be included in snapshots.

### Sorcery
A card type that can only be cast during a player's main phase when the stack is empty.

### Stack
The zone where spells and abilities go while waiting to resolve.

### State-Based Action
Game rules that automatically check and update the game state, such as putting creatures with lethal damage into the graveyard.

### Step
A subdivision of a phase in a turn.

### System
*(Bevy)* A function that operates on components, implementing game logic.

### SystemSet
*(Bevy)* A collection of systems that can be configured together for ordering and dependencies.

## T

### Tap
To turn a card sideways to indicate it has been used.

### Target
An object or player chosen to be affected by a spell or ability.

### Transform
*(Bevy)* A component that stores the relative position, rotation, and scale of an entity.

### Triggered Ability
An ability that automatically triggers when a specific event occurs.

### Turn
A full cycle through all phases for a single player.

## U

### UI
User interface elements that display game information and accept player input.

### Untap
To return a tapped card to its upright position, typically done at the beginning of a player's turn.

### Update
*(Bevy)* The main schedule where most game systems run each frame.

## W

### Winning the Game
In Commander, a player wins by reducing all opponents' life totals to 0, dealing 21 or more combat damage with a single commander to a player, or through alternative win conditions on cards.

### World
*(Bevy)* The container for all entities, components, and resources in the ECS.

## Z

### Zone
A place where cards can exist during the game: library, hand, battlefield, graveyard, stack, exile, and command.

---

*Note: This glossary is continuously updated as new terms are added to the codebase or as Magic: The Gathering terminology evolves.* 