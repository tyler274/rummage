use crate::card::Card;
/// Menu system for the game, handling main menu state and interactions.
///
/// This module provides:
/// - Menu state management (MainMenu, Loading, InGame)
/// - Button components and interactions
/// - Menu layout and styling
/// - State transitions between menu and game
///
/// # State Flow
/// ```plaintext
/// MainMenu -> Loading -> InGame
///     ^          |
///     |          |
///     +----------+
/// ```
///
/// # Menu Layout
/// The menu is structured as a centered container with:
/// - Vertical stack of buttons
/// - Consistent button sizing and spacing
/// - Hover and click interactions
/// - Smooth state transitions
use bevy::prelude::*;

/// Game states for managing transitions between different parts of the game.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
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

/// Actions associated with menu buttons
#[derive(Component)]
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
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(Update, menu_action.run_if(in_state(GameState::MainMenu)))
            // Loading state systems
            .add_systems(OnEnter(GameState::Loading), start_game_loading)
            .add_systems(OnExit(GameState::Loading), finish_loading)
            // Pause menu systems
            .add_systems(OnEnter(GameState::PausedGame), setup_pause_menu)
            .add_systems(OnExit(GameState::PausedGame), cleanup_pause_menu)
            .add_systems(
                Update,
                pause_menu_action.run_if(in_state(GameState::PausedGame)),
            )
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(GameState::InGame)),
            );
    }
}

// Button colors for different interaction states
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// Sets up the main menu interface with buttons and layout
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d::default());

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
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                match action {
                    MenuButtonAction::NewGame => {
                        next_state.set(GameState::Loading);
                    }
                    MenuButtonAction::LoadGame => {
                        next_state.set(GameState::Loading);
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
fn start_game_loading(mut next_state: ResMut<NextState<GameState>>) {
    info!("Starting game loading sequence...");
    // For now, immediately transition to InGame
    // In the future, we can add actual loading logic here
    next_state.set(GameState::InGame);
}

/// Performs cleanup and finalization after loading completes
fn finish_loading() {
    info!("Game loading complete!");
}

/// Sets up the pause menu interface with buttons and layout
fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Container for pause menu
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
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut hand_query: Query<Entity, With<Card>>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                match action {
                    MenuButtonAction::Resume => {
                        next_state.set(GameState::InGame);
                    }
                    MenuButtonAction::Restart => {
                        // Despawn current hand
                        for entity in hand_query.iter_mut() {
                            commands.entity(entity).despawn_recursive();
                        }
                        next_state.set(GameState::Loading);
                    }
                    MenuButtonAction::Settings => {
                        info!("Settings menu not implemented yet");
                    }
                    MenuButtonAction::MainMenu => {
                        // Despawn current hand before returning to main menu
                        for entity in hand_query.iter_mut() {
                            commands.entity(entity).despawn_recursive();
                        }
                        next_state.set(GameState::MainMenu);
                    }
                    MenuButtonAction::Quit => {
                        std::process::exit(0);
                    }
                    _ => {}
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

/// Handles escape key input to toggle pause menu
fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::PausedGame);
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
