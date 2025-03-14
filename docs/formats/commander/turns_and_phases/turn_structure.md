# Turn Structure

## Overview

The Turn Structure module manages the flow of a Commander game, handling the sequence of phases and steps within each player's turn. It coordinates player transitions, priority passing, and phase-specific actions while accounting for the multiplayer nature of Commander.

## Core Turn Sequence

A turn in Commander follows the standard Magic: The Gathering sequence:

1. **Beginning Phase**
   - Untap Step
   - Upkeep Step
   - Draw Step

2. **Precombat Main Phase**

3. **Combat Phase**
   - Beginning of Combat Step
   - Declare Attackers Step
   - Declare Blockers Step
   - Combat Damage Step
   - End of Combat Step

4. **Postcombat Main Phase**

5. **Ending Phase**
   - End Step
   - Cleanup Step

## Multiplayer Considerations

In Commander, turn order proceeds clockwise from the starting player. The format introduces special considerations:

- **Turn Order Determination**: Typically random at the start of the game
- **Player Elimination**: When a player loses, turns continue with the remaining players
- **Extra Turns**: Cards that grant extra turns work the same as in standard Magic
- **"Skip your next turn" effects**: These follow standard Magic rules but can have significant political impact

## Data Structures

The turn structure is managed through the following robust data structures:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Beginning(BeginningStep),
    Precombat(PrecombatStep),
    Combat(CombatStep),
    Postcombat(PostcombatStep),
    Ending(EndingStep),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeginningStep {
    Untap,
    Upkeep,
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrecombatStep {
    Main,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombatStep {
    Beginning,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrike,
    CombatDamage,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PostcombatStep {
    Main,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndingStep {
    End,
    Cleanup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtraTurnSource {
    Card(Entity),
    Ability(Entity, Entity), // (Source, Ability)
    Rule,
}

#[derive(Resource)]
pub struct TurnManager {
    // Current turn information
    pub current_phase: Phase,
    pub active_player_index: usize,
    pub turn_number: u32,
    
    // Priority system
    pub priority_player_index: usize,
    pub all_players_passed: bool,
    pub stack_is_empty: bool,
    
    // Multiplayer tracking
    pub player_order: Vec<Entity>,
    pub extra_turns: VecDeque<(Entity, ExtraTurnSource)>,
    pub skipped_turns: HashSet<Entity>,
    
    // Time tracking
    pub time_limits: HashMap<Entity, Duration>,
    pub turn_started_at: Option<Instant>,
    
    // Phase history for reverting game states and debugging
    pub phase_history: VecDeque<(Phase, Entity)>,
    
    // Special phase tracking
    pub additional_phases: VecDeque<(Phase, Entity)>,
    pub modified_phases: HashMap<Phase, PhaseModification>,
}

#[derive(Debug, Clone)]
pub struct PhaseModification {
    pub source: Entity,
    pub skip_priority: bool,
    pub skip_actions: bool,
    pub additional_actions: Vec<GameAction>,
}
```

## Detailed Implementation

### Phase Progression System

```rust
impl TurnManager {
    // Advances to the next phase in the turn sequence
    pub fn advance_phase(&mut self) -> Result<Phase, TurnError> {
        let previous_phase = self.current_phase;
        self.current_phase = match self.current_phase {
            Phase::Beginning(BeginningStep::Untap) => Phase::Beginning(BeginningStep::Upkeep),
            Phase::Beginning(BeginningStep::Upkeep) => Phase::Beginning(BeginningStep::Draw),
            Phase::Beginning(BeginningStep::Draw) => Phase::Precombat(PrecombatStep::Main),
            Phase::Precombat(PrecombatStep::Main) => Phase::Combat(CombatStep::Beginning),
            Phase::Combat(CombatStep::Beginning) => Phase::Combat(CombatStep::DeclareAttackers),
            Phase::Combat(CombatStep::DeclareAttackers) => Phase::Combat(CombatStep::DeclareBlockers),
            // Check if first strike damage should happen
            Phase::Combat(CombatStep::DeclareBlockers) => {
                if self.has_first_strike_creatures() {
                    Phase::Combat(CombatStep::FirstStrike)
                } else {
                    Phase::Combat(CombatStep::CombatDamage)
                }
            },
            Phase::Combat(CombatStep::FirstStrike) => Phase::Combat(CombatStep::CombatDamage),
            Phase::Combat(CombatStep::CombatDamage) => Phase::Combat(CombatStep::End),
            Phase::Combat(CombatStep::End) => Phase::Postcombat(PostcombatStep::Main),
            Phase::Postcombat(PostcombatStep::Main) => Phase::Ending(EndingStep::End),
            Phase::Ending(EndingStep::End) => Phase::Ending(EndingStep::Cleanup),
            Phase::Ending(EndingStep::Cleanup) => {
                // Check for additional or extra turns
                self.process_turn_end()?;
                // The next turn's first phase
                Phase::Beginning(BeginningStep::Untap)
            }
        };
        
        // Record phase change in history
        self.phase_history.push_back((previous_phase, self.get_active_player()));
        
        // Check if this phase should be modified or skipped
        if let Some(modification) = self.modified_phases.get(&self.current_phase) {
            if modification.skip_actions {
                // Skip this phase and move to the next one
                return self.advance_phase();
            }
        }
        
        // Handle priority assignment for the new phase
        self.reset_priority_for_new_phase();
        
        Ok(self.current_phase)
    }
    
    // Process the end of a turn
    fn process_turn_end(&mut self) -> Result<(), TurnError> {
        // Check if there are additional phases for this turn
        if let Some((phase, _)) = self.additional_phases.pop_front() {
            self.current_phase = phase;
            return Ok(());
        }
        
        // Move to the next player
        let next_player = self.determine_next_player()?;
        self.active_player_index = self.player_order.iter()
            .position(|&p| p == next_player)
            .ok_or(TurnError::InvalidPlayerIndex)?;
        self.turn_number += 1;
        
        // Reset turn timer
        self.turn_started_at = Some(Instant::now());
        
        Ok(())
    }
    
    // Determine who takes the next turn
    fn determine_next_player(&mut self) -> Result<Entity, TurnError> {
        // Check for extra turns first
        if let Some((player, source)) = self.extra_turns.pop_front() {
            // Log extra turn for game record
            info!("Player {:?} is taking an extra turn from source {:?}", player, source);
            return Ok(player);
        }
        
        // Get the next player in turn order
        let next_index = (self.active_player_index + 1) % self.player_order.len();
        let next_player = self.player_order[next_index];
        
        // Check if the next player should skip their turn
        if self.skipped_turns.remove(&next_player) {
            // Record the skipped turn
            info!("Player {:?} is skipping their turn", next_player);
            
            // Move to the player after that
            self.active_player_index = next_index;
            return self.determine_next_player();
        }
        
        Ok(next_player)
    }
}
```

### Priority System

```rust
impl TurnManager {
    // Reset priority for a new phase
    fn reset_priority_for_new_phase(&mut self) {
        self.all_players_passed = false;
        
        // In most phases, active player gets priority first
        self.priority_player_index = self.active_player_index;
        
        // For Beginning::Untap and Ending::Cleanup, no player gets priority by default
        match self.current_phase {
            Phase::Beginning(BeginningStep::Untap) | Phase::Ending(EndingStep::Cleanup) => {
                self.all_players_passed = true;
            },
            _ => {}
        }
    }
    
    // Pass priority to the next player
    pub fn pass_priority(&mut self) -> Result<(), TurnError> {
        // Calculate next player with priority
        let next_index = (self.priority_player_index + 1) % self.player_order.len();
        
        // If we're back to the active player, everyone has passed
        if next_index == self.active_player_index {
            self.all_players_passed = true;
            
            // If the stack is empty and everyone has passed, move to the next phase
            if self.stack_is_empty {
                self.advance_phase()?;
            } else {
                // Resolve the top item on the stack
                // This would trigger a different system
                self.stack_is_empty = false; // This would be set by the stack system
            }
        } else {
            // Pass to the next player
            self.priority_player_index = next_index;
        }
        
        Ok(())
    }
}
```

## Edge Cases and Their Handling

### Player Elimination During a Turn

```rust
impl TurnManager {
    // Handle a player being eliminated
    pub fn handle_player_elimination(&mut self, eliminated_player: Entity) -> Result<(), TurnError> {
        // Remove player from the turn order
        let index = self.player_order.iter()
            .position(|&p| p == eliminated_player)
            .ok_or(TurnError::PlayerNotFound)?;
        self.player_order.remove(index);
        
        // Adjust active player index if necessary
        if index <= self.active_player_index && self.active_player_index > 0 {
            self.active_player_index -= 1;
        }
        
        // Adjust priority player index if necessary
        if index <= self.priority_player_index && self.priority_player_index > 0 {
            self.priority_player_index -= 1;
        }
        
        // Remove any extra or skipped turns for this player
        self.extra_turns.retain(|(player, _)| *player != eliminated_player);
        self.skipped_turns.remove(&eliminated_player);
        
        // If the active player was eliminated
        if index == self.active_player_index {
            // Move to the next player's turn
            let next_player = self.determine_next_player()?;
            self.active_player_index = self.player_order.iter()
                .position(|&p| p == next_player)
                .ok_or(TurnError::InvalidPlayerIndex)?;
            
            // Reset to the beginning of the turn
            self.current_phase = Phase::Beginning(BeginningStep::Untap);
            self.reset_priority_for_new_phase();
        }
        
        Ok(())
    }
}
```

### Handling Multiple Extra Turns

When multiple extra turns are queued, they're processed in the order they were created:

```rust
impl TurnManager {
    // Add an extra turn to the queue
    pub fn add_extra_turn(&mut self, player: Entity, source: ExtraTurnSource) {
        self.extra_turns.push_back((player, source));
    }
}
```

### Time Limits and Slow Play

```rust
impl TurnManager {
    // Check if a player has exceeded their time limit
    pub fn check_time_limit(&self, player: Entity) -> Result<bool, TurnError> {
        if let Some(limit) = self.time_limits.get(&player) {
            if let Some(start_time) = self.turn_started_at {
                return Ok(start_time.elapsed() > *limit);
            }
        }
        Ok(false)
    }
}
```

## Verification and Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test normal phase progression
    #[test]
    fn test_normal_phase_progression() {
        let mut turn_manager = setup_test_turn_manager();
        
        // Start at Beginning::Untap
        assert_eq!(turn_manager.current_phase, Phase::Beginning(BeginningStep::Untap));
        
        // Advance through all phases
        for _ in 0..12 {  // Total number of phases/steps in a turn
            turn_manager.advance_phase().unwrap();
        }
        
        // Should be back at Beginning::Untap for the next player
        assert_eq!(turn_manager.current_phase, Phase::Beginning(BeginningStep::Untap));
        assert_eq!(turn_manager.active_player_index, 1); // Next player
    }
    
    // Test extra turn handling
    #[test]
    fn test_extra_turn() {
        let mut turn_manager = setup_test_turn_manager();
        let player1 = turn_manager.player_order[0];
        
        // Add an extra turn for the current player
        turn_manager.add_extra_turn(player1, ExtraTurnSource::Rule);
        
        // Advance through a full turn
        for _ in 0..12 {
            turn_manager.advance_phase().unwrap();
        }
        
        // Should still be the same player's turn
        assert_eq!(turn_manager.active_player_index, 0);
    }
    
    // Test skipped turn
    #[test]
    fn test_skipped_turn() {
        let mut turn_manager = setup_test_turn_manager();
        let player2 = turn_manager.player_order[1];
        
        // Mark player 2's turn to be skipped
        turn_manager.skipped_turns.insert(player2);
        
        // Advance through a full turn
        for _ in 0..12 {
            turn_manager.advance_phase().unwrap();
        }
        
        // Should skip to player 3
        assert_eq!(turn_manager.active_player_index, 2);
    }
    
    // Test player elimination
    #[test]
    fn test_player_elimination() {
        let mut turn_manager = setup_test_turn_manager();
        let player2 = turn_manager.player_order[1];
        
        // Eliminate player 2
        turn_manager.handle_player_elimination(player2).unwrap();
        
        // Check player order is updated
        assert_eq!(turn_manager.player_order.len(), 3);
        assert!(!turn_manager.player_order.contains(&player2));
        
        // Complete current turn
        for _ in 0..12 {
            turn_manager.advance_phase().unwrap();
        }
        
        // Should skip from player 1 to player 3 (now at index 1)
        assert_eq!(turn_manager.active_player_index, 1);
    }
    
    // Test priority passing
    #[test]
    fn test_priority_passing() {
        let mut turn_manager = setup_test_turn_manager();
        
        // Move to a phase where priority is assigned
        turn_manager.advance_phase().unwrap(); // To Beginning::Upkeep
        
        // Active player should have priority
        assert_eq!(turn_manager.priority_player_index, 0);
        
        // Pass priority to each player
        for i in 1..4 {
            turn_manager.pass_priority().unwrap();
            assert_eq!(turn_manager.priority_player_index, i);
        }
        
        // After all players pass, should move to next phase
        turn_manager.pass_priority().unwrap();
        assert_eq!(turn_manager.current_phase, Phase::Beginning(BeginningStep::Draw));
    }
    
    // Setup helper
    fn setup_test_turn_manager() -> TurnManager {
        let player1 = Entity::from_raw(1);
        let player2 = Entity::from_raw(2);
        let player3 = Entity::from_raw(3);
        let player4 = Entity::from_raw(4);
        
        TurnManager {
            current_phase: Phase::Beginning(BeginningStep::Untap),
            active_player_index: 0,
            turn_number: 1,
            priority_player_index: 0,
            all_players_passed: true,
            stack_is_empty: true,
            player_order: vec![player1, player2, player3, player4],
            extra_turns: VecDeque::new(),
            skipped_turns: HashSet::new(),
            time_limits: HashMap::new(),
            turn_started_at: Some(Instant::now()),
            phase_history: VecDeque::new(),
            additional_phases: VecDeque::new(),
            modified_phases: HashMap::new(),
        }
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::game_engine::stack::Stack;
    use crate::game_engine::state::GameState;
    
    // Test turns and stack interaction
    #[test]
    fn test_turns_with_stack() {
        let mut app = App::new();
        
        // Set up game resources
        app.insert_resource(setup_test_turn_manager());
        app.insert_resource(Stack::default());
        app.insert_resource(GameState::default());
        
        // Add relevant systems
        app.add_systems(Update, (
            phase_transition_system,
            stack_resolution_system,
            priority_system,
        ));
        
        // Simulate a turn with stack interactions
        app.update();
        
        // Add a spell to the stack
        let mut stack = app.world.resource_mut::<Stack>();
        stack.push(create_test_spell());
        
        // Verify stack is not empty
        let turn_manager = app.world.resource::<TurnManager>();
        assert!(!turn_manager.stack_is_empty);
        
        // Simulate priority passing and stack resolution
        for _ in 0..5 {
            app.update();
        }
        
        // Stack should be empty and phase should have advanced
        let turn_manager = app.world.resource::<TurnManager>();
        assert!(turn_manager.stack_is_empty);
        assert_eq!(turn_manager.current_phase, Phase::Beginning(BeginningStep::Draw));
    }
    
    // Test full turn cycle with multiple players
    #[test]
    fn test_full_turn_cycle() {
        let mut app = App::new();
        
        // Set up game resources
        app.insert_resource(setup_test_turn_manager());
        
        // Add systems
        app.add_systems(Update, phase_transition_system);
        
        // Track starting player
        let start_player = app.world.resource::<TurnManager>().active_player_index;
        
        // Simulate a full turn cycle (all players take one turn)
        for _ in 0..48 {  // 4 players × 12 phases
            app.update();
        }
        
        // Should be back to starting player
        let turn_manager = app.world.resource::<TurnManager>();
        assert_eq!(turn_manager.active_player_index, start_player);
        assert_eq!(turn_manager.turn_number, 5); // Starting turn + 4 players
    }
}
```

### End-to-End Testing

```rust
#[cfg(test)]
mod e2e_tests {
    use super::*;
    use crate::test_utils::setup_full_game;
    
    // Test a full game scenario
    #[test]
    fn test_game_until_winner() {
        let mut app = setup_full_game(4); // 4 players
        
        // Run the game until only one player remains
        let max_turns = 100; // Prevent infinite loop
        for _ in 0..max_turns {
            app.update();
            
            // Check if the game has ended
            let turn_manager = app.world.resource::<TurnManager>();
            if turn_manager.player_order.len() == 1 {
                break;
            }
            
            // Simulate random player actions based on the current phase
            simulate_player_actions(&mut app);
        }
        
        // Verify we have a winner
        let turn_manager = app.world.resource::<TurnManager>();
        assert_eq!(turn_manager.player_order.len(), 1, "Game should end with one player remaining");
    }
    
    // Test complex turn interaction with time limits
    #[test]
    fn test_time_limits() {
        let mut app = setup_full_game(4);
        
        // Set time limits for all players
        {
            let mut turn_manager = app.world.resource_mut::<TurnManager>();
            for &player in &turn_manager.player_order {
                turn_manager.time_limits.insert(player, Duration::from_secs(30));
            }
        }
        
        // Fast-forward time for current player
        let current_player = {
            let turn_manager = app.world.resource::<TurnManager>();
            turn_manager.player_order[turn_manager.active_player_index]
        };
        
        // Mock time advancing past the limit for testing
        {
            let mut turn_manager = app.world.resource_mut::<TurnManager>();
            turn_manager.turn_started_at = Some(Instant::now() - Duration::from_secs(31));
        }
        
        // Add system to check time limits
        app.add_systems(Update, check_time_limits_system);
        
        // Run one update
        app.update();
        
        // Verify player's turn was ended due to time limit
        let turn_manager = app.world.resource::<TurnManager>();
        assert_ne!(turn_manager.active_player_index, 
                  turn_manager.player_order.iter().position(|&p| p == current_player).unwrap());
    }
}
```

## Performance Considerations

- The `TurnManager` uses efficient data structures to minimize overhead:
  - `HashMap` for O(1) lookup of player time limits
  - `VecDeque` for O(1) queue operations for extra turns and phase history
  - `HashSet` for O(1) lookup of skipped turns

- Phase transitions are optimized to only trigger necessary game events

- Time complexity analysis:
  - Advancing phase: O(1)
  - Determining next player: O(1) in most cases, O(n) worst case (all players skip)
  - Priority passing: O(1)
  - Player elimination: O(n) where n is the number of players

## Extensibility

The design allows for easy extension with new phases or rules:

- Additional phases can be added to the `Phase` enum
- Custom phase progression can be implemented by overriding the `advance_phase` method
- Game variants can be supported by modifying the `TurnManager` behavior

By following this implementation approach, the turn structure will robustly handle all standard Magic: The Gathering rules as well as the specific requirements of the Commander format. 