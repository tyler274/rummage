use crate::camera::components::AppLayer;
use crate::menu::{
    components::{MenuButtonAction, MenuCamera, MenuItem},
    state::GameMenuState,
    styles::*,
    ui::PreviousWindowSize,
};
use bevy::audio::{AudioPlayer, PlaybackMode, PlaybackSettings, Volume};
use bevy::prelude::*;
use bevy::text::JustifyText;
#[allow(unused_imports)]
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, PositionType, UiRect, Val};

/// Component to mark background image for easier access
#[derive(Component)]
#[allow(dead_code)]
pub struct MenuBackground;

/// Component to mark the main menu music entity
#[derive(Component)]
pub struct MainMenuMusic;

/// Sets up the main menu interface with buttons and layout
pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_menu_items: Query<Entity, With<MenuItem>>,
) {
    info!("Setting up main menu interface");

    // Check for existing menu items first
    let menu_items_count = existing_menu_items.iter().count();
    if menu_items_count > 0 {
        info!(
            "Found {} existing menu items, they will be cleaned up by cleanup systems",
            menu_items_count
        );
        // Immediately clean up any existing menu items to avoid conflicts
        for entity in existing_menu_items.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Load and play background music
    let music_handle = asset_server.load("music/Negev sings Hava Nagila [XwZwz0iCuF0].ogg");
    debug!("Starting main menu music playback");
    info!("Loading music file: music/Negev sings Hava Nagila [XwZwz0iCuF0].ogg");

    // Create audio entity with increased volume and explicit settings
    let music_entity = commands
        .spawn((
            AudioPlayer::new(music_handle),
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(1.0), // Maximum volume
                speed: 1.0,               // Normal speed
                paused: false,            // Ensure not paused
                ..default()
            },
            MainMenuMusic,
            Name::new("Main Menu Music"),
        ))
        .id();

    info!("Spawned main menu music entity: {:?}", music_entity);

    // Load the background image
    let background_image: Handle<Image> = asset_server.load("images/menu_background.jpeg");
    info!("Loading menu background image: images/menu_background.jpeg");

    // Create a background using a UI Node
    let background_entity = commands
        .spawn((
            Node {
                width: Val::Px(1920.0),
                height: Val::Px(1080.0),
                ..default()
            },
            ImageNode {
                image: background_image,
                ..default()
            },
            MenuBackground,
            MenuItem,
            AppLayer::Menu.layer(),
            GlobalZIndex(-10),
            Name::new("Menu Background Image"),
            PreviousWindowSize {
                width: 1920.0,
                height: 1080.0,
            },
        ))
        .id();

    info!("Spawned menu background: {:?}", background_entity);

    // Main menu container - now transparent to let the background image show through
    let menu_container = commands
        .spawn((
            // Node component (non-deprecated in Bevy 0.15.x)
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MenuItem,
            AppLayer::Menu.layer(),
            Name::new("Main Menu Container"),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
            GlobalZIndex(5), // Ensure container is above background but below buttons
        ))
        .id();

    info!("Spawned main menu container: {:?}", menu_container);

    // Create a container for the menu buttons as a child of the menu container
    let mut button_container = Entity::PLACEHOLDER;
    commands.entity(menu_container).with_children(|parent| {
        button_container = parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                MenuItem,
                Name::new("Menu Buttons Container"),
                GlobalZIndex(5), // Ensure container is above background but below buttons
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ))
            .id();
    });

    info!("Spawned button container: {:?}", button_container);

    // Add buttons to the button container
    commands.entity(button_container).with_children(|parent| {
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

    info!("Main menu setup complete, forcing visibility...");

    // Force visibility on all menu items directly
    commands.entity(menu_container).insert(Visibility::Visible);
    commands
        .entity(button_container)
        .insert(Visibility::Visible);
    commands
        .entity(background_entity)
        .insert(Visibility::Visible);

    // After setup, verify all menu items are visible
    for entity in existing_menu_items.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }

    // Log visibility status for key entities for debugging
    info!("Main menu setup complete with visibility set");
    info!("Menu container entity: {:?}", menu_container);
    info!("Button container entity: {:?}", button_container);
    info!("Background entity: {:?}", background_entity);
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
                position_type: PositionType::Relative,
                ..default()
            },
            // Border color
            BackgroundColor(Color::srgba(0.6, 0.5, 0.2, 0.5)),
            BorderRadius::all(Val::Px(5.0)),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
            GlobalZIndex(20), // Ensure buttons appear above everything
            MenuItem,
            Name::new(format!("{} Button Container", text)),
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
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    BorderRadius::all(Val::Px(4.0)),
                    Button,
                    action,
                    Visibility::Visible,
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    GlobalZIndex(21), // Ensure inner button appears above container
                    MenuItem,
                    Name::new(format!("{} Button", text)),
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
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                        GlobalZIndex(22), // Ensure text appears above button
                        MenuItem,
                        Name::new(format!("{} Text", text)),
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
    mut save_load_state: ResMut<NextState<crate::menu::save_load::SaveLoadUiState>>,
    mut save_load_context: ResMut<crate::menu::save_load::SaveLoadUiContext>,
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
                        info!("Load Game button pressed - showing load game UI");
                        // Set the context flag to indicate we're coming from the main menu
                        save_load_context.from_pause_menu = false;
                        save_load_state.set(crate::menu::save_load::SaveLoadUiState::LoadGame);
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
                    MenuButtonAction::SaveGame => {
                        info!(
                            "Save Game button pressed from main menu - not applicable in this context"
                        );
                        // No action needed in main menu context
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
