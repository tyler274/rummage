use crate::camera::components::{AppLayer, MenuCamera};
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
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

/// Sets up the main menu interface with buttons and layout
pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // The camera is now spawned by setup_menu_camera in plugin.rs
    // No need to spawn another camera here

    // Spawn Star of David in world space with proper z-index
    commands.spawn(create_star_of_david());

    // Main menu container - with dark background
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
            // Black semi-transparent background
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.95)),
            MenuItem,
            AppLayer::Menu.layer(), // Add to menu layer
            Visibility::Visible,    // Explicitly set to visible
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .with_children(|parent| {
            // Add the logo
            parent
                .spawn((
                    create_logo(),
                    AppLayer::Menu.layer(), // Add to menu layer
                    Visibility::Visible,    // Explicitly set to visible
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        create_hebrew_text(&asset_server),
                        AppLayer::Menu.layer(), // Add to menu layer
                        Visibility::Visible,    // Explicitly set to visible
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ));
                    parent.spawn((
                        create_english_text(&asset_server),
                        AppLayer::Menu.layer(), // Add to menu layer
                        Visibility::Visible,    // Explicitly set to visible
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ));
                    parent.spawn((
                        create_decorative_elements(),
                        AppLayer::Menu.layer(), // Add to menu layer
                        Visibility::Visible,    // Explicitly set to visible
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
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
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
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
        projection.scale = 1.0; // Menu camera should be at 1:1 scale for proper UI layout
    }
}

/// Creates a menu button with text and interaction handlers
fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuButtonAction,
    asset_server: &AssetServer,
) {
    // Button container
    parent
        .spawn((
            button_style(),
            BackgroundColor(NORMAL_BUTTON),
            Button,
            action,
            AppLayer::Menu.layer(), // Add to menu layer
            Visibility::Visible,    // Explicitly set to visible
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .with_children(|parent| {
            // Button text
            parent.spawn((
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(Color::WHITE),
                AppLayer::Menu.layer(), // Add to menu layer
                Visibility::Visible,    // Explicitly set to visible
                InheritedVisibility::default(),
                ViewVisibility::default(),
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
                // Visual feedback - change to pressed color
                *color = BackgroundColor(PRESSED_BUTTON);

                // Process the button action
                match action {
                    MenuButtonAction::NewGame => {
                        info!("New Game button pressed - transitioning to Loading state");
                        next_state.set(GameMenuState::Loading);
                    }
                    MenuButtonAction::LoadGame => {
                        info!("Load Game button pressed - not implemented yet");
                        // TODO: Implement save/load game functionality
                    }
                    MenuButtonAction::Multiplayer => {
                        info!("Multiplayer button pressed - not implemented yet");
                        // TODO: Implement multiplayer functionality
                    }
                    MenuButtonAction::Settings => {
                        info!("Settings button pressed - transitioning to Settings state");
                        next_state.set(GameMenuState::Settings);
                    }
                    MenuButtonAction::Quit => {
                        info!("Quit button pressed - exiting application");
                        exit.send(bevy::app::AppExit::default());
                    }
                    MenuButtonAction::Resume => {
                        info!("Resume button pressed - transitioning to InGame state");
                        next_state.set(GameMenuState::InGame);
                    }
                    MenuButtonAction::Restart => {
                        info!("Restart button pressed - transitioning to Loading state");
                        next_state.set(GameMenuState::Loading);
                    }
                    MenuButtonAction::MainMenu => {
                        info!("Main Menu button pressed - transitioning to MainMenu state");
                        next_state.set(GameMenuState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                // Visual feedback - change to hover color
                *color = BackgroundColor(HOVERED_BUTTON);
            }
            Interaction::None => {
                // Reset to normal color
                *color = BackgroundColor(NORMAL_BUTTON);
            }
        }
    }
}
