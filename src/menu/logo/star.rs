use crate::camera::components::AppLayer;
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

        // Use ImageNode to display the star image
        commands.entity(entity).with_children(|parent| {
            // Create a UI image node for the star
            parent.spawn((
                ImageNode::new(asset_server.load("textures/star.png")),
                Node {
                    width: Val::Px(120.0),
                    height: Val::Px(120.0),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                GlobalZIndex(100), // Add high z-index to ensure visibility
                Name::new("Star Image"),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));
        });
    }
}

/// Create a Star of David bundle for spawning
pub fn create_star_of_david() -> impl Bundle {
    info!("Creating StarOfDavid bundle as UI component");
    (
        // Use UI-oriented components
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
        BackgroundColor(Color::NONE), // Transparent background
        StarOfDavid,
        AppLayer::Menu.layer(), // Only visible on menu layer
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    )
}
