#[cfg(test)]
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowResolution};
use rummage::menu::*;

fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy::input::InputPlugin,
        bevy::ui::UiPlugin::default(),
        bevy::text::TextPlugin::default(),
    ))
    .init_state::<GameState>();

    // Manually spawn a window entity for testing
    app.world_mut().spawn((
        Window {
            resolution: WindowResolution::new(800.0, 600.0),
            ..default()
        },
        PrimaryWindow,
    ));

    // Add menu plugin to set up all menu systems
    app.add_plugins(MenuPlugin);

    // Run startup systems
    app.update();

    app
}

// Mock test systems for button interactions
#[cfg(test)]
fn mock_button_system(mut query: Query<(&Interaction, &mut BackgroundColor), With<Button>>) {
    for (interaction, mut color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(HOVERED_BUTTON);
            }
            Interaction::None => {
                *color = BackgroundColor(NORMAL_BUTTON);
            }
        }
    }
}

#[test]
fn test_initial_state() {
    let mut app = setup_test_app();
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state, GameState::MainMenu);
}

#[test]
fn test_main_menu_setup() {
    let mut app = setup_test_app();
    app.update();

    // Check menu UI exists
    let menu_count = {
        let world = app.world_mut();
        world
            .query_filtered::<Entity, (With<Button>, With<MenuItem>)>()
            .iter(&world)
            .count()
    };
    assert!(menu_count > 0);
}

#[test]
fn test_new_game_transition() {
    let mut app = setup_test_app();
    app.update();

    // Find the New Game button
    let new_game_button = {
        let world = app.world_mut();
        let mut query =
            world.query_filtered::<(Entity, &MenuButtonAction), (With<Button>, With<MenuItem>)>();
        query
            .iter(world)
            .find(|(_, action)| matches!(**action, MenuButtonAction::NewGame))
            .map(|(entity, _)| entity)
            .expect("No New Game button found")
    };

    // Simulate clicking "New Game" button
    app.world_mut()
        .entity_mut(new_game_button)
        .insert(Interaction::Pressed);
    app.update();

    // Should transition to Loading
    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Loading);

    // Update again to let the loading system run
    app.update();

    // Should transition to InGame
    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::InGame);
}

#[test]
fn test_pause_menu() {
    let mut app = setup_test_app();

    // Start game
    {
        let world = app.world_mut();
        world
            .resource_mut::<NextState<GameState>>()
            .set(GameState::InGame);
    }
    app.update();

    // Press escape to pause
    {
        let world = app.world_mut();
        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
    }
    app.update();

    // Check state changed to paused
    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state, GameState::PausedGame);

    // Check pause menu UI exists
    let menu_count = {
        let world = app.world_mut();
        world
            .query_filtered::<Entity, (With<Button>, With<MenuItem>)>()
            .iter(&world)
            .count()
    };
    assert!(menu_count > 0);

    // Press escape again to unpause
    {
        let world = app.world_mut();
        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
    }
    app.update();

    // Check returned to game state
    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state, GameState::InGame);
}

#[test]
fn test_cleanup() {
    let mut app = setup_test_app();

    // Start game then pause
    {
        let world = app.world_mut();
        world
            .resource_mut::<NextState<GameState>>()
            .set(GameState::InGame);
    }
    app.update();
    {
        let world = app.world_mut();
        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
    }
    app.update();

    // Check menu exists
    let menu_count = {
        let world = app.world_mut();
        world
            .query_filtered::<Entity, With<MenuItem>>()
            .iter(&world)
            .count()
    };
    assert!(menu_count > 0);

    // Unpause
    {
        let world = app.world_mut();
        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
    }
    app.update();

    // Check menu cleaned up
    let menu_count = {
        let world = app.world_mut();
        world
            .query_filtered::<Entity, With<MenuItem>>()
            .iter(&world)
            .count()
    };
    assert_eq!(menu_count, 0);
}

#[test]
fn test_button_interactions() {
    let mut app = setup_test_app();

    // Add our mock button system
    app.add_systems(Update, mock_button_system);
    app.update();

    // Store the button entity and then modify its interaction state
    let button_entity = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, (With<Button>, With<MenuItem>)>();
        query.iter(world).next().expect("No menu buttons found")
    };

    // Set to hover state
    app.world_mut()
        .entity_mut(button_entity)
        .insert(Interaction::Hovered);
    app.update();

    // Check hover color
    let has_hovered_button = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<&BackgroundColor, With<Button>>();
        query
            .iter(world)
            .any(|color| *color == BackgroundColor(HOVERED_BUTTON))
    };
    assert!(has_hovered_button, "No button with hover color found");

    // Set to pressed state
    app.world_mut()
        .entity_mut(button_entity)
        .insert(Interaction::Pressed);
    app.update();

    // Check pressed color
    let has_pressed_button = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<&BackgroundColor, With<Button>>();
        query
            .iter(world)
            .any(|color| *color == BackgroundColor(PRESSED_BUTTON))
    };
    assert!(has_pressed_button, "No button with pressed color found");
}
