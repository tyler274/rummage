use crate::camera::components::AppLayer;
use crate::menu::{
    components::*,
    state::{GameMenuState, StateTransitionContext},
    styles::*,
};
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, Val};

/// Sets up the pause menu interface
pub fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        ))
        .with_children(|parent| {
            // Pause menu container
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
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        AppLayer::Menu.layer(),
                    ));

                    // Menu buttons
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
                    spawn_menu_button(parent, "Quit", MenuButtonAction::Quit, &asset_server);
                });
        });
}

/// Creates a menu button with text and interaction handlers
fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuButtonAction,
    _asset_server: &AssetServer,
) {
    parent
        .spawn((
            button_style(),
            BackgroundColor(NORMAL_BUTTON),
            Button,
            action,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                text_style(),
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
                    MenuButtonAction::Restart => {
                        // Reset the context flag since we want a full restart
                        context.from_pause_menu = false;
                        next_state.set(GameMenuState::Loading);
                    }
                    MenuButtonAction::MainMenu => {
                        // Reset the context flag since we're going to main menu
                        context.from_pause_menu = false;
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
    }
}
