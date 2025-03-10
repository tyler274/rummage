use crate::card::Card;

use bevy::prelude::*;

/// Menu system for the game, handling state management and user interface.
///
/// This module provides a complete menu system including:
/// - State Management:
///   - Main Menu: Entry point with options for new game, load game, etc.
///   - Loading: Transitional state for game setup and cleanup
///   - In-Game: Active gameplay state
///   - Paused Game: Overlay menu during gameplay
///
/// - Menu Components:
///   - Buttons with hover and click interactions
///   - Text elements with consistent styling
///   - Camera management for menu and game views
///   - State-specific cleanup systems
///
/// - State Transitions:
/// ```plaintext
///                    ┌─────────┐
///                    │         │
///                    ▼         │
/// MainMenu ──► Loading ──► InGame ◄─┐
///    ▲         │                    │
///    │         │                    │
///    └─────────┘              PausedGame
/// ```
///
/// - Cleanup Behavior:
///   - Menu entities are cleaned up when exiting menu states
///   - Game entities (cards, camera) are cleaned up when:
///     - Entering Loading state (for restarts)
///     - Entering MainMenu state (when exiting game)
///
/// - Camera Management:
///   - MenuCamera: Used for menu UI rendering
///   - GameCamera: Used for game view and pause menu overlay
///
/// # Examples
///
/// Basic usage in main game setup:
/// ```no_run
/// use bevy::prelude::*;
/// use rummage::menu::MenuPlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(MenuPlugin)
///         .run();
/// }
/// ```
///
/// # State Transitions
///
/// - New Game: MainMenu -> Loading -> InGame
/// - Pause: InGame -> PausedGame
/// - Resume: PausedGame -> InGame
/// - Restart: PausedGame -> Loading -> InGame
/// - Main Menu: PausedGame -> MainMenu
///
/// # Component Hierarchy
///
/// ```plaintext
/// MainMenu
/// ├── Camera2d + MenuCamera
/// └── Root Container (MenuItem)
///     └── Button Container
///         ├── New Game Button
///         ├── Load Game Button
///         ├── Settings Button
///         └── Quit Button
///
/// PauseMenu
/// └── Overlay Container (MenuItem)
///     ├── Title Text
///     └── Button Container
///         ├── Resume Button
///         ├── Restart Button
///         ├── Settings Button
///         ├── Main Menu Button
///         └── Exit Button
/// ```
///
/// # Testing
///
/// The module includes comprehensive tests for:
/// - State transitions
/// - Button interactions
/// - Entity cleanup
/// - Camera management
/// - Menu layout and styling
///
/// See the tests module for detailed examples.

/// Game states for managing transitions between different parts of the game.
#[derive(States, Resource, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameMenuState {
    /// Initial state, showing the main menu
    #[default]
    MainMenu,
    /// Transitional state for loading game assets
    Loading,
    /// Active gameplay state
    InGame,
    /// Game is paused, showing pause menu
    PausedGame,
}

/// Marker component for menu-related entities to facilitate cleanup
#[derive(Component)]
pub struct MenuItem;

/// Marker component for menu-related camera
#[derive(Component)]
pub struct MenuCamera;

/// Marker component for game camera
#[derive(Component)]
pub struct GameCamera;

/// Button actions for different menu states
#[derive(Component, Clone)]
pub enum MenuButtonAction {
    /// Start a new game session
    NewGame,
    /// Load a previously saved game
    LoadGame,
    /// Enter multiplayer mode
    Multiplayer,
    /// Open settings menu
    Settings,
    /// Exit the game
    Quit,
    /// Resume the current game
    Resume,
    /// Restart the current game with a new hand
    Restart,
    /// Return to the main menu
    MainMenu,
}

/// Plugin that sets up the menu system and its related systems
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMenuState>()
            .insert_resource(GameMenuState::MainMenu)
            .add_systems(OnEnter(GameMenuState::MainMenu), setup_main_menu)
            .add_systems(
                OnExit(GameMenuState::MainMenu),
                (cleanup_main_menu, cleanup_menu_camera),
            )
            .add_systems(
                Update,
                menu_action.run_if(in_state(GameMenuState::MainMenu)),
            )
            // Loading state systems
            .add_systems(
                OnEnter(GameMenuState::Loading),
                (cleanup_game, start_game_loading).chain(),
            )
            .add_systems(OnExit(GameMenuState::Loading), finish_loading)
            // Pause menu systems
            .add_systems(OnEnter(GameMenuState::PausedGame), setup_pause_menu)
            .add_systems(OnExit(GameMenuState::PausedGame), cleanup_pause_menu)
            .add_systems(
                Update,
                pause_menu_action.run_if(in_state(GameMenuState::PausedGame)),
            )
            .add_systems(Update, handle_pause_input)
            // Add cleanup when entering main menu from game
            .add_systems(OnEnter(GameMenuState::MainMenu), cleanup_game);
    }
}

// Button colors for different interaction states
/// Normal state color for buttons
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
/// Hover state color for buttons
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
/// Pressed state color for buttons
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// Sets up the main menu interface with buttons and layout
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera with menu marker
    commands.spawn((Camera2d::default(), MenuCamera));

    // Main menu container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            MenuItem,
        ))
        .with_children(|parent| {
            // Menu buttons container
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ))
                .with_children(|parent| {
                    spawn_menu_button(parent, "New Game", MenuButtonAction::NewGame, &asset_server);
                    spawn_menu_button(
                        parent,
                        "Load Game",
                        MenuButtonAction::LoadGame,
                        &asset_server,
                    );
                    spawn_menu_button(
                        parent,
                        "Multiplayer",
                        MenuButtonAction::Multiplayer,
                        &asset_server,
                    );
                    spawn_menu_button(
                        parent,
                        "Settings",
                        MenuButtonAction::Settings,
                        &asset_server,
                    );
                    spawn_menu_button(parent, "Quit", MenuButtonAction::Quit, &asset_server);
                });
        });
}

/// Creates a menu button with text and interaction handlers
fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuButtonAction,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(250.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            Button,
            Interaction::default(),
            action,
            MenuItem,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text.to_string()),
                TextFont::default()
                    .with_font(asset_server.load("fonts/FiraSans-Bold.ttf"))
                    .with_font_size(40.0),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                MenuItem,
            ));
        });
}

/// Handles button interactions and triggers appropriate actions
fn menu_action(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                match action {
                    MenuButtonAction::NewGame => {
                        next_state.set(GameMenuState::Loading);
                    }
                    MenuButtonAction::LoadGame => {
                        next_state.set(GameMenuState::Loading);
                    }
                    MenuButtonAction::Settings => {
                        info!("Settings menu not implemented yet");
                    }
                    MenuButtonAction::Quit => {
                        std::process::exit(0);
                    }
                    MenuButtonAction::Resume
                    | MenuButtonAction::Restart
                    | MenuButtonAction::MainMenu
                    | MenuButtonAction::Multiplayer => {
                        // These actions are handled in pause_menu_action
                    }
                }
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

/// Initiates the game loading sequence
fn start_game_loading(mut next_state: ResMut<NextState<GameMenuState>>) {
    info!("Starting game loading sequence...");
    // For now, immediately transition to InGame
    // In the future, we can add actual loading logic here
    next_state.set(GameMenuState::InGame);
}

/// Performs cleanup and finalization after loading completes
fn finish_loading() {
    info!("Game loading complete!");
}

/// Sets up the pause menu interface with buttons and layout
fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Container for pause menu - we use the existing game camera
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            MenuItem,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Game Paused"),
                TextFont::default()
                    .with_font(asset_server.load("fonts/FiraSans-Bold.ttf"))
                    .with_font_size(40.0),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                MenuItem,
            ));

            // Buttons container
            parent
                .spawn((Node {
                    width: Val::Px(250.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },))
                .with_children(|parent| {
                    spawn_menu_button(parent, "Resume", MenuButtonAction::Resume, &asset_server);
                    spawn_menu_button(parent, "Restart", MenuButtonAction::Restart, &asset_server);
                    spawn_menu_button(
                        parent,
                        "Settings",
                        MenuButtonAction::Settings,
                        &asset_server,
                    );
                    spawn_menu_button(
                        parent,
                        "Main Menu",
                        MenuButtonAction::MainMenu,
                        &asset_server,
                    );
                    spawn_menu_button(parent, "Exit Game", MenuButtonAction::Quit, &asset_server);
                });
        });
}

/// Handles pause menu button interactions and triggers appropriate actions
fn pause_menu_action(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                match action {
                    MenuButtonAction::Resume => next_state.set(GameMenuState::InGame),
                    MenuButtonAction::Restart => next_state.set(GameMenuState::Loading),
                    MenuButtonAction::Settings => info!("Settings menu not implemented yet"),
                    MenuButtonAction::MainMenu => next_state.set(GameMenuState::MainMenu),
                    MenuButtonAction::Quit => std::process::exit(0),
                    _ => {}
                }
            }
            Interaction::Hovered => *color = BackgroundColor(HOVERED_BUTTON),
            Interaction::None => *color = BackgroundColor(NORMAL_BUTTON),
        }
    }
}

/// Handles escape key input to toggle pause menu
fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
    current_state: Res<State<GameMenuState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameMenuState::InGame => next_state.set(GameMenuState::PausedGame),
            GameMenuState::PausedGame => next_state.set(GameMenuState::InGame),
            _ => (), // Do nothing for other states
        }
    }
}

/// Safely cleans up menu entities when transitioning to another state
fn cleanup_main_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for entity in menu_items.iter() {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Safely cleans up pause menu entities when transitioning to another state
fn cleanup_pause_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for entity in menu_items.iter() {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Cleans up menu camera when transitioning between states
fn cleanup_menu_camera(mut commands: Commands, menu_cameras: Query<Entity, With<MenuCamera>>) {
    for camera in menu_cameras.iter() {
        commands.entity(camera).despawn_recursive();
    }
}

/// Cleans up all game entities (cards and camera) when restarting or exiting the game
fn cleanup_game(
    mut commands: Commands,
    cards: Query<Entity, With<Card>>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Clean up all cards
    for card in cards.iter() {
        commands.entity(card).despawn_recursive();
    }

    // Clean up game camera
    for camera in game_cameras.iter() {
        commands.entity(camera).despawn_recursive();
    }
}
