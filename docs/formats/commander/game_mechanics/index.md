# Game Mechanics

This section covers the core game mechanics specific to the Commander format implementation.

## Contents

- [Game State](game_state.md) - Core state tracking and management
- [State-Based Actions](state_based_actions.md) - Commander-specific state checks and automatic game actions
- [Triggered Abilities](triggered_abilities.md) - Commander-specific triggered abilities and effects
- [Random Elements](random_elements.md) - Testing random elements like coin flips and dice rolls
- [Testing Guide](testing_guide.md) - Special considerations for testing Commander mechanics

The game mechanics section defines how the Commander format's unique rules are implemented and enforced within our game engine, including:

- Commander-specific state tracking
- Unique state-based actions like commander damage checks
- Special triggered abilities for command zone interactions
- Format-specific mechanics like Lieutenant and Partner triggers
- Handling multiple player elimination and game completion
- Randomized mechanics like coin flips and dice rolls

These mechanics build on the foundational Magic: The Gathering rules while incorporating the unique aspects of the Commander format. The implementation is designed to be both accurate to the official rules and performant for multiplayer gameplay. 