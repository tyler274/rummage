use crate::camera::components::AppLayer;
use crate::game_engine::save::{LoadGameEvent, SaveGameEvent};
use crate::menu::{
    components::*,
    input_blocker::InputBlocker,
    state::{GameMenuState, StateTransitionContext},
    styles::*,
};
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, Val};

/// Sets up the pause menu interface
pub fn setup_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_menu_items: Query<Entity, With<MenuItem>>,
) {
    // Check for existing menu items first and clean them up if necessary
    let existing_count = existing_menu_items.iter().count();
    if existing_count > 0 {
        info!(
            "Found {} existing menu items, cleaning up before creating pause menu",
            existing_count
        );
        for entity in existing_menu_items.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // First, create a full-screen transparent input blocker
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        AppLayer::Menu.layer(),
        InputBlocker,
        MenuItem,
        Name::new("Pause Menu Input Blocker"),
    ));

    // Then spawn the pause menu UI
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            MenuItem,
            AppLayer::Menu.layer(),
            GlobalZIndex(-5),
        ))
        .with_children(|parent| {
            // Pause menu container
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(450.0), // Increased height to accommodate logo
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start, // Changed to start for better positioning
                        align_items: AlignItems::Center,
                        padding: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Logo container is now the first child above the PAUSED text
                    // The star component itself will be created in the setup_pause_star function
                    parent.spawn((
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(150.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        Name::new("Logo Position"),
                        MenuItem,
                        AppLayer::Menu.layer(),
                    ));

                    // Title (now appears after the logo)
                    parent.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        AppLayer::Menu.layer(),
                    ));

                    // Add spacing after the title
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(20.0),
                            ..default()
                        },
                        Name::new("Title Spacing"),
                        AppLayer::Menu.layer(),
                    ));

                    // Menu buttons
                    spawn_menu_button(parent, "Resume", MenuButtonAction::Resume, &asset_server);
                    spawn_menu_button(
                        parent,
                        "Save Game",
                        MenuButtonAction::SaveGame,
                        &asset_server,
                    );
                    spawn_menu_button(
                        parent,
                        "Load Game",
                        MenuButtonAction::LoadGame,
                        &asset_server,
                    );
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
                    spawn_menu_button(parent, "Quit", MenuButtonAction::Quit, &asset_server);
                });
        });
}

/// Creates a button for the pause menu
fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuButtonAction,
    _asset_server: &AssetServer,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(180.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            action,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::from_section(text, text_style()),
                TextLayout::new_with_justify(JustifyText::Center),
                AppLayer::Menu.layer(),
            ));
        });
}

/// Handles button interactions in the pause menu
pub fn pause_menu_action(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    mut exit: EventWriter<bevy::app::AppExit>,
    _save_events: EventWriter<SaveGameEvent>,
    _load_events: EventWriter<LoadGameEvent>,
    mut save_load_state: ResMut<NextState<crate::menu::save_load::SaveLoadUiState>>,
    mut save_load_context: ResMut<crate::menu::save_load::SaveLoadUiContext>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                match action {
                    MenuButtonAction::Resume => {
                        // Set the context flag to indicate we're coming from the pause menu
                        context.from_pause_menu = true;
                        next_state.set(GameMenuState::InGame);
                    }
                    MenuButtonAction::SaveGame => {
                        info!("Opening save game dialog");
                        // Set the context flag to indicate we're coming from the pause menu
                        save_load_context.from_pause_menu = true;
                        save_load_state.set(crate::menu::save_load::SaveLoadUiState::SaveGame);
                    }
                    MenuButtonAction::LoadGame => {
                        info!("Opening load game dialog");
                        // Set the context flag to indicate we're coming from the pause menu
                        save_load_context.from_pause_menu = true;
                        save_load_state.set(crate::menu::save_load::SaveLoadUiState::LoadGame);
                    }
                    MenuButtonAction::Restart => {
                        // Reset the context flag since we want a full restart
                        context.from_pause_menu = false;
                        next_state.set(GameMenuState::Loading);
                    }
                    MenuButtonAction::MainMenu => {
                        // Reset the context flag since we're going to main menu
                        info!("Resetting from_pause_menu flag because we're going to main menu");
                        context.from_pause_menu = false;
                        // Also clear the settings_origin to ensure we don't have stale data
                        info!("Clearing settings_origin to avoid stale data");
                        context.settings_origin = None;
                        next_state.set(GameMenuState::MainMenu);
                    }
                    MenuButtonAction::Settings => {
                        // Set the context flag to indicate we're coming from the pause menu
                        info!("Setting context flag: from_pause_menu = true");
                        context.from_pause_menu = true;
                        next_state.set(GameMenuState::Settings);
                    }
                    MenuButtonAction::Quit => {
                        exit.send(bevy::app::AppExit::default());
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

/// Handles keyboard input for pausing/unpausing the game
pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    current_state: Res<State<GameMenuState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        // Only process escape key if not transitioning to settings
        if !context.from_pause_menu || *current_state.get() != GameMenuState::Settings {
            match current_state.get() {
                GameMenuState::InGame => {
                    info!("Escape key pressed: Pausing game");
                    next_state.set(GameMenuState::PausedGame);
                }
                GameMenuState::PausedGame => {
                    // Set the context flag to indicate we're coming from the pause menu
                    info!("Escape key pressed: Resuming game from pause menu");
                    context.from_pause_menu = true;
                    next_state.set(GameMenuState::InGame);
                }
                _ => {}
            }
        } else {
            info!("Ignoring escape key press during settings transition");
        }
    }
}
