# Glossary

This glossary provides definitions for Magic: The Gathering terms, game engine concepts, and Bevy-related terminology used throughout the Rummage documentation.

## Magic: The Gathering Terms

### A

- **Ability**: A characteristic of an object that lets it affect the game. An ability can be a characteristic ability or an activated, triggered, or static ability.
- **Activated Ability**: An ability that requires a cost to be paid to use it.
- **Active Player**: The player whose turn it is.
- **Additional Cost**: A cost a spell may have that its controller may or may not pay.
- **Affinity**: A keyword ability that reduces how much mana you need to spend to cast a spell.

### B

- **Battlefield**: The zone where permanents exist.
- **Beginning Phase**: The first phase of a turn, consisting of the untap, upkeep, and draw steps.
- **Block**: To declare a creature as a blocker against an attacking creature.
- **Blocking Creature**: A creature that has been declared as a blocker against an attacking creature.

### C

- **Cast**: To take a card from the hand, pay its costs, and put it on the stack.
- **Color Identity**: The combined colors of mana symbols in a card's casting cost and rules text. Used in Commander format.
- **Combat Damage**: Damage dealt during the combat damage step by attacking creatures and blocking creatures as a result of combat.
- **Command Zone**: A special game zone where certain special objects exist during a game.
- **Commander**: A legendary creature card designated to lead a deck in the Commander format.
- **Commander Damage**: Combat damage dealt to a player by a commander. A player who has been dealt 21 or more combat damage by the same commander loses the game.

### D

- **Deck**: The collection of cards a player starts the game with.
- **Double Strike**: A keyword ability that causes a creature to deal both first-strike and regular combat damage.
- **Draw**: To put the top card of a player's library into their hand.

### E

- **Effect**: What happens when a spell or ability resolves.
- **Exile**: To put an object into the exile zone from wherever it is.
- **ETB**: "Enters the battlefield," referring to abilities that trigger when a permanent enters the battlefield.

### H

- **Hand**: The zone where a player holds cards they have drawn but not played yet.

### I

- **Instant**: A card type that can be cast at any time a player has priority.

### L

- **Library**: The zone where a player's deck is kept during a game.
- **Life Total**: The amount of life a player has. Players lose when this reaches 0.

### M

- **Mana**: The primary resource in the game, used to cast spells and activate abilities.
- **Mana Cost**: The cost to cast a spell, indicated by mana symbols in the upper right corner of a card.
- **Mulligan**: The process of setting aside an initial hand of cards and drawing a new one.

### P

- **Permanent**: A card or token that is on the battlefield. Lands, creatures, artifacts, enchantments, and planeswalkers are permanents.
- **Priority**: The system that determines which player is allowed to perform actions at any given time.

### R

- **Resolve**: To carry out the instructions of a spell or ability.

### S

- **Spell**: A card on the stack or a copy of a card on the stack.
- **Stack**: The zone where spells and abilities wait to resolve.
- **Static Ability**: An ability that is continuously affecting the game.

### T

- **Tap**: To turn a permanent sideways to show it has been used.
- **Target**: An object, player, or zone a spell or ability will affect.
- **Triggered Ability**: An ability that triggers when its trigger event happens.
- **Turn**: A unit of play, consisting of various phases and steps.

### Z

- **Zone**: An area where objects can be during a game.

## Game Engine Concepts

- **Component**: In ECS, data attached to entities that defines their properties.
- **Entity**: A unique identifier that represents a game object in the ECS pattern.
- **ECS**: Entity Component System, the architecture pattern used in Bevy.
- **Plugin**: A modular collection of systems, resources, and components in Bevy.
- **Query**: A request for entities that match specific component criteria.
- **Resource**: Global data accessible by systems in Bevy.
- **Snapshot**: A saved game state that can be restored later.
- **System**: Logic that operates on entities with specific components.

## Networking Terms

- **Replicon**: The networking library integrated with Bevy used in Rummage.
- **Rollback**: A technique in networking where game state is reverted and re-simulated when receiving authoritative updates.
- **Deterministic**: A system whose behavior is entirely predictable given the same inputs.
- **RNG Seed**: A value used to initialize a random number generator to produce a deterministic sequence.

## Testing Terms

- **Integration Test**: A test that verifies multiple components working together.
- **Unit Test**: A test that verifies a single component or function in isolation.
- **Visual Differential Testing**: A testing technique that compares visual outputs against expected results.
- **E2E Test**: End-to-end testing that validates complete workflows from start to finish.

---

This glossary will be updated as new terms are added to the codebase and documentation. 