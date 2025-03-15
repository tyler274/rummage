use crate::game_engine::phase::{ActivePlayer, CurrentPhase, MAIN1};
use crate::game_engine::zones::{Zone, ZoneMarker};
use crate::mana::ManaPool;
use crate::player::components::Player;
use bevy::prelude::*;

/// Test fixtures for the game engine

/// Creates test players
#[allow(dead_code)]
pub fn create_test_players(app: &mut App, count: usize) -> Vec<Entity> {
    let mut players = Vec::with_capacity(count);

    for _i in 0..count {
        // Create player with indexed name
        let player_name = format!("Test Player {}", _i + 1);
        // Position players in a circle around the origin
        let angle = (_i as f32 / count as f32) * std::f32::consts::TAU;
        let position = Vec2::new(angle.cos() * 15.0, angle.sin() * 15.0);

        let player = create_test_player(app, &player_name, position);
        players.push(player);
    }

    players
}

/// Creates a test player entity
#[allow(dead_code)]
pub fn create_test_player(app: &mut App, _name: &str, _position: Vec2) -> Entity {
    let entity = app
        .world_mut()
        .spawn((
            Player {
                name: _name.to_string(),
                life: 40,
                mana_pool: ManaPool::default(),
                player_index: 0,
            },
            Transform::from_translation(Vec3::new(_position.x, _position.y, 0.0)),
        ))
        .id();

    entity
}

/// Creates multiple test cards in the app world
#[allow(dead_code)]
pub fn create_test_cards(app: &mut App, count: usize) -> Vec<Entity> {
    let mut entities = Vec::new();

    for _i in 0..count {
        let entity = app.world_mut().spawn(()).id();
        entities.push(entity);
    }

    entities
}

/// Setup function for four player game
#[allow(dead_code)]
pub fn setup_four_player_game(app: &mut App) -> [Entity; 4] {
    let players = create_test_players(app, 4);
    [players[0], players[1], players[2], players[3]]
}

/// Creates a player with a test deck
#[allow(dead_code)]
pub fn setup_test_player_with_card(app: &mut App) -> (Entity, Entity) {
    let player = create_test_players(app, 1)[0];
    let card = create_test_cards(app, 1)[0];

    (player, card)
}

/// Sets up a full game with players, cards, etc.
#[allow(dead_code)]
pub fn setup_full_game(_app: &mut App) {
    // Set up 4 players
    let players = setup_four_player_game(_app);

    // Set up initial game state
    _app.world_mut().insert_resource(CurrentPhase(MAIN1));

    // Mark first player as active
    _app.world_mut().entity_mut(players[0]).insert(ActivePlayer);

    // Set up starting life totals and decks
    for player in players.iter() {
        // Each player gets a starting deck
        for _ in 0..40 {
            let card = create_test_cards(_app, 1)[0];
            // Add card to player's library
            _app.world_mut().entity_mut(card).insert(ZoneMarker {
                zone_type: Zone::Library,
                owner: Some(*player),
            });
        }
    }
}

/// Saves the current game state for comparison
#[allow(dead_code)]
pub fn save_game_state(_app: &App, _name: &str) {
    use std::fs::File;
    use std::io::Write;

    // Create a serializable representation of the game state
    let mut state_data = Vec::new();
    let world = _app.world();

    // Serialize phase
    let phase = world.resource::<CurrentPhase>().0;
    state_data.push(format!("Phase: {:?}", phase));

    // Serialize players - manual iteration to avoid borrowing issues
    for entity in world.iter_entities() {
        if let Some(player) = world.get::<Player>(entity.id()) {
            state_data.push(format!(
                "Player: {} (Entity: {:?})",
                player.name,
                entity.id()
            ));
        }
    }

    // Save to file
    let path = format!("test_artifacts/game_states/{}.state", _name);
    let mut file = File::create(path).expect("Failed to create state file");
    for line in state_data {
        writeln!(file, "{}", line).expect("Failed to write state data");
    }
}

/// Compares the current game state with a saved one
#[allow(dead_code)]
pub fn compare_with_saved_state(_app: &App, _name: &str) -> bool {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    // Generate current state
    let mut current_state = Vec::new();
    let world = _app.world();

    // Serialize current phase
    let phase = world.resource::<CurrentPhase>().0;
    current_state.push(format!("Phase: {:?}", phase));

    // Serialize current players - manual iteration to avoid borrowing issues
    for entity in world.iter_entities() {
        if let Some(player) = world.get::<Player>(entity.id()) {
            current_state.push(format!(
                "Player: {} (Entity: {:?})",
                player.name,
                entity.id()
            ));
        }
    }

    // Load saved state
    let path = format!("test_artifacts/game_states/{}.state", _name);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return false, // State doesn't exist
    };

    let reader = BufReader::new(file);
    let saved_state: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    // Compare states (simplified comparison)
    if current_state.len() != saved_state.len() {
        return false;
    }

    for (i, line) in current_state.iter().enumerate() {
        if !saved_state[i].contains(line) {
            return false;
        }
    }

    true
}

/// Sets up a specific test scenario
#[allow(dead_code)]
pub fn setup_test_scenario(_app: &mut App, _scenario: &str) {
    match _scenario {
        "empty_board" => {
            // Set up just the players with no cards in play
            setup_four_player_game(_app);
        }
        "basic_creatures" => {
            // Set up players with basic creatures on the battlefield
            let players = setup_four_player_game(_app);

            // Each player gets 2 basic creatures
            for player in players.iter() {
                for _ in 0..2 {
                    let card = create_test_cards(_app, 1)[0];
                    // Add card to battlefield
                    _app.world_mut().entity_mut(card).insert(ZoneMarker {
                        zone_type: Zone::Battlefield,
                        owner: Some(*player),
                    });
                }
            }
        }
        "complex_board" => {
            // Set up a complex board state for testing
            setup_full_game(_app);
            // Add additional complexity as needed
        }
        _ => {
            // Default to empty board
            setup_four_player_game(_app);
        }
    }
}

/// Sets up a basic test environment
#[allow(dead_code)]
pub fn setup_test_environment(app: &mut App) {
    // Register all required components and resources
    app.init_resource::<Time>();

    // Initialize other required systems and resources
    // ...

    // Run startup systems
    app.update();
}
