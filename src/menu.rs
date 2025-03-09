use bevy::app::AppExit;
use bevy::prelude::*;

// Note: As of Bevy 0.15.x, we use the modern component system instead of bundles.
// Required components are automatically added when using the primary component.

// Game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Loading,
}

// UI component for menu items
#[derive(Component)]
pub struct MenuItem;

// UI component for specific menu actions
#[derive(Component)]
pub enum MenuButtonAction {
    NewGame,
    LoadGame,
    Multiplayer,
    Settings,
    Quit,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(Update, menu_action.run_if(in_state(GameState::MainMenu)));
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

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
            ));
        });
}

fn cleanup_main_menu(mut commands: Commands, menu_items: Query<Entity, With<Node>>) {
    for entity in menu_items.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

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
                        next_state.set(GameState::Loading);
                    }
                    MenuButtonAction::LoadGame => {
                        next_state.set(GameState::Loading);
                    }
                    MenuButtonAction::Multiplayer => {
                        println!("Multiplayer not implemented yet");
                    }
                    MenuButtonAction::Settings => {
                        println!("Settings not implemented yet");
                    }
                    MenuButtonAction::Quit => {
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
