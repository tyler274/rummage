# Special Card Implementations

This document covers the implementation details of cards with unique and complex mechanics that require special handling in our engine.

## Shahrazad

![Shahrazad](https://gatherer.wizards.com/Handlers/Image.ashx?multiverseid=980&type=card)

Shahrazad is a unique sorcery that creates a subgame:

```
Shahrazad {W}{W}
Sorcery
Players play a Magic subgame, using their libraries as their decks. Each player who doesn't win the subgame loses half their life, rounded up.
```

### Implementation Details

Shahrazad requires deep integration with the game engine to manage subgames:

```rust
#[derive(Component)]
pub struct Shahrazad;

impl CardEffect for Shahrazad {
    fn resolve(&self, world: &mut World, effect_ctx: &EffectContext) -> EffectResult {
        // Get access to necessary systems
        let mut subgame_system = world.resource_mut::<SubgameSystem>();
        
        // Start a subgame using the libraries as decks
        subgame_system.start_new_subgame(effect_ctx);
        
        // The subgame will run completely before continuing
        // Return pending to indicate this effect pauses resolution
        EffectResult::Pending
    }
}

// Called when the subgame completes
pub fn apply_shahrazad_results(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Life), With<Player>>,
    subgame_result: Res<SubgameResult>,
) {
    // Apply life loss to players who didn't win
    for (player_entity, mut life) in player_query.iter_mut() {
        if !subgame_result.winners.contains(&player_entity) {
            // Lose half life, rounded up
            life.value = life.value - (life.value + 1) / 2;
        }
    }
}
```

### Testing Approach

Testing Shahrazad is complex due to its nesting capabilities:

1. **Unit Testing**: We verify the subgame initialization and cleanup logic
2. **Integration Testing**: We test interactions between the subgame and main game
3. **End-to-End Testing**: We validate full gameplay flows with subgames

Example test:
```rust
#[test]
fn test_shahrazad_life_loss() {
    let mut app = App::new();
    // Setup test environment...
    
    // Initial life totals
    let player1_life = 20;
    let player2_life = 20;
    
    // Cast Shahrazad
    // ... test code to cast the spell ...
    
    // Simulate subgame where player 1 wins
    app.world.resource_mut::<SubgameResult>().winners = vec![player1_entity];
    app.update(); // Triggers apply_shahrazad_results
    
    // Verify player 2 lost half their life
    let new_player2_life = app.world.get::<Life>(player2_entity).unwrap().value;
    assert_eq!(new_player2_life, 10); // 20 -> 10
}
```

## Karn Liberated

![Karn Liberated](https://gatherer.wizards.com/Handlers/Image.ashx?multiverseid=397828&type=card)

Karn Liberated is a planeswalker with an ultimate ability that restarts the game:

```
Karn Liberated {7}
Legendary Planeswalker — Karn
Starting loyalty: 6

+4: Target player exiles a card from their hand.
−3: Exile target permanent.
−14: Restart the game, leaving in exile all non-Aura permanent cards exiled with Karn Liberated. Then put those cards onto the battlefield under your control.
```

### Implementation Details

Karn's implementation focuses on tracking exiled cards and restarting the game:

```rust
#[derive(Component)]
pub struct KarnLiberated;

#[derive(Component)]
pub struct ExiledWithKarn;

impl CardEffect for KarnUltimate {
    fn resolve(&self, world: &mut World, effect_ctx: &EffectContext) -> EffectResult {
        // Verify loyalty cost can be paid
        if !self.can_pay_loyalty_cost(world, effect_ctx, -14) {
            return EffectResult::Failed;
        }
        
        // Pay loyalty cost
        self.pay_loyalty_cost(world, effect_ctx, -14);
        
        // Trigger game restart
        let mut restart_events = world.resource_mut::<Events<GameRestartEvent>>();
        restart_events.send(GameRestartEvent {
            source: effect_ctx.source_entity,
            restart_type: RestartType::KarnUltimate,
        });
        
        EffectResult::Success
    }
}

// System that runs when a game restart event occurs
pub fn handle_karn_restart(
    mut commands: Commands,
    mut restart_events: EventReader<GameRestartEvent>,
    exiled_cards: Query<(Entity, &CardData, &Owner), With<ExiledWithKarn>>,
    karn_controller: Query<&Controller, With<KarnLiberated>>,
) {
    for event in restart_events.read() {
        if event.restart_type != RestartType::KarnUltimate {
            continue;
        }
        
        // Get Karn's controller
        let karn_owner = karn_controller.get_single().unwrap();
        
        // Track cards that will be put onto the battlefield
        let cards_to_return = exiled_cards
            .iter()
            .filter(|(_, card_data, _)| !card_data.is_aura())
            .map(|(entity, _, _)| entity)
            .collect::<Vec<_>>();
        
        // Initialize new game state...
        
        // Put exiled cards onto the battlefield under Karn owner's control
        for card_entity in cards_to_return {
            // Set controller to Karn's controller
            commands.entity(card_entity)
                .insert(Controller(karn_owner.0))
                .remove::<ExiledWithKarn>();
                
            // Move to battlefield
            // ... zone transition code ...
        }
    }
}
```

### Testing Approach

Testing Karn Liberated's restart ability requires comprehensive state verification:

1. **Unit Testing**: We verify card tracking and the restart triggering
2. **Integration Testing**: We ensure exiled cards correctly transfer to the new game
3. **End-to-End Testing**: We validate the complete game restart flow

Example test:
```rust
#[test]
fn test_karn_ultimate_restart() {
    let mut app = App::new();
    // Setup test environment...
    
    // Create test cards and exile them with Karn
    let test_card1 = spawn_test_card(&mut app.world, "Test Card 1");
    let test_card2 = spawn_test_card(&mut app.world, "Test Card 2");
    
    // Add ExiledWithKarn to the cards
    app.world.entity_mut(test_card1).insert(ExiledWithKarn);
    app.world.entity_mut(test_card2).insert(ExiledWithKarn);
    
    // Activate Karn's ultimate
    // ... test code to activate the ability ...
    
    // Verify game restarted and cards are on the battlefield
    let card1_zone = app.world.get::<Zone>(test_card1).unwrap();
    assert_eq!(*card1_zone, Zone::Battlefield);
    
    let card1_controller = app.world.get::<Controller>(test_card1).unwrap();
    assert_eq!(card1_controller.0, karn_controller_id);
}
```

## Other Special Cards

### Mindslaver

Mindslaver allows a player to control another player's turn. Implementation involves intercepting player choices and redirecting control.

### Panglacial Wurm

Panglacial Wurm can be cast while searching a library. Implementation requires special handling of the timing and visibility rules during search effects.

### Time Stop

Time Stop ends the turn immediately. Implementation involves carefully cleaning up the stack and skipping directly to the cleanup step.

## Integration with Game Engine

Special cards leverage several core engine systems:

- [Snapshot System](../../core_systems/snapshot/index.md) for state management
- [Event System](../../game_engine/events/index.md) for communicating complex state changes
- [State Management](../../game_engine/state/index.md) for tracking game/turn state

For more on subgames and game restarting mechanics, see:
- [Subgames and Game Restarting Rules](../../mtg_rules/subgames.md)
- [Technical Implementation](../../game_engine/state/subgames.md) 