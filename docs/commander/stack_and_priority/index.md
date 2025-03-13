# Stack and Priority

This section covers the implementation of the stack and priority system in the Commander format.

## Contents

- [The Stack](stack.md) - Implementation of the stack for spells and abilities
- [Priority Passing](priority_passing.md) - Priority system in multiplayer games
- [Stack Resolution](stack_resolution.md) - Resolution of objects on the stack

The stack and priority section defines how spells and abilities are processed in Commander games:

- Stack implementation for tracking spells and abilities
- Priority passing algorithm for multiplayer Commander games
- Special timing rules for Commander-specific abilities
- Stack resolution mechanics in complex multiplayer scenarios
- Handling triggered abilities from multiple players simultaneously

The multiplayer nature of Commander introduces unique challenges in stack and priority management, particularly when multiple players have simultaneous triggers or wish to respond to the same action. 