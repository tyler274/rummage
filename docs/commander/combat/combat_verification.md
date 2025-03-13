# Combat Verification

## Overview

A robust verification framework is critical for ensuring that the Combat System correctly implements the complex rules of Magic: The Gathering's Commander format. This document outlines our comprehensive testing approach, which covers unit testing, integration testing, system testing, and end-to-end testing strategies to verify both basic functionality and edge cases.

## Testing Framework Architecture

```rust
// Test utility for setting up combat scenarios
pub struct CombatScenarioBuilder {
    pub app: App,
    pub active_player: Entity,
    pub players: Vec<Entity>,
    pub attackers: Vec<Entity>,
    pub blockers: Vec<Entity>,
    pub effects: Vec<(Entity, Effect)>,
}

impl CombatScenarioBuilder {
    pub fn new() -> Self {
        let mut app = App::new();
        
        // Add essential plugins and systems
        app.add_plugins(MinimalPlugins)
           .add_plugins(TestTurnSystemPlugin)
           .add_plugins(TestCombatSystemPlugin);
        
        // Initialize resources
        app.insert_resource(CombatSystem::default());
        app.insert_resource(TurnManager::default());
        
        // Create default active player
        let active_player = app.world.spawn(Player {
            life_total: 40,
            commander_damage: HashMap::new(),
            ..Default::default()
        }).id();
        
        Self {
            app,
            active_player,
            players: vec![active_player],
            attackers: Vec::new(),
            blockers: Vec::new(),
            effects: Vec::new(),
        }
    }
    
    // Add a player to the scenario
    pub fn add_player(&mut self) -> Entity {
        let player = self.app.world.spawn(Player {
            life_total: 40,
            commander_damage: HashMap::new(),
            ..Default::default()
        }).id();
        
        self.players.push(player);
        player
    }
    
    // Add an attacking creature
    pub fn add_attacker(&mut self, 
                        power: u32, 
                        toughness: u32, 
                        controller: Entity, 
                        is_commander: bool) -> Entity {
        let mut components = (
            Card::default(),
            Creature {
                power,
                toughness,
                attacking: None,
                blocking: Vec::new(),
                ..Default::default()
            },
            Controllable { controller },
        );
        
        let entity = if is_commander {
            self.app.world.spawn((components, Commander)).id()
        } else {
            self.app.world.spawn(components).id()
        };
        
        self.attackers.push(entity);
        entity
    }
    
    // Add a blocking creature
    pub fn add_blocker(&mut self, 
                      power: u32, 
                      toughness: u32, 
                      controller: Entity) -> Entity {
        let entity = self.app.world.spawn((
            Card::default(),
            Creature {
                power,
                toughness,
                attacking: None,
                blocking: Vec::new(),
                ..Default::default()
            },
            Controllable { controller },
        )).id();
        
        self.blockers.push(entity);
        entity
    }
    
    // Add an effect to an entity
    pub fn add_effect(&mut self, entity: Entity, effect: Effect) {
        self.effects.push((entity, effect));
        
        // Apply the effect to the entity
        let mut effects = self.app.world.entity_mut(entity)
            .get_mut::<ActiveEffects>()
            .unwrap_or_else(|| {
                self.app.world.entity_mut(entity).insert(ActiveEffects(Vec::new()));
                self.app.world.entity_mut(entity).get_mut::<ActiveEffects>().unwrap()
            });
        
        effects.0.push(effect);
    }
    
    // Set up attacks
    pub fn declare_attacks(&mut self, attacks: Vec<(Entity, Entity)>) {
        let mut combat_system = self.app.world.resource_mut::<CombatSystem>();
        
        for (attacker, defender) in attacks {
            // Update creature component
            let mut attacker_entity = self.app.world.entity_mut(attacker);
            let mut creature = attacker_entity.get_mut::<Creature>().unwrap();
            creature.attacking = Some(defender);
            
            // Check if attacker is a commander
            let is_commander = attacker_entity.contains::<Commander>();
            
            // Add to combat system
            combat_system.attackers.insert(attacker, AttackData {
                attacker,
                defender,
                is_commander,
                requirements: Vec::new(),
                restrictions: Vec::new(),
            });
        }
    }
    
    // Set up blocks
    pub fn declare_blocks(&mut self, blocks: Vec<(Entity, Vec<Entity>)>) {
        let mut combat_system = self.app.world.resource_mut::<CombatSystem>();
        
        for (blocker, blocked_attackers) in blocks {
            // Update creature component
            let mut blocker_entity = self.app.world.entity_mut(blocker);
            let mut creature = blocker_entity.get_mut::<Creature>().unwrap();
            creature.blocking = blocked_attackers.clone();
            
            // Add to combat system
            combat_system.blockers.insert(blocker, BlockData {
                blocker,
                blocked_attackers,
                requirements: Vec::new(),
                restrictions: Vec::new(),
            });
        }
    }
    
    // Advance to a specific combat step
    pub fn advance_to(&mut self, step: CombatStep) {
        let mut turn_manager = self.app.world.resource_mut::<TurnManager>();
        turn_manager.current_phase = Phase::Combat(step);
        
        let mut combat_system = self.app.world.resource_mut::<CombatSystem>();
        combat_system.active_combat_step = Some(step);
    }
    
    // Execute combat and return results
    pub fn execute(&mut self) -> CombatResult {
        // Run the relevant systems based on the current step
        match self.app.world.resource::<CombatSystem>().active_combat_step {
            Some(CombatStep::Beginning) => {
                self.app.update_with_system(beginning_of_combat_system);
            }
            Some(CombatStep::DeclareAttackers) => {
                self.app.update_with_system(declare_attackers_system);
            }
            Some(CombatStep::DeclareBlockers) => {
                self.app.update_with_system(declare_blockers_system);
            }
            Some(CombatStep::FirstStrike) => {
                self.app.update_with_system(first_strike_damage_system);
            }
            Some(CombatStep::CombatDamage) => {
                self.app.update_with_system(apply_combat_damage_system);
            }
            Some(CombatStep::End) => {
                self.app.update_with_system(end_of_combat_system);
            }
            None => {
                // Run all systems in sequence
                self.advance_to(CombatStep::Beginning);
                self.app.update_with_system(beginning_of_combat_system);
                
                self.advance_to(CombatStep::DeclareAttackers);
                self.app.update_with_system(declare_attackers_system);
                
                self.advance_to(CombatStep::DeclareBlockers);
                self.app.update_with_system(declare_blockers_system);
                
                // Check if first strike is needed
                if self.has_first_strike_creatures() {
                    self.advance_to(CombatStep::FirstStrike);
                    self.app.update_with_system(first_strike_damage_system);
                }
                
                self.advance_to(CombatStep::CombatDamage);
                self.app.update_with_system(apply_combat_damage_system);
                
                self.advance_to(CombatStep::End);
                self.app.update_with_system(end_of_combat_system);
            }
        }
        
        // Collect and return results
        self.collect_combat_results()
    }
    
    // Helper to check if any creatures have first strike
    fn has_first_strike_creatures(&self) -> bool {
        for attacker in &self.attackers {
            if let Ok(creature) = self.app.world.entity(*attacker).get::<Creature>() {
                if creature.has_ability(Ability::Keyword(Keyword::FirstStrike)) || 
                   creature.has_ability(Ability::Keyword(Keyword::DoubleStrike)) {
                    return true;
                }
            }
        }
        
        for blocker in &self.blockers {
            if let Ok(creature) = self.app.world.entity(*blocker).get::<Creature>() {
                if creature.has_ability(Ability::Keyword(Keyword::FirstStrike)) || 
                   creature.has_ability(Ability::Keyword(Keyword::DoubleStrike)) {
                    return true;
                }
            }
        }
        
        false
    }
    
    // Collect results of combat
    fn collect_combat_results(&self) -> CombatResult {
        let mut result = CombatResult::default();
        
        // Collect player life totals and commander damage
        for player in &self.players {
            if let Ok(player_component) = self.app.world.entity(*player).get::<Player>() {
                result.player_life.insert(*player, player_component.life_total);
                result.commander_damage.insert(*player, player_component.commander_damage.clone());
            }
        }
        
        // Collect creature status
        for creature in self.attackers.iter().chain(self.blockers.iter()) {
            if let Ok(creature_component) = self.app.world.entity(*creature).get::<Creature>() {
                result.creature_status.insert(*creature, CreatureStatus {
                    power: creature_component.power,
                    toughness: creature_component.toughness,
                    damage: creature_component.damage,
                    destroyed: self.app.world.entity(*creature).contains::<Destroyed>(),
                    tapped: self.app.world.entity(*creature).contains::<Tapped>(),
                });
            }
        }
        
        // Collect combat events
        let combat_system = self.app.world.resource::<CombatSystem>();
        result.combat_events = combat_system.combat_history.clone();
        
        result
    }
}

// Structure to hold combat test results
#[derive(Default)]
pub struct CombatResult {
    pub player_life: HashMap<Entity, i32>,
    pub commander_damage: HashMap<Entity, HashMap<Entity, u32>>,
    pub creature_status: HashMap<Entity, CreatureStatus>,
    pub combat_events: VecDeque<CombatEvent>,
}

#[derive(Default)]
pub struct CreatureStatus {
    pub power: u32,
    pub toughness: u32,
    pub damage: u32,
    pub destroyed: bool,
    pub tapped: bool,
}
```

## Unit Testing

### Combat Steps Tests

Each combat step has dedicated unit tests to verify its functionality in isolation:

```rust
#[cfg(test)]
mod beginning_of_combat_tests {
    use super::*;
    
    #[test]
    fn test_beginning_of_combat_initialization() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Add some creatures
        let opponent = builder.add_player();
        builder.add_attacker(2, 2, builder.active_player, false);
        builder.add_attacker(3, 3, builder.active_player, true); // Commander
        
        // Set up the beginning of combat step
        builder.advance_to(CombatStep::Beginning);
        
        // Execute and get results
        let result = builder.execute();
        
        // Verify combat system is properly initialized
        let combat_system = builder.app.world.resource::<CombatSystem>();
        assert_eq!(combat_system.active_combat_step, Some(CombatStep::Beginning));
        
        // Verify begin combat event was recorded
        assert!(result.combat_events.iter().any(|event| 
            matches!(event, CombatEvent::BeginCombat { .. })));
    }
    
    // Additional tests...
}

#[cfg(test)]
mod declare_attackers_tests {
    use super::*;
    
    #[test]
    fn test_declare_attackers_basic() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Add some creatures and a player to attack
        let opponent = builder.add_player();
        let attacker1 = builder.add_attacker(2, 2, builder.active_player, false);
        let attacker2 = builder.add_attacker(3, 3, builder.active_player, true); // Commander
        
        // Set up attacks
        builder.advance_to(CombatStep::DeclareAttackers);
        builder.declare_attacks(vec![
            (attacker1, opponent),
            (attacker2, opponent),
        ]);
        
        // Execute and get results
        let result = builder.execute();
        
        // Verify creatures are marked as attacking
        let combat_system = builder.app.world.resource::<CombatSystem>();
        assert_eq!(combat_system.attackers.len(), 2);
        
        // Verify attack events were recorded
        assert_eq!(
            result.combat_events.iter()
                .filter(|event| matches!(event, CombatEvent::AttackDeclared { .. }))
                .count(),
            2
        );
        
        // Verify creatures are tapped
        for (entity, status) in &result.creature_status {
            assert!(status.tapped);
        }
    }
    
    // Additional tests...
}

// Similar unit tests for other combat steps...
```

### Edge Case Tests

Dedicated tests for all identified edge cases:

```rust
#[cfg(test)]
mod edge_case_tests {
    use super::*;
    
    #[test]
    fn test_indestructible_creatures() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Setup creatures
        let opponent = builder.add_player();
        let attacker = builder.add_attacker(2, 2, builder.active_player, false);
        let blocker = builder.add_blocker(4, 4, opponent);
        
        // Make attacker indestructible
        builder.add_effect(attacker, Effect::Indestructible);
        
        // Set up combat
        builder.advance_to(CombatStep::DeclareAttackers);
        builder.declare_attacks(vec![(attacker, opponent)]);
        
        builder.advance_to(CombatStep::DeclareBlockers);
        builder.declare_blocks(vec![(blocker, vec![attacker])]);
        
        // Execute full combat
        builder.advance_to(CombatStep::CombatDamage);
        let result = builder.execute();
        
        // Verify attacker received lethal damage but wasn't destroyed
        let attacker_status = result.creature_status.get(&attacker).unwrap();
        assert_eq!(attacker_status.damage, 4); // Received damage
        assert!(!attacker_status.destroyed); // But not destroyed
        
        // Verify blocker took damage
        let blocker_status = result.creature_status.get(&blocker).unwrap();
        assert_eq!(blocker_status.damage, 2);
        assert!(!blocker_status.destroyed);
    }
    
    #[test]
    fn test_deathtouch() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Setup creatures
        let opponent = builder.add_player();
        let attacker = builder.add_attacker(1, 1, builder.active_player, false);
        let blocker = builder.add_blocker(10, 10, opponent);
        
        // Add deathtouch to attacker
        builder.add_effect(attacker, Effect::Keyword(Keyword::Deathtouch));
        
        // Set up combat
        builder.advance_to(CombatStep::DeclareAttackers);
        builder.declare_attacks(vec![(attacker, opponent)]);
        
        builder.advance_to(CombatStep::DeclareBlockers);
        builder.declare_blocks(vec![(blocker, vec![attacker])]);
        
        // Execute full combat
        builder.advance_to(CombatStep::CombatDamage);
        let result = builder.execute();
        
        // Verify blocker was destroyed by deathtouch
        let blocker_status = result.creature_status.get(&blocker).unwrap();
        assert!(blocker_status.destroyed);
        
        // Verify attacker was also destroyed 
        let attacker_status = result.creature_status.get(&attacker).unwrap();
        assert!(attacker_status.destroyed);
    }
    
    #[test]
    fn test_trample() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Setup creatures
        let opponent = builder.add_player();
        let attacker = builder.add_attacker(6, 6, builder.active_player, false);
        let blocker = builder.add_blocker(2, 2, opponent);
        
        // Add trample to attacker
        builder.add_effect(attacker, Effect::Keyword(Keyword::Trample));
        
        // Set up combat
        builder.advance_to(CombatStep::DeclareAttackers);
        builder.declare_attacks(vec![(attacker, opponent)]);
        
        builder.advance_to(CombatStep::DeclareBlockers);
        builder.declare_blocks(vec![(blocker, vec![attacker])]);
        
        // Execute full combat
        builder.advance_to(CombatStep::CombatDamage);
        let result = builder.execute();
        
        // Verify opponent took trample damage
        assert_eq!(result.player_life[&opponent], 40 - 4); // 6 power - 2 toughness = 4 damage
        
        // Verify blocker was destroyed
        let blocker_status = result.creature_status.get(&blocker).unwrap();
        assert!(blocker_status.destroyed);
    }
    
    #[test]
    fn test_multiple_blockers() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Setup creatures
        let opponent = builder.add_player();
        let attacker = builder.add_attacker(5, 5, builder.active_player, false);
        let blocker1 = builder.add_blocker(2, 2, opponent);
        let blocker2 = builder.add_blocker(2, 2, opponent);
        
        // Set up combat
        builder.advance_to(CombatStep::DeclareAttackers);
        builder.declare_attacks(vec![(attacker, opponent)]);
        
        builder.advance_to(CombatStep::DeclareBlockers);
        builder.declare_blocks(vec![
            (blocker1, vec![attacker]),
            (blocker2, vec![attacker]),
        ]);
        
        // Add damage assignment order
        let mut combat_system = builder.app.world.resource_mut::<CombatSystem>();
        combat_system.damage_assignment_order = vec![blocker1, blocker2];
        
        // Execute full combat
        builder.advance_to(CombatStep::CombatDamage);
        let result = builder.execute();
        
        // Verify both blockers were destroyed
        let blocker1_status = result.creature_status.get(&blocker1).unwrap();
        assert!(blocker1_status.destroyed);
        
        let blocker2_status = result.creature_status.get(&blocker2).unwrap();
        assert!(blocker2_status.destroyed);
        
        // Verify attacker took damage from both blockers
        let attacker_status = result.creature_status.get(&attacker).unwrap();
        assert_eq!(attacker_status.damage, 4);
        assert!(!attacker_status.destroyed);
        
        // Verify player took no damage
        assert_eq!(result.player_life[&opponent], 40);
    }
    
    // Many more edge case tests...
}
```

## Integration Tests

Integration tests verify that different parts of the combat system work together correctly:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_combat_sequence() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Setup a complex combat scenario
        let opponent = builder.add_player();
        let attacker1 = builder.add_attacker(3, 3, builder.active_player, false);
        let attacker2 = builder.add_attacker(4, 4, builder.active_player, true); // Commander
        let blocker1 = builder.add_blocker(2, 2, opponent);
        let blocker2 = builder.add_blocker(5, 5, opponent);
        
        // Add some abilities
        builder.add_effect(attacker1, Effect::Keyword(Keyword::Trample));
        builder.add_effect(blocker2, Effect::Keyword(Keyword::Deathtouch));
        
        // Run through the entire combat sequence
        builder.declare_attacks(vec![
            (attacker1, opponent),
            (attacker2, opponent),
        ]);
        
        builder.declare_blocks(vec![
            (blocker1, vec![attacker1]),
            (blocker2, vec![attacker2]),
        ]);
        
        // Execute and get results
        let result = builder.execute();
        
        // Verify final state
        
        // Attacker1 should have dealt trample damage
        assert_eq!(result.player_life[&opponent], 40 - 1); // 3 power - 2 toughness = 1 damage
        
        // Attacker2 (commander) should be destroyed by deathtouch
        let attacker2_status = result.creature_status.get(&attacker2).unwrap();
        assert!(attacker2_status.destroyed);
        
        // Blocker1 should be destroyed
        let blocker1_status = result.creature_status.get(&blocker1).unwrap();
        assert!(blocker1_status.destroyed);
        
        // Blocker2 should take damage but survive
        let blocker2_status = result.creature_status.get(&blocker2).unwrap();
        assert_eq!(blocker2_status.damage, 4);
        assert!(!blocker2_status.destroyed);
        
        // Commander damage should be tracked
        assert_eq!(result.commander_damage[&opponent][&attacker2], 4);
    }
    
    #[test]
    fn test_complex_combat_with_triggers() {
        // Test implementation omitted for brevity
        // This would test combat with abilities that trigger on attack, block, or damage
    }
    
    #[test]
    fn test_multiple_turns_commander_damage() {
        // Test that tracks commander damage across multiple turns
        // Implementation omitted for brevity
    }
    
    // Additional integration tests...
}
```

## System Tests

System tests verify the combat system's interaction with other systems:

```rust
#[cfg(test)]
mod system_tests {
    use super::*;
    
    #[test]
    fn test_combat_with_stack_interaction() {
        let mut app = App::new();
        
        // Add complete game systems
        app.add_plugins(MinimalPlugins)
           .add_plugins(GameLogicPlugins);
        
        // Set up test entities and start a game
        // Implementation omitted for brevity
        
        // Create a combat scenario with abilities that go on the stack
        // Implementation omitted for brevity
        
        // Run the game until combat is complete
        for _ in 0..20 { // Limit iterations to prevent infinite loops
            app.update();
            
            // Check if combat is complete
            let combat_system = app.world.resource::<CombatSystem>();
            if combat_system.active_combat_step == Some(CombatStep::End) {
                break;
            }
        }
        
        // Verify stack interactions worked correctly
        // Implementation omitted for brevity
    }
    
    #[test]
    fn test_combat_with_state_changes() {
        // Test that combat properly interacts with game state changes
        // Implementation omitted for brevity
    }
    
    // Additional system tests...
}
```

## End-to-End Tests

End-to-end tests verify the combat system in the context of a complete game:

```rust
#[cfg(test)]
mod e2e_tests {
    use super::*;
    
    #[test]
    fn test_complete_game_with_combat() {
        let mut app = App::new();
        
        // Add complete game plugins
        app.add_plugins(DefaultPlugins)
           .add_plugins(GamePlugins);
        
        // Set up a complete game with multiple players
        setup_complete_game(&mut app, 4); // 4 players
        
        // Play through several turns
        for _ in 0..10 { // 10 turns
            play_turn(&mut app);
        }
        
        // Verify game state
        let game_state = app.world.resource::<GameState>();
        
        // Check remaining players
        let player_query = app.world.query_filtered::<Entity, With<Player>>();
        let remaining_players = player_query.iter(&app.world).count();
        
        // In most cases, we should still have multiple players
        assert!(remaining_players > 1, "Game ended too quickly");
        
        // Verify commander damage has been dealt
        let player_query = app.world.query::<&Player>();
        let has_commander_damage = player_query
            .iter(&app.world)
            .any(|player| !player.commander_damage.is_empty());
        
        assert!(has_commander_damage, "No commander damage was dealt in the game");
    }
    
    #[test]
    fn test_commander_damage_victory() {
        // Test a game where a player wins via commander damage
        // Implementation omitted for brevity
    }
    
    // Helper function to play a single turn
    fn play_turn(app: &mut App) {
        // Get current phase
        let turn_manager = app.world.resource::<TurnManager>();
        let start_phase = turn_manager.current_phase;
        
        // Run updates until we complete a turn
        for _ in 0..100 { // Limit iterations to prevent infinite loops
            app.update();
            
            let turn_manager = app.world.resource::<TurnManager>();
            
            // If we've come back to the same phase, we've completed a turn
            if turn_manager.current_phase == start_phase 
               && turn_manager.active_player_index != turn_manager.active_player_index {
                break;
            }
        }
    }
    
    // Helper function to set up a complete game
    fn setup_complete_game(app: &mut App, num_players: usize) {
        // Implementation omitted for brevity
    }
    
    // Additional end-to-end tests...
}
```

## Property-Based Testing

We also employ property-based testing to verify invariants across randomized scenarios:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn commander_damage_consistency(
            // Generate random commander power between 1 and 10
            commander_power in 1u32..10,
            // Generate random number of combat rounds between 1 and 5
            combat_rounds in 1usize..5
        ) {
            // Setup combat scenario with commander of specified power
            let mut builder = CombatScenarioBuilder::new();
            let opponent = builder.add_player();
            let commander = builder.add_attacker(commander_power, commander_power, builder.active_player, true);
            
            // Track expected damage
            let mut expected_damage = 0;
            
            // Simulate multiple rounds of combat
            for _ in 0..combat_rounds {
                builder.advance_to(CombatStep::DeclareAttackers);
                builder.declare_attacks(vec![(commander, opponent)]);
                
                // No blocks for simplicity
                builder.advance_to(CombatStep::DeclareBlockers);
                
                // Execute combat damage
                builder.advance_to(CombatStep::CombatDamage);
                builder.execute();
                
                // Update expected damage
                expected_damage += commander_power;
            }
            
            // Check final state
            let result = builder.collect_combat_results();
            
            // Verify commander damage matches expected total
            prop_assert_eq!(
                result.commander_damage[&opponent][&commander],
                expected_damage
            );
            
            // Verify life total reflects damage
            prop_assert_eq!(
                result.player_life[&opponent],
                40 - (expected_damage as i32)
            );
        }
        
        #[test]
        fn multiple_attackers_damage_distribution(
            // Generate 1-5 attackers with power 1-5 each
            attackers in prop::collection::vec((1u32..5, 1u32..5), 1..5)
        ) {
            let mut builder = CombatScenarioBuilder::new();
            let opponent = builder.add_player();
            
            // Create attackers and track total power
            let mut attacker_entities = Vec::new();
            let mut total_power = 0;
            
            for (power, toughness) in attackers {
                let attacker = builder.add_attacker(
                    power, toughness, builder.active_player, false);
                attacker_entities.push(attacker);
                total_power += power;
            }
            
            // Declare all attackers attacking opponent
            builder.advance_to(CombatStep::DeclareAttackers);
            builder.declare_attacks(
                attacker_entities.iter()
                    .map(|&a| (a, opponent))
                    .collect()
            );
            
            // No blocks
            builder.advance_to(CombatStep::DeclareBlockers);
            
            // Execute combat damage
            builder.advance_to(CombatStep::CombatDamage);
            let result = builder.execute();
            
            // Verify opponent's life total correctly reflects all damage
            prop_assert_eq!(
                result.player_life[&opponent],
                40 - (total_power as i32)
            );
        }
    }
}
```

## Test Coverage Goals

Our testing strategy aims for the following coverage metrics:

1. **Line Coverage**: At least 90% of all combat-related code
2. **Branch Coverage**: At least 85% of all conditional branches
3. **Path Coverage**: At least 75% of all execution paths
4. **Edge Case Coverage**: 100% of identified edge cases have dedicated tests

## Continuous Integration

All combat tests are integrated into the CI pipeline with the following workflow:

1. Unit tests run on every commit
2. Integration tests run on every PR
3. System and end-to-end tests run nightly
4. Coverage reports generated after each test run

## Debugging Tools

To assist with debugging, we provide specialized tools:

```rust
/// Utility for debugging combat scenarios
pub fn debug_combat_state(combat_system: &CombatSystem, world: &World) {
    println!("=== COMBAT DEBUG STATE ===");
    
    // Print current combat step
    println!("Combat Step: {:?}", combat_system.active_combat_step);
    
    // Print attackers
    println!("\nAttackers:");
    for (entity, attack_data) in &combat_system.attackers {
        if let Ok((_, creature, _)) = world.query::<(Entity, &Creature, Option<&Commander>)>().get(world, *entity) {
            println!("  {:?} ({}:{}) attacking {:?}{}",
                entity,
                creature.power,
                creature.toughness,
                attack_data.defender,
                if attack_data.is_commander { " [COMMANDER]" } else { "" }
            );
        }
    }
    
    // Print blockers
    println!("\nBlockers:");
    for (entity, block_data) in &combat_system.blockers {
        if let Ok((_, creature)) = world.query::<(Entity, &Creature)>().get(world, *entity) {
            println!("  {:?} ({}:{}) blocking: {:?}",
                entity,
                creature.power,
                creature.toughness,
                block_data.blocked_attackers
            );
        }
    }
    
    // Print combat history
    println!("\nCombat History:");
    for (i, event) in combat_system.combat_history.iter().enumerate() {
        println!("  [{}] {:?}", i, event);
    }
    
    println!("=========================");
}
```

## Verification Strategy Evolution

Our verification approach is not static; it evolves over time:

1. New edge cases identified during testing or gameplay are added to the test suite
2. Performance bottlenecks identified through testing are addressed
3. Test frameworks are updated as the combat system evolves
4. Regression tests are added for any bugs found in production

By following this comprehensive verification strategy, we ensure the Commander combat system correctly implements all rules, handles edge cases properly, and provides a robust foundation for the game engine. 