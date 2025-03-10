use crate::menu::{components::MenuItem, state::GameMenuState};
use bevy::prelude::*;

/// Simple menu action enum
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SimpleMenuAction {
    Play,
    Quit,
}

/// Set up a simple menu that works well in WSL2
pub fn setup_simple_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up simple menu for WSL2 compatibility");

    // Root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.2).into(),
                ..default()
            },
            MenuItem,
            Name::new("Simple Menu Root"),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                TextBundle::from_section(
                    "Rummage",
                    TextStyle {
                        font_size: 64.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
                MenuItem,
            ));

            // Play button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.25).into(),
                        ..default()
                    },
                    MenuItem,
                    SimpleMenuAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        MenuItem,
                    ));
                });

            // Quit button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.25).into(),
                        ..default()
                    },
                    MenuItem,
                    SimpleMenuAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Quit",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        MenuItem,
                    ));
                });
        });

    info!("Simple menu setup complete");
}

/// Handle simple menu actions
pub fn simple_menu_action(
    interaction_query: Query<
        (&Interaction, &SimpleMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for (interaction, menu_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match menu_action {
                SimpleMenuAction::Play => {
                    info!("Play button pressed, transitioning to game state");
                    next_state.set(GameMenuState::InGame);
                }
                SimpleMenuAction::Quit => {
                    info!("Quit button pressed, exiting application");
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}
