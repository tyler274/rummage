use crate::camera::components::AppLayer;
use crate::menu::{
    components::*,
    logo::{
        create_decorative_elements, create_english_text, create_hebrew_text, create_logo,
        create_star_of_david,
    },
    state::GameMenuState,
    styles::*,
};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, UiRect, Val};

/// Sets up the main menu interface with buttons and layout
pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up main menu");

    // Create a root node for the main menu
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
            // Use a solid background color for better visibility in WSL2
            BackgroundColor(Color::rgb(0.1, 0.1, 0.2)),
            // Ensure the menu is visible
            Visibility::Visible,
            ZIndex::Global(100), // Ensure it's on top
            MenuItem,
            Name::new("Main Menu Root"),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::from_sections([TextSection {
                    value: "Rummage".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 64.0,
                        color: Color::WHITE,
                    },
                }]),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                MenuItem,
                Name::new("Main Menu Title"),
            ));

            // Play button
            parent
                .spawn((
                    Button {},
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::rgb(0.15, 0.15, 0.25)),
                    Visibility::Visible,
                    MenuItem,
                    MenuButtonAction::NewGame,
                    Interaction::default(),
                    Name::new("Play Button"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::from_sections([TextSection {
                            value: "Play".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        }]),
                        MenuItem,
                    ));
                });

            // Quit button
            parent
                .spawn((
                    Button {},
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::rgb(0.15, 0.15, 0.25)),
                    Visibility::Visible,
                    MenuItem,
                    MenuButtonAction::Quit,
                    Interaction::default(),
                    Name::new("Quit Button"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::from_sections([TextSection {
                            value: "Quit".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        }]),
                        MenuItem,
                    ));
                });
        });

    info!("Main menu setup complete");
}

/// Sets the initial zoom level for the menu camera
pub fn set_menu_camera_zoom(mut query: Query<&mut OrthographicProjection, With<MenuCamera>>) {
    if let Ok(mut projection) = query.get_single_mut() {
        projection.scale = 0.1; // Zoom out more to see the Star of David
        projection.near = -1000.0;
        projection.far = 1000.0;
    }
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
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            Button {},
            action,
            Interaction::default(),
            AppLayer::Menu.layer(), // Ensure it's on the menu layer
            Visibility::Visible,    // Explicitly set to visible
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::from_sections([TextSection {
                    value: text.to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/DejaVuSans.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                }]),
                TextLayout::new_with_justify(JustifyText::Center),
                Node::default(),
                AppLayer::Menu.layer(), // Ensure it's on the menu layer
                Visibility::Visible,    // Explicitly set to visible
            ));
        });
}

/// Handles button interactions in the main menu
pub fn menu_action(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                match action {
                    MenuButtonAction::NewGame => {
                        next_state.set(GameMenuState::Loading);
                    }
                    MenuButtonAction::LoadGame => {
                        // TODO: Implement load game functionality
                    }
                    MenuButtonAction::Multiplayer => {
                        // TODO: Implement multiplayer functionality
                    }
                    MenuButtonAction::Settings => {
                        // TODO: Implement settings functionality
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
