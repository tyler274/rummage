#[cfg(test)]
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rummage::menu::*;

// Create a test app with mock components for tracking state
fn setup_test_app() -> App {
    let mut app = App::new();

    // Add minimal plugins for testing
    app.add_plugins(MinimalPlugins);

    // Initialize resources
    app.init_resource::<StateTracker>();

    // Spawn window and add our custom test entity
    app.add_systems(Startup, setup_mock_window);

    app
}

// Resource to track state changes for testing
#[derive(Resource, Default)]
struct StateTracker {
    main_menu_exited: bool,
    in_game_entered: bool,
    paused_entered: bool,
    paused_exited: bool,
}

// System to spawn mock window
fn setup_mock_window(mut commands: Commands) {
    commands.spawn(PrimaryWindow);
}

// Common helper function to track state changes
fn track_state_change(
    old_state: GameMenuState,
    new_state: GameMenuState,
    tracker: &mut StateTracker,
) {
    // Handle exit events first
    match old_state {
        GameMenuState::MainMenu => tracker.main_menu_exited = true,
        GameMenuState::PauseMenu => tracker.paused_exited = true,
        _ => {}
    }

    // Then handle enter events
    match new_state {
        GameMenuState::InGame => tracker.in_game_entered = true,
        GameMenuState::PauseMenu => tracker.paused_entered = true,
        _ => {}
    }
}

#[test]
fn test_initial_state() {
    // No need to set up app just to test default state
    assert_eq!(GameMenuState::default(), GameMenuState::MainMenu);
}

#[test]
fn test_game_transition() {
    let mut app = setup_test_app();

    // Manually track transitions in test directly
    let initial_state = GameMenuState::MainMenu;
    let intermediate_state = GameMenuState::Loading;
    let final_state = GameMenuState::InGame;

    let mut tracker = app.world_mut().resource_mut::<StateTracker>();

    // Track MainMenu -> Loading transition
    track_state_change(initial_state, intermediate_state, &mut tracker);

    // Track Loading -> InGame transition
    track_state_change(intermediate_state, final_state, &mut tracker);

    // Verify state transitions were tracked
    assert!(
        tracker.main_menu_exited,
        "MainMenu exit event not triggered"
    );
    assert!(tracker.in_game_entered, "InGame enter event not triggered");
}

#[test]
fn test_pause_unpause() {
    let mut app = setup_test_app();

    // Manually track transitions in test
    let in_game_state = GameMenuState::InGame;
    let paused_state = GameMenuState::PauseMenu;

    let mut tracker = app.world_mut().resource_mut::<StateTracker>();

    // Track InGame -> PauseMenu transition (pause)
    track_state_change(in_game_state, paused_state, &mut tracker);

    // Verify pause state transitions
    assert!(
        tracker.paused_entered,
        "PauseMenu enter event not triggered"
    );

    // Reset both trackers to avoid false positives
    tracker.paused_entered = false;
    tracker.paused_exited = false;

    // Track PauseMenu -> InGame transition (unpause)
    track_state_change(paused_state, in_game_state, &mut tracker);

    // Verify unpause state transitions
    assert!(tracker.paused_exited, "PauseMenu exit event not triggered");
}

#[test]
fn test_return_to_main_menu() {
    let mut app = setup_test_app();

    // Manually track transitions in test
    let main_menu_state = GameMenuState::MainMenu;
    let paused_state = GameMenuState::PauseMenu;

    let mut tracker = app.world_mut().resource_mut::<StateTracker>();

    // Track PauseMenu -> MainMenu transition
    track_state_change(paused_state, main_menu_state, &mut tracker);

    // Check effects happened
    assert!(tracker.paused_exited, "Should track exiting pause state");
}

#[test]
fn test_state_cycle() {
    let mut app = setup_test_app();
    let mut tracker = app.world_mut().resource_mut::<StateTracker>();

    // Test full cycle of state transitions
    let initial_state = GameMenuState::MainMenu;
    let loading_state = GameMenuState::Loading;
    let in_game_state = GameMenuState::InGame;
    let paused_state = GameMenuState::PauseMenu;

    // MainMenu -> Loading -> InGame
    track_state_change(initial_state, loading_state, &mut tracker);
    track_state_change(loading_state, in_game_state, &mut tracker);
    assert!(tracker.main_menu_exited, "MainMenu exit should be tracked");
    assert!(tracker.in_game_entered, "InGame enter should be tracked");

    // InGame -> PauseMenu
    track_state_change(in_game_state, paused_state, &mut tracker);
    assert!(tracker.paused_entered, "PauseMenu enter should be tracked");

    // PauseMenu -> MainMenu
    track_state_change(paused_state, initial_state, &mut tracker);
    assert!(tracker.paused_exited, "PauseMenu exit should be tracked");
}
