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
use bevy::audio::{AudioPlayer, PlaybackMode, PlaybackSettings, Volume};
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, PositionType, UiRect, Val};

/// Component to mark background image for easier access
#[derive(Component)]
#[allow(dead_code)]
struct MenuBackground;

/// Component to mark the main menu music entity
#[derive(Component)]
pub struct MainMenuMusic;

/// Sets up the main menu interface with buttons and layout
pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // The camera is now spawned by setup_menu_camera in plugin.rs
    // No need to spawn another camera here

    // Load and play background music
    let music_handle = asset_server.load("music/Negev sings Hava Nagila [XwZwz0iCuF0].ogg");
    debug!("Starting main menu music playback");
    commands.spawn((
        AudioPlayer::new(music_handle),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.5),
            ..default()
        },
        MainMenuMusic,
        Name::new("Main Menu Music"),
    ));

    // Try to load the background image, but we won't use it for now due to the difficulties
    // We're just logging to confirm the path is correct for future reference
    let _background_handle: Handle<Image> = asset_server.load("images/menu_background.jpeg");
    info!("Attempted to load background image, not using it currently");

    // Since we've had so many issues with the background image, just focus on the current UI
    // The decorative gold styling actually looks very good!

    // Spawn Star of David in world space with proper z-index
    commands.spawn(create_star_of_david());

    // Main menu container - with rich golden background
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
            // Rich background with a deeper brown at the base - more golden tone
            BackgroundColor(Color::srgb(0.22, 0.15, 0.05)),
            MenuItem,
            AppLayer::Menu.layer(),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .with_children(|parent| {
            // Add gradient top overlay (lighter at top, creates a gradient effect)
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(70.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.4, 0.3, 0.1, 0.4)),
            ));

            // Add bottom vignette (darker at bottom)
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.05, 0.03, 0.01, 0.6)),
            ));

            // Add a center highlight (subtle radial effect)
            parent.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(70.0),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(15.0),
                    left: Val::Percent(15.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.5, 0.4, 0.15, 0.15)),
                // Add rounded corners to create a soft radial effect
                BorderRadius::all(Val::Percent(50.0)),
            ));

            // Add decorative horizontal line at top
            parent.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Px(2.0),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.7, 0.6, 0.2, 0.6)),
            ));

            // Add decorative horizontal line at bottom
            parent.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Px(2.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.7, 0.6, 0.2, 0.6)),
            ));

            // Add left decorative vertical element
            parent.spawn((
                Node {
                    width: Val::Px(8.0),
                    height: Val::Percent(60.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(5.0),
                    top: Val::Percent(20.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.7, 0.6, 0.2, 0.3)),
                BorderRadius::all(Val::Px(4.0)),
            ));

            // Add right decorative vertical element
            parent.spawn((
                Node {
                    width: Val::Px(8.0),
                    height: Val::Percent(60.0),
                    position_type: PositionType::Absolute,
                    right: Val::Percent(5.0),
                    top: Val::Percent(20.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.7, 0.6, 0.2, 0.3)),
                BorderRadius::all(Val::Px(4.0)),
            ));

            // Add diagonal decorative elements for extra flair
            parent.spawn((
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Px(1.0),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(25.0),
                    left: Val::Percent(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.7, 0.6, 0.2, 0.2)),
                // Rotate the element
                Transform::from_rotation(Quat::from_rotation_z(0.2)),
                GlobalTransform::default(),
            ));

            parent.spawn((
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Px(1.0),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(75.0),
                    left: Val::Percent(30.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.7, 0.6, 0.2, 0.2)),
                // Rotate the element in the opposite direction
                Transform::from_rotation(Quat::from_rotation_z(-0.2)),
                GlobalTransform::default(),
            ));

            // Add the logo
            parent
                .spawn((
                    create_logo(),
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        create_hebrew_text(&asset_server),
                        AppLayer::Menu.layer(),
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ));
                    parent.spawn((
                        create_english_text(&asset_server),
                        AppLayer::Menu.layer(),
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ));
                    parent.spawn((
                        create_decorative_elements(),
                        AppLayer::Menu.layer(),
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ));
                });

            // Menu buttons container with a border (using nested nodes for border effect)
            parent
                .spawn((
                    Node {
                        width: Val::Px(302.0), // Slightly larger to create border effect
                        height: Val::Px(402.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(50.0)),
                        ..default()
                    },
                    // Border color
                    BackgroundColor(Color::srgba(0.6, 0.5, 0.2, 0.3)),
                    BorderRadius::all(Val::Px(9.0)), // Slightly larger radius for outer element
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ))
                .with_children(|parent| {
                    // Inner container (actual menu background)
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
                            // Slightly transparent dark panel for buttons
                            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.85)),
                            BorderRadius::all(Val::Px(8.0)),
                            AppLayer::Menu.layer(),
                            Visibility::Visible,
                            InheritedVisibility::default(),
                            ViewVisibility::default(),
                        ))
                        .with_children(|parent| {
                            spawn_menu_button(
                                parent,
                                "New Game",
                                MenuButtonAction::NewGame,
                                &asset_server,
                            );
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
                            spawn_menu_button(
                                parent,
                                "Quit",
                                MenuButtonAction::Quit,
                                &asset_server,
                            );
                        });
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
    // Outer button container (for border effect)
    parent
        .spawn((
            Node {
                width: Val::Px(252.0), // Slightly larger for border effect
                height: Val::Px(52.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            // Border color
            BackgroundColor(Color::srgba(0.6, 0.5, 0.2, 0.5)),
            BorderRadius::all(Val::Px(5.0)),
            AppLayer::Menu.layer(),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .with_children(|parent| {
            // Inner button (actual button)
            parent
                .spawn((
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    BorderRadius::all(Val::Px(4.0)),
                    Button,
                    action,
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
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
                        TextColor(Color::srgba(0.95, 0.95, 0.95, 1.0)),
                        AppLayer::Menu.layer(),
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ));
                });
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
