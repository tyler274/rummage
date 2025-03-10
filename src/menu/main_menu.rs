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
    // The camera is now spawned by setup_menu_camera in plugin.rs
    // No need to spawn another camera here

    // Spawn Star of David in world space with proper z-index
    commands.spawn(create_star_of_david());

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
            AppLayer::Menu.layer(), // Add to menu layer
            Visibility::Visible,    // Explicitly set to visible
        ))
        .with_children(|parent| {
            // Add the logo
            parent
                .spawn((
                    create_logo(),
                    AppLayer::Menu.layer(), // Add to menu layer
                    Visibility::Visible,    // Explicitly set to visible
                ))
                .with_children(|parent| {
                    parent.spawn((
                        create_hebrew_text(&asset_server),
                        AppLayer::Menu.layer(), // Add to menu layer
                        Visibility::Visible,    // Explicitly set to visible
                    ));
                    parent.spawn((
                        create_english_text(&asset_server),
                        AppLayer::Menu.layer(), // Add to menu layer
                        Visibility::Visible,    // Explicitly set to visible
                    ));
                    parent.spawn((
                        create_decorative_elements(),
                        AppLayer::Menu.layer(), // Add to menu layer
                        Visibility::Visible,    // Explicitly set to visible
                    ));
                });

            // Menu buttons container
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(50.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(), // Add to menu layer
                    Visibility::Visible,    // Explicitly set to visible
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
            button_style(),
            BackgroundColor(NORMAL_BUTTON),
            Button,
            action,
            AppLayer::Menu.layer(), // Ensure it's on the menu layer
            Visibility::Visible,    // Explicitly set to visible
        ))
        .with_children(|parent| {
            parent.spawn((
                Text(text.to_string()),
                TextFont {
                    font: asset_server.load("fonts/DejaVuSans.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
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
