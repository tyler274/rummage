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
use bevy::app::AppExit;
use bevy::prelude::*;

// Note: As of Bevy 0.15.x, we use the modern component system instead of bundles.
// Required components are automatically added when using the primary component.

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
            .add_systems(OnExit(GameState::Loading), finish_loading);
    }
}

// Button colors for different interaction states
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// Sets up the main menu interface with buttons and layout
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera with all required components
    commands.spawn((
        Camera2d,
        Camera::default(),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        MenuItem,
    ));

    // Main menu container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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
                    MenuItem,
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
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(65.0),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            action,
            MenuItem,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                MenuItem,
            ));
        });
}

/// Safely cleans up menu entities when transitioning to another state
fn cleanup_main_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for entity in menu_items.iter() {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Handles button interactions and triggers appropriate actions
fn menu_action(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match menu_button_action {
                    MenuButtonAction::NewGame => {
                        info!("Starting new game...");
                        next_state.set(GameState::Loading);
                    }
                    MenuButtonAction::LoadGame => {
                        info!("Loading saved game...");
                        next_state.set(GameState::Loading);
                    }
                    MenuButtonAction::Multiplayer => {
                        info!("Multiplayer not implemented yet");
                    }
                    MenuButtonAction::Settings => {
                        info!("Settings not implemented yet");
                    }
                    MenuButtonAction::Quit => {
                        info!("Exiting game...");
                        let _ = app_exit_events.send(AppExit::default());
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
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
