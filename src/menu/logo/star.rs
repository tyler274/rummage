use crate::camera::components::AppLayer;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, NodeBundle, UiRect, Val};

/// Component for the Star of David shape
#[derive(Component)]
pub struct StarOfDavid;

/// Resource to control logging frequency and track entity state
#[derive(Resource)]
pub struct StarOfDavidLogState {
    /// Last recorded number of stars
    last_star_count: usize,
    /// Last recorded state of stars having children
    last_children_state: std::collections::HashMap<Entity, bool>,
}

impl Default for StarOfDavidLogState {
    fn default() -> Self {
        Self {
            last_star_count: 0,
            last_children_state: std::collections::HashMap::new(),
        }
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
fn monitor_star_of_david_changes(
    query: Query<Entity, (With<StarOfDavid>, Changed<Children>)>,
    children_query: Query<&Children>,
    mut log_state: ResMut<StarOfDavidLogState>,
    star_count_query: Query<Entity, With<StarOfDavid>>,
) {
    // Only process if changes or if star count has changed
    let current_star_count = star_count_query.iter().count();
    let count_changed = current_star_count != log_state.last_star_count;

    if count_changed {
        debug!("StarOfDavid entities found: {}", current_star_count);
        log_state.last_star_count = current_star_count;
    }

    // Check for entities with changed Children component
    for entity in &query {
        let has_children = children_query
            .get(entity)
            .map(|children| !children.is_empty())
            .unwrap_or(false);

        debug!(
            "StarOfDavid entity {:?} children changed, has children: {}",
            entity, has_children
        );
    }
}

/// System to create and render the Star of David using UI components
pub fn render_star_of_david(
    mut commands: Commands,
    query: Query<Entity, (With<StarOfDavid>, Without<Children>)>,
    windows: Query<&Window>,
) {
    // Get window dimensions for proper positioning
    let window_width = windows.iter().next().map(|w| w.width()).unwrap_or(1920.0);
    let window_height = windows.iter().next().map(|w| w.height()).unwrap_or(1080.0);

    // Only process entities that don't have children yet
    for entity in &query {
        info!(
            "Rendering Star of David as UI element for entity {:?}, window: {}x{}",
            entity, window_width, window_height
        );

        // Create two triangles as UI elements with borders
        commands.entity(entity).with_children(|parent| {
            // Container for the star
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            position_type: PositionType::Absolute,
                            left: Val::Percent(50.0),
                            top: Val::Px(50.0),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Star Container"),
                ))
                .with_children(|star_parent| {
                    // First triangle (pointing up)
                    star_parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                ..default()
                            },
                            background_color: Color::srgb(1.0, 1.0, 0.0).into(),
                            ..default()
                        },
                        BorderRadius::all(Val::Percent(50.0)),
                        Outline {
                            width: Val::Px(2.0),
                            color: Color::GOLD,
                            offset: Val::Px(0.0),
                        },
                        Name::new("Star Triangle Up"),
                    ));

                    // Second triangle (pointing down)
                    star_parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                transform_origin: TransformOrigin::Center,
                                ..default()
                            },
                            background_color: Color::srgb(1.0, 1.0, 0.0).into(),
                            transform: Transform::from_rotation(Quat::from_rotation_z(
                                std::f32::consts::PI,
                            )),
                            ..default()
                        },
                        BorderRadius::all(Val::Percent(50.0)),
                        Outline {
                            width: Val::Px(2.0),
                            color: Color::GOLD,
                            offset: Val::Px(0.0),
                        },
                        Name::new("Star Triangle Down"),
                    ));
                });
        });
    }
}

/// Create a Star of David bundle for spawning
pub fn create_star_of_david() -> impl Bundle {
    info!("Creating StarOfDavid bundle as UI component");
    (
        // Use UI-oriented components
        NodeBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
            z_index: ZIndex::Global(50), // High z-index for visibility
            ..default()
        },
        StarOfDavid,
        AppLayer::Menu.layer(), // Only visible on menu layer
        Name::new("Star of David UI Component"),
    )
}
