use crate::menu::state::GameMenuState;
use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, PositionType, UiRect, Val};

/// Component for the Star of David image
#[derive(Component)]
pub struct StarOfDavid;

/// Resource to control logging frequency and track entity state
#[derive(Resource)]
pub struct StarOfDavidLogState {
    /// Last recorded number of stars
    last_star_count: usize,
}

impl Default for StarOfDavidLogState {
    fn default() -> Self {
        Self { last_star_count: 0 }
    }
}

/// Plugin that renders the Star of David
pub struct StarOfDavidPlugin;

impl Plugin for StarOfDavidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarOfDavidLogState>()
            // Split into two systems: one for monitoring changes and one for rendering
            .add_systems(
                Update,
                (monitor_star_of_david_changes, render_star_of_david).run_if(
                    |state: Res<State<GameMenuState>>| {
                        matches!(
                            state.get(),
                            GameMenuState::MainMenu | GameMenuState::PausedGame
                        )
                    },
                ),
            );
    }

    fn finish(&self, _app: &mut App) {
        // No custom pipeline setup needed
    }
}

/// System to monitor changes in StarOfDavid entities and log when necessary
pub fn monitor_star_of_david_changes(
    stars: Query<(Entity, Option<&Parent>, Option<&Name>, &Visibility), With<StarOfDavid>>,
    mut log_state: ResMut<StarOfDavidLogState>,
    mut commands: Commands,
    menu_state: Res<State<crate::menu::state::GameMenuState>>,
) {
    let current_count = stars.iter().count();
    let visible_count = stars
        .iter()
        .filter(|(_, _, _, vis)| *vis == Visibility::Visible)
        .count();

    // Track stars by context (pause menu vs main menu)
    let current_state = menu_state.get();

    // Only log when the count changes or if multiple visible stars detected
    if current_count != log_state.last_star_count || visible_count > 1 {
        info!(
            "Star of David status: {} total, {} visible in state {:?}",
            current_count, visible_count, current_state
        );
        log_state.last_star_count = current_count;
    }

    // Only enforce star visibility if needed, not every frame
    // Find stars that need to be shown in current state
    let mut main_menu_stars = Vec::new();
    let mut pause_stars = Vec::new();

    // Categorize stars by name
    for (entity, _, name, visibility) in stars.iter() {
        if let Some(name_comp) = name {
            let name_str = name_comp.as_str();
            if name_str.contains("Main Menu") {
                main_menu_stars.push((entity, *visibility));
            } else if name_str.contains("Pause") {
                pause_stars.push((entity, *visibility));
            }
        }
    }

    // Choose which stars should be visible based on current state
    let needs_main_menu_star = matches!(current_state, crate::menu::state::GameMenuState::MainMenu);
    let needs_pause_star = matches!(current_state, crate::menu::state::GameMenuState::PausedGame);

    // Handle main menu stars
    if needs_main_menu_star && !main_menu_stars.is_empty() {
        // Find the first main menu star
        let first_main_menu_star = main_menu_stars[0].0;

        // Only update visibility if it's not already visible
        if main_menu_stars[0].1 != Visibility::Visible {
            info!("Making main menu star visible: {:?}", first_main_menu_star);
            commands
                .entity(first_main_menu_star)
                .insert(Visibility::Visible);
        }

        // Hide other main menu stars (if any)
        for (entity, visibility) in main_menu_stars.iter().skip(1) {
            if *visibility != Visibility::Hidden {
                commands.entity(*entity).insert(Visibility::Hidden);
            }
        }
    } else if !needs_main_menu_star {
        // Hide all main menu stars if we're not in main menu
        for (entity, visibility) in main_menu_stars.iter() {
            if *visibility != Visibility::Hidden {
                commands.entity(*entity).insert(Visibility::Hidden);
            }
        }
    }

    // Handle pause stars
    if needs_pause_star && !pause_stars.is_empty() {
        // Find the first pause star
        let first_pause_star = pause_stars[0].0;

        // Only update visibility if it's not already visible
        if pause_stars[0].1 != Visibility::Visible {
            info!("Making pause star visible: {:?}", first_pause_star);
            commands
                .entity(first_pause_star)
                .insert(Visibility::Visible);
        }

        // Hide other pause stars (if any)
        for (entity, visibility) in pause_stars.iter().skip(1) {
            if *visibility != Visibility::Hidden {
                commands.entity(*entity).insert(Visibility::Hidden);
            }
        }
    } else if !needs_pause_star {
        // Hide all pause stars if we're not in pause menu
        for (entity, visibility) in pause_stars.iter() {
            if *visibility != Visibility::Hidden {
                commands.entity(*entity).insert(Visibility::Hidden);
            }
        }
    }

    // Remove excess stars if we have more than we need (more than one per type)
    if current_count > 2 {
        info!("Cleaning up excess stars to prevent duplication issues");

        // Collect stars to keep (one main menu star and one pause star at most)
        let mut stars_to_keep = Vec::new();
        let mut main_menu_star_kept = false;
        let mut pause_star_kept = false;

        for (entity, _, name, _) in stars.iter() {
            if let Some(name_comp) = name {
                let name_str = name_comp.as_str();
                if name_str.contains("Main Menu") && !main_menu_star_kept {
                    stars_to_keep.push(entity);
                    main_menu_star_kept = true;
                } else if name_str.contains("Pause") && !pause_star_kept {
                    stars_to_keep.push(entity);
                    pause_star_kept = true;
                }
            }
        }

        // Despawn any stars that aren't the ones we're keeping
        for (entity, _, _, _) in stars.iter() {
            if !stars_to_keep.contains(&entity) {
                warn!("Despawning excess Star of David: {:?}", entity);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

/// System to create and render the Star of David using UI image
pub fn render_star_of_david(
    mut commands: Commands,
    query: Query<Entity, (With<StarOfDavid>, Without<Children>)>,
    asset_server: Res<AssetServer>,
) {
    // Only process entities that don't have children yet
    for entity in &query {
        info!(
            "Rendering Star of David as UI image for entity {:?}",
            entity
        );

        // First, ensure the parent entity has proper UI components
        commands.entity(entity).insert((
            ZIndex::default(),
            Transform::default(),
            GlobalTransform::default(),
        ));

        // Use ImageNode to display the star image
        commands.entity(entity).with_children(|parent| {
            // Create a UI image node for the star - don't add RenderLayers since it inherits from parent
            parent.spawn((
                // Use individual components instead of NodeBundle
                Node {
                    width: Val::Px(120.0),
                    height: Val::Px(120.0),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                ZIndex(100), // Use ZIndex directly with a value
                Visibility::Inherited,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                ImageNode::new(asset_server.load("textures/star.png")),
                Name::new("Star Image"),
            ));
        });
    }
}

/// Create a Star of David bundle for spawning
pub fn create_star_of_david() -> impl Bundle {
    info!("Creating StarOfDavid bundle as UI component");
    (
        // Use individual components instead of NodeBundle
        Node {
            width: Val::Px(120.0),
            height: Val::Px(120.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        StarOfDavid,
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
        // Add this to ensure we're in a UI hierarchy
        ZIndex::default(),
        Transform::default(),
        GlobalTransform::default(),
    )
}
