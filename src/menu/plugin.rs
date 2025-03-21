use bevy::prelude::*;

use crate::{
    camera::components::GameCamera,
    cards::Card,
    menu::{
        camera::{
            ensure_single_menu_camera, manage_camera_visibility, manage_pause_camera_visibility,
            setup_menu_camera,
        },
        cleanup::{
            cleanup_game, cleanup_main_menu, cleanup_menu_camera, cleanup_pause_menu,
            cleanup_star_of_david_thoroughly,
        },
        components::{MenuBackground, MenuCamera, MenuItem, MenuRoot},
        credits::CreditsPlugin,
        deck::DeckManagerPlugin,
        input_blocker::InputBlockerPlugin,
        logo::{StarOfDavid, StarOfDavidPlugin, render_star_of_david},
        main::MainMenuPlugin,
        main_menu::{menu_action, set_menu_camera_zoom, setup_main_menu},
        pause_menu::{handle_pause_input, pause_menu_action, setup_pause_menu},
        save_load::SaveLoadUiPlugin,
        settings::{SettingsMenuState, SettingsPlugin},
        stars,
        state::GameMenuState,
        state::StateTransitionContext,
        state_transitions, ui,
    },
};

// Import types from the ui module
use crate::menu::ui::{MenuVisibilityLogState, MenuVisibilityState, PreviousWindowSize};

/// Plugin that sets up the menu system and its related systems
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMenuState>()
            .insert_resource(GameMenuState::MainMenu)
            .insert_resource(StateTransitionContext::default())
            .init_resource::<MenuVisibilityLogState>()
            .init_resource::<MenuVisibilityState>()
            .add_plugins((
                StarOfDavidPlugin,
                SettingsPlugin,
                MainMenuPlugin,
                CreditsPlugin,
                DeckManagerPlugin,
                SaveLoadUiPlugin,
                InputBlockerPlugin,
            ))
            // Main Menu state
            .add_systems(
                OnEnter(GameMenuState::MainMenu),
                (
                    cleanup_game,
                    cleanup_menu_camera,
                    cleanup_star_of_david_thoroughly,
                    // First ensure all old menu items are cleaned up
                    |mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>| {
                        let count = menu_items.iter().count();
                        if count > 0 {
                            info!("Cleaning up {} existing menu items before setup", count);
                            for entity in menu_items.iter() {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                    },
                    // Then set up new menu items
                    setup_main_menu,
                    setup_menu_camera,
                    set_menu_camera_zoom,
                    ensure_single_menu_camera,
                    stars::setup_main_menu_star,
                    // Finally verify and force visibility
                    |mut commands: Commands, menu_items: Query<(Entity, &Visibility), With<MenuItem>>| {
                        let count = menu_items.iter().count();
                        info!("After main menu setup, found {} menu items", count);
                        if count == 0 {
                            warn!("No menu items found after main menu setup - this may indicate a setup issue");
                        } else {
                            // Force visibility for all menu items
                            for (entity, visibility) in menu_items.iter() {
                                if *visibility != Visibility::Visible {
                                    info!("Forcing menu item {:?} to be visible", entity);
                                    commands.entity(entity).insert(Visibility::Visible);
                                }
                            }
                        }
                    },
                    // Add an additional check to ensure UI items have proper parents
                    |menu_items: Query<(Entity, Option<&Parent>, Option<&Name>), With<MenuItem>>,
                     mut commands: Commands,
                     ui_node_query: Query<Entity, With<Node>>| {
                        // Check for any menu items that don't have a proper parent
                        for (entity, parent, name) in menu_items.iter() {
                            if let Some(parent_ref) = parent {
                                // Check if parent has a UI node component
                                if !ui_node_query.contains(parent_ref.get()) {
                                    let name_str = name.map_or(String::from("unnamed"), |n| n.to_string());
                                    warn!("Main menu item has non-UI parent: {:?} ({})", entity, name_str);
                                    
                                    // For orphaned UI elements without proper parents, we could re-parent them
                                    // This is an advanced fix - enable only if you're sure this is needed
                                    // commands.entity(parent_ref.get()).insert((Node::default(), ViewVisibility::default()));
                                }
                            }
                        }
                    }
                ).chain(),
            )
            .add_systems(
                OnExit(GameMenuState::MainMenu),
                (cleanup_main_menu, cleanup_menu_camera).chain(),
            )
            .add_systems(
                Update,
                (
                    menu_action,
                    render_star_of_david,
                    ui::update_menu_visibility_state,
                    ui::debug_menu_visibility,
                    ui::update_menu_background,
                    // Add a system to ensure menu items are visible when in MainMenu state
                    |menu_items: Query<(Entity, &Visibility), With<MenuItem>>, 
                     mut commands: Commands,
                     game_state: Res<State<GameMenuState>>| {
                        // Only run when in MainMenu state
                        if *game_state.get() != GameMenuState::MainMenu {
                            return;
                        }
                        
                        let count = menu_items.iter().count();
                        let hidden_count = menu_items.iter()
                            .filter(|(_, visibility)| **visibility != Visibility::Visible)
                            .count();
                            
                        if hidden_count > 0 {
                            info!("Found {} hidden menu items out of {} total, forcing visibility", 
                                  hidden_count, count);
                                  
                            // Force visibility for any menu items that aren't visible
                            for (entity, visibility) in menu_items.iter() {
                                if *visibility != Visibility::Visible {
                                    commands.entity(entity).insert(Visibility::Visible);
                                }
                            }
                        }
                    }
                )
                    .run_if(in_state(GameMenuState::MainMenu)),
            )
            // Ensure menu items are visible when in MainMenu
            .add_systems(
                Update,
                (|mut menu_items: Query<(&mut Visibility, &Name), (With<MenuItem>, Changed<Visibility>)>| {
                    // Only update items whose visibility has changed
                    if !menu_items.is_empty() {
                        info!("Setting visibility for {} changed menu items", menu_items.iter().count());
                        
                        for (mut visibility, name) in menu_items.iter_mut() {
                            if *visibility != Visibility::Visible {
                                info!("Setting menu item '{}' visibility to Visible", name);
                                *visibility = Visibility::Visible;
                            }
                        }
                    }
                }).run_if(in_state(GameMenuState::MainMenu))
            )
            // Loading state systems
            .add_systems(
                OnEnter(GameMenuState::Loading),
                (cleanup_game, cleanup_menu_camera).chain(),
            )
            .add_systems(
                Update,
                state_transitions::start_game_loading.run_if(in_state(GameMenuState::Loading)),
            )
            .add_systems(
                OnExit(GameMenuState::Loading),
                state_transitions::finish_loading,
            )
            // Pause menu systems
            .add_systems(
                OnEnter(GameMenuState::PausedGame),
                (
                    cleanup_menu_camera,
                    setup_pause_menu,
                    setup_menu_camera,
                    ensure_single_menu_camera,
                    manage_pause_camera_visibility,
                    stars::setup_pause_star,
                ),
            )
            .add_systems(
                OnExit(GameMenuState::PausedGame),
                (cleanup_pause_menu).chain(),
            )
            .add_systems(
                Update,
                (pause_menu_action, render_star_of_david)
                    .run_if(in_state(GameMenuState::PausedGame)),
            )
            // Settings menu systems
            .add_systems(
                OnEnter(GameMenuState::Settings),
                (
                    setup_menu_camera,
                    state_transitions::setup_settings_transition,
                )
                    .chain(),
            )
            .add_systems(
                OnExit(GameMenuState::Settings),
                |context: Res<StateTransitionContext>| {
                    // Log the transition from settings
                    info!(
                        "Exiting settings, returning to {:?}",
                        context.settings_origin
                    );
                },
            )
            // InGame state systems
            .add_systems(
                OnEnter(GameMenuState::InGame),
                (cleanup_menu_camera, cleanup_star_of_david_thoroughly).chain(),
            )
            .add_systems(Update, handle_pause_input)
            // Run camera visibility management in all states, but with proper ordering
            .add_systems(PostUpdate, manage_camera_visibility)
            // Add a system to ensure menu items are visible in appropriate states
            .add_systems(
                PostUpdate,
                |mut menu_items: Query<(&mut Visibility, &Name), With<MenuItem>>,
                 state: Res<State<GameMenuState>>| {
                    let should_be_visible = matches!(
                        state.get(),
                        GameMenuState::MainMenu | GameMenuState::PausedGame | GameMenuState::Settings
                    );
                    
                    for (mut visibility, name) in menu_items.iter_mut() {
                        if should_be_visible && *visibility != Visibility::Visible {
                            info!("Setting menu item '{}' to Visible in state {:?}", name, state.get());
                            *visibility = Visibility::Visible;
                        } else if !should_be_visible && *visibility == Visibility::Visible {
                            info!("Setting menu item '{}' to Hidden in state {:?}", name, state.get());
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
            )
            // Add system to monitor state transitions and diagnostics
            .add_systems(
                PostUpdate,
                |state: Res<State<GameMenuState>>, 
                 _next_state: ResMut<NextState<GameMenuState>>,
                 mut last_state: Local<Option<GameMenuState>>,
                 menu_items: Query<Entity, With<MenuItem>>,
                 mut commands: Commands,
                 asset_server: Res<AssetServer>| {
                    // If the state changed, log it
                    if last_state.is_none() || *last_state.as_ref().unwrap() != *state.get() {
                        if let Some(old_state) = last_state.as_ref() {
                            info!("State changed from {:?} to {:?}", old_state, state.get());
                        } else {
                            info!("Initial state: {:?}", state.get());
                        }
                        *last_state = Some(*state.get());
                        
                        // If we're in MainMenu but have no menu items, force setup
                        if *state.get() == GameMenuState::MainMenu {
                            let count = menu_items.iter().count();
                            if count == 0 {
                                info!("We're in MainMenu state but have no menu items! Forcing setup...");
                                // Recursively despawn any leftover items first
                                for entity in menu_items.iter() {
                                    commands.entity(entity).despawn_recursive();
                                }
                                
                                // Then run setup
                                setup_main_menu(commands, asset_server, menu_items);
                            } else {
                                info!("In MainMenu state with {} menu items", count);
                                
                                // Force visibility on all menu items even if they exist
                                for entity in menu_items.iter() {
                                    commands.entity(entity).insert(Visibility::Visible);
                                }
                            }
                        }
                    }
                }
            )
            // Add a separate system for periodic checking of main menu items
            .add_systems(
                PostUpdate,
                |state: Res<State<GameMenuState>>,
                 mut commands: Commands,
                 menu_items: Query<Entity, With<MenuItem>>,
                 asset_server: Res<AssetServer>| {
                    // Periodically check if we're in MainMenu state but have no visible menu items
                    if *state.get() == GameMenuState::MainMenu {
                        let count = menu_items.iter().count();
                        
                        if count == 0 {
                            info!("No menu items found in MainMenu state! Scheduling setup...");
                            
                            // Since we can't directly call setup_main_menu here because of the borrowing issues,
                            // we'll set a flag in a resource to trigger the setup in another system
                            commands.insert_resource(NeedsMainMenuSetup(true));
                        }
                    }
                }
            )
            // Add a system to run after the check that will actually perform the setup if needed
            .add_systems(
                PostUpdate,
                |mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 menu_items: Query<Entity, With<MenuItem>>,
                 setup_flag: Option<Res<NeedsMainMenuSetup>>,
                 mut next_state: ResMut<NextState<GameMenuState>>| {
                    // Only proceed if the flag resource exists and is set to true
                    if let Some(flag) = setup_flag {
                        if flag.0 {
                            // Remove the flag first
                            commands.remove_resource::<NeedsMainMenuSetup>();
                            
                            // Set up the main menu
                            info!("Setting up main menu from dedicated system");
                            setup_main_menu(commands, asset_server, menu_items);
                            
                            // Force state refresh to trigger OnEnter systems
                            let current_state = GameMenuState::MainMenu;
                            next_state.set(current_state);
                        }
                    }
                }
            )
            // Define the resource to track when we need to set up the main menu
            .init_resource::<NeedsMainMenuSetup>()
            // Add startup system to ensure menu components are visible on first run
            .add_systems(
                Startup,
                |mut menu_items: Query<(&mut Visibility, Option<&Name>), With<MenuItem>>,
                 _commands: Commands| {
                    let item_count = menu_items.iter().count();
                    info!("On startup, found {} menu items to force visible", item_count);
                    
                    for (mut visibility, name) in menu_items.iter_mut() {
                        if *visibility != Visibility::Visible {
                            if let Some(name) = name {
                                info!("Forcing '{}' to be visible on startup", name);
                            } else {
                                info!("Forcing unnamed menu item to be visible on startup");
                            }
                            *visibility = Visibility::Visible;
                        }
                    }
                }
            )
            // Add enforcement system that runs only when visibility changes
            .add_systems(
                PostUpdate,
                |mut items: Query<(&mut Visibility, &GlobalZIndex, &Name), (With<MenuItem>, Changed<Visibility>)>| {
                    let item_count = items.iter().count();
                    if item_count > 0 {
                        info!("Fixing visibility for {} changed menu items", item_count);
                        
                        for (mut visibility, z_index, name) in items.iter_mut() {
                            if *visibility != Visibility::Visible && z_index.0 > 0 {
                                info!("Forcing menu item '{}' to be visible", name);
                                *visibility = Visibility::Visible;
                            }
                        }
                    }
                }
            )
            // Add a system to ensure menu items have proper UI hierarchies
            .add_systems(
                PostUpdate,
                |menu_items: Query<(Entity, &Parent, Option<&Name>, &Node), With<MenuItem>>,
                 parents: Query<Entity, (Without<Node>, Without<ViewVisibility>)>,
                 mut commands: Commands,
                 mut found_issues: Local<bool>| {
                    // Only run this diagnostic once if issues are found
                    if *found_issues {
                        return;
                    }
                    
                    // Check for menu items that have non-UI parent entities
                    let mut issues = false;
                    for (entity, parent, name, _) in menu_items.iter() {
                        if parents.contains(parent.get()) {
                            issues = true;
                            let name_str = name.map_or(String::from("unnamed"), |n| n.to_string());
                            warn!("UI hierarchy issue: Node {:?} ({}) is in a non-UI entity hierarchy", 
                                  entity, name_str);
                            
                            // For serious hierarchy issues, we could try to fix them here
                            // Example: commands.entity(parent.get()).insert((Node::default(), ViewVisibility::default()));
                        }
                    }
                    
                    // Set the flag if issues were found
                    if issues {
                        warn!("UI hierarchy issues detected - this may cause layout problems");
                        *found_issues = true;
                    }
                }
            );
    }
}

/// Set up the transition context for settings menu
fn setup_settings_transition(
    mut context: ResMut<StateTransitionContext>,
    current_state: Res<State<GameMenuState>>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
) {
    info!(
        "Setting up settings transition from state: {:?}, from_pause_menu: {}",
        current_state.get(),
        context.from_pause_menu
    );

    // Always reset from_pause_menu flag when transitioning from MainMenu
    if *current_state.get() == GameMenuState::MainMenu {
        info!("Resetting from_pause_menu flag because we're in MainMenu state");
        context.from_pause_menu = false;
        // Explicitly set the settings origin to MainMenu
        info!("Explicitly setting settings_origin to MainMenu");
        context.settings_origin = Some(GameMenuState::MainMenu);
    } else if context.from_pause_menu || *current_state.get() == GameMenuState::PausedGame {
        // If the flag is set or we're coming from the pause menu, set the origin to PausedGame
        info!("Detected transition from pause menu");
        context.settings_origin = Some(GameMenuState::PausedGame);
    } else {
        // Fall back to checking the current state
        match current_state.get() {
            GameMenuState::Settings if context.settings_origin.is_none() => {
                // If we're already in Settings state but have no origin,
                // default to main menu
                info!("Already in Settings state with no origin, defaulting to main menu");
                context.settings_origin = Some(GameMenuState::MainMenu);
            }
            _ => {
                if context.settings_origin.is_none() {
                    // Default to main menu if coming from an unexpected state
                    info!("Entering settings from unexpected state, defaulting to main menu");
                    context.settings_origin = Some(GameMenuState::MainMenu);
                } else {
                    info!(
                        "Using existing settings origin: {:?}",
                        context.settings_origin
                    );
                }
            }
        }
    }

    // Ensure we're showing the main settings screen when entering settings
    info!("Setting SettingsMenuState to Main");
    settings_state.set(SettingsMenuState::Main);
}

/// Setup Star of David for pause menu
fn setup_pause_star(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    asset_server: Res<AssetServer>,
    existing_stars: Query<Entity, With<StarOfDavid>>,
) {
    use crate::menu::logo::{
        create_english_text, create_hebrew_text, create_logo, create_star_of_david,
    };

    // Skip if we already have stars to prevent duplicates
    if !existing_stars.is_empty() {
        info!("Stars of David already exist, skipping creation in pause menu");
        return;
    }

    info!("Setting up Star of David for pause menu");

    // Find the menu camera for text elements
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!(
            "Attaching Star of David and text to pause menu camera: {:?}",
            camera_entity
        );

        // Create a complete entity hierarchy using a single commands operation
        commands.entity(camera_entity).with_children(|parent| {
            // Create a parent entity that will contain the star and text elements
            parent
                .spawn((create_logo(), Name::new("Pause Logo Group")))
                .with_children(|logo_parent| {
                    // Spawn the Star of David with the logo container as parent
                    logo_parent.spawn((create_star_of_david(), Name::new("Pause Star of David")));

                    // Spawn the Hebrew text below the star
                    logo_parent.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Pause Hebrew Logo Text"),
                    ));

                    // Spawn the English text below the Hebrew text
                    logo_parent.spawn((
                        create_english_text(&asset_server),
                        Name::new("Pause English Logo Text"),
                    ));
                });
        });

        info!("Created pause menu logo container with Star of David and text elements");
    } else {
        warn!("No menu camera found for pause menu Star of David and text");

        // If no camera is found, create a standalone UI node hierarchy
        commands
            .spawn((
                // Ensure this is a proper UI node at the root
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                Name::new("Pause Menu Root"),
                MenuRoot,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Transform::default(),
                GlobalTransform::default(),
            ))
            .with_children(|parent| {
                parent
                    .spawn((create_logo(), Name::new("Pause Logo Group")))
                    .with_children(|logo_parent| {
                        // Spawn the Star of David with the logo container as parent
                        logo_parent
                            .spawn((create_star_of_david(), Name::new("Pause Star of David")));

                        // Spawn the Hebrew text below the star
                        logo_parent.spawn((
                            create_hebrew_text(&asset_server),
                            Name::new("Pause Hebrew Logo Text"),
                        ));

                        // Spawn the English text below the Hebrew text
                        logo_parent.spawn((
                            create_english_text(&asset_server),
                            Name::new("Pause English Logo Text"),
                        ));
                    });
            });

        info!("Created standalone pause menu logo container with Star of David and text elements");
    }
}

/// Starts the game loading process
pub fn start_game_loading(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    cards: Query<Entity, With<Card>>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Check if we're coming from the pause menu
    if context.from_pause_menu {
        info!("Coming from pause menu, skipping loading process and going directly to InGame");
        // Reset the flag
        context.from_pause_menu = false;

        // When resuming from pause menu, we shouldn't spawn new cameras
        // Go directly to InGame without performing cleanup that would remove game entities
        next_state.set(GameMenuState::InGame);
        return;
    }

    // Normal loading process
    info!("Checking game state for transition...");
    info!("Number of cards: {}", cards.iter().count());
    info!("Number of game cameras: {}", game_cameras.iter().count());

    // Force cleanup if any game cameras or cards remain
    if !game_cameras.is_empty() {
        warn!(
            "Cleaning up {} remaining game cameras",
            game_cameras.iter().count()
        );
        for entity in game_cameras.iter() {
            info!("Force despawning game camera entity: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }

    if !cards.is_empty() {
        warn!("Cleaning up {} remaining cards", cards.iter().count());
        for entity in cards.iter() {
            info!("Force despawning card entity: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }

    // Transition to InGame state
    info!("Transitioning to InGame state...");
    next_state.set(GameMenuState::InGame);
}

/// Finishes the game loading process
fn finish_loading() {
    info!("Loading state finished");
}

/// Update menu visibility state resource
fn update_menu_visibility_state(
    menu_items: Query<&Visibility, With<MenuItem>>,
    mut menu_state: ResMut<MenuVisibilityState>,
) {
    let total_items = menu_items.iter().count();
    let visible_items = menu_items
        .iter()
        .filter(|visibility| matches!(visibility, Visibility::Visible))
        .count();

    // Only update if changed
    if menu_state.item_count != total_items || menu_state.visible_count != visible_items {
        menu_state.item_count = total_items;
        menu_state.visible_count = visible_items;
    }
}

/// Debug system to check visibility of menu elements
fn debug_menu_visibility(
    menu_cameras: Query<(Entity, &Visibility), (With<MenuCamera>, Changed<Visibility>)>,
    menu_items: Query<(Entity, &Visibility), (With<MenuItem>, Changed<Visibility>)>,
    mut log_state: ResMut<MenuVisibilityLogState>,
    menu_state: Res<MenuVisibilityState>,
) {
    // Count cameras and menu items with changed visibility
    let camera_count = menu_cameras.iter().count();
    let menu_item_count = menu_items.iter().count();

    // Only proceed if any component actually changed
    if camera_count == 0 && menu_item_count == 0 {
        return;
    }

    // Collect current camera states for changed cameras
    for (entity, visibility) in menu_cameras.iter() {
        log_state.camera_states.insert(entity, *visibility);
        debug!(
            "Menu camera {:?} visibility changed to: {:?}",
            entity, visibility
        );
    }

    // Count visible items
    if menu_item_count > 0 {
        // We need to query all items to get the total count when visibility changes
        let all_items = menu_state.item_count;
        let visible_items = menu_state.visible_count;

        // Only log if visibility actually changed
        if visible_items != log_state.last_visible_items {
            log_state.last_item_count = all_items;
            log_state.last_visible_items = visible_items;
            debug!("Total menu items: {}", all_items);
            debug!("Visible menu items: {}/{}", visible_items, all_items);
        }
    }
}

/// Handle cleanup when returning to main menu
#[allow(dead_code)]
fn handle_game_cleanup(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // ... existing code ...
}

/// System to update the menu background image size based on window dimensions
fn update_menu_background(
    windows: Query<&Window>,
    mut backgrounds: Query<(&mut Node, &mut PreviousWindowSize), With<MenuBackground>>,
    mut missing_size_backgrounds: Query<
        (Entity, &mut Node),
        (With<MenuBackground>, Without<PreviousWindowSize>),
    >,
    mut commands: Commands,
) {
    // Get the primary window
    if let Ok(window) = windows.get_single() {
        let current_width = window.width();
        let current_height = window.height();

        // Get all background image nodes and update their size
        for (mut node, mut prev_size) in &mut backgrounds {
            // Check if window size has changed
            if prev_size.width != current_width || prev_size.height != current_height {
                // Update the UI node size to match the window size exactly
                node.width = Val::Px(current_width);
                node.height = Val::Px(current_height);

                // Update the previous size
                prev_size.width = current_width;
                prev_size.height = current_height;

                // Log window size changes at debug level
                debug!(
                    "Window size changed: {}x{}, updating menu background size",
                    current_width, current_height
                );
            }
        }

        // Add PreviousWindowSize component to any background nodes that don't have it
        for (entity, mut node) in missing_size_backgrounds.iter_mut() {
            // Update the node size
            node.width = Val::Px(current_width);
            node.height = Val::Px(current_height);

            // Add the PreviousWindowSize component
            commands.entity(entity).insert(PreviousWindowSize {
                width: current_width,
                height: current_height,
            });

            debug!(
                "Added PreviousWindowSize component to menu background. Window size: {}x{}",
                current_width, current_height
            );
        }
    }
}

/// Sets up a Star of David for the main menu and attaches it to the menu camera
fn setup_main_menu_star(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    asset_server: Res<AssetServer>,
) {
    use crate::menu::logo::{
        create_english_text, create_hebrew_text, create_logo, create_star_of_david,
    };

    info!("Setting up Star of David for main menu");

    // Check if we have a menu camera to attach to
    if let Some(camera) = menu_cameras.iter().next() {
        info!("Found menu camera for Star of David: {:?}", camera);
        
        // Create the star of david with text directly under camera
        commands.entity(camera).with_children(|parent| {
            // Create a logo container
            parent.spawn((
                create_logo(),
                Name::new("Main Menu Logo Container"),
                MenuItem,
                Visibility::Visible,
                GlobalZIndex(30),
            ))
            .with_children(|container| {
                // Add the Star of David
                container.spawn((
                    create_star_of_david(),
                    Name::new("Main Menu Star of David"),
                    MenuItem, 
                    Visibility::Visible,
                    GlobalZIndex(31),
                ));
                
                // Add Hebrew text
                container.spawn((
                    create_hebrew_text(&asset_server),
                    Name::new("Main Menu Hebrew Text"),
                    MenuItem,
                    Visibility::Visible,
                    GlobalZIndex(31),
                ));
                
                // Add English text
                container.spawn((
                    create_english_text(&asset_server),
                    Name::new("Main Menu English Text"),
                    MenuItem,
                    Visibility::Visible,
                    GlobalZIndex(31),
                ));
            });
        });
        
        info!("Created and attached Star of David and text to menu camera");
    } else {
        // No camera found, create standalone logo
        warn!("No menu camera found, creating standalone logo");
        
        // Create a root node with the logo as its child
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Name::new("Main Menu Logo Root"),
                MenuRoot,
                MenuItem,
                Visibility::Visible,
                GlobalZIndex(30),
            ))
            .with_children(|parent| {
                // Create the logo container as a child of the root
                parent.spawn((
                    create_logo(),
                    Name::new("Main Menu Logo Container"),
                    MenuItem,
                    Visibility::Visible,
                    GlobalZIndex(31),
                ))
                .with_children(|container| {
                    // Add the Star of David
                    container.spawn((
                        create_star_of_david(),
                        Name::new("Main Menu Star of David"),
                        MenuItem, 
                        Visibility::Visible,
                        GlobalZIndex(32),
                    ));
                    
                    // Add Hebrew text
                    container.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Main Menu Hebrew Text"),
                        MenuItem,
                        Visibility::Visible,
                        GlobalZIndex(32),
                    ));
                    
                    // Add English text
                    container.spawn((
                        create_english_text(&asset_server),
                        Name::new("Main Menu English Text"),
                        MenuItem,
                        Visibility::Visible,
                        GlobalZIndex(32),
                    ));
                });
            });
        
        info!("Created standalone Star of David and text");
    }
}

/// Creates the logo container for menu items
fn create_logo() -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(300.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        Visibility::Visible,
        ZIndex::default(),    // Ensure we have a ZIndex for proper UI hierarchy
        Transform::default(), // Use Transform instead of TransformBundle
        GlobalTransform::default(), // Add GlobalTransform explicitly
    )
}

/// Resource to track when we need to set up the main menu
#[derive(Resource, Default)]
struct NeedsMainMenuSetup(bool);
