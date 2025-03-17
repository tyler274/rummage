use crate::camera::components::AppLayer;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;

/// Component for the Star of David shape
#[derive(Component)]
pub struct StarOfDavid;

/// Resource to control logging frequency and track entity state
#[derive(Resource)]
pub struct StarOfDavidLogState {
    /// When we last logged star debug info
    last_log_time: f32,
    /// Minimum time between logs in seconds
    log_interval: f32,
    /// Last recorded number of stars
    last_star_count: usize,
    /// Last recorded state of stars having children
    last_children_state: std::collections::HashMap<Entity, bool>,
    /// Whether any changes were made to stars since last log
    changes_made: bool,
}

impl Default for StarOfDavidLogState {
    fn default() -> Self {
        Self {
            last_log_time: 0.0,
            log_interval: 30.0, // Only log once every 30 seconds at most
            last_star_count: 0,
            last_children_state: std::collections::HashMap::new(),
            changes_made: false,
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
    // Check for entities with changed Children component
    let changed_entities = query.iter().count();

    // Only process if changes or if star count has changed
    let current_star_count = star_count_query.iter().count();
    let count_changed = current_star_count != log_state.last_star_count;

    if changed_entities > 0 || count_changed {
        let mut current_children_state = std::collections::HashMap::new();
        let mut children_changed = false;

        // Check all entities to build a complete children state map
        for entity in star_count_query.iter() {
            let has_children = children_query
                .get(entity)
                .map(|children| !children.is_empty())
                .unwrap_or(false);

            // Record current state
            current_children_state.insert(entity, has_children);

            // Check if this entity's state has changed
            if log_state.last_children_state.get(&entity) != Some(&has_children) {
                children_changed = true;
            }
        }

        // Log changes if detected
        if count_changed || children_changed {
            debug!("StarOfDavid entities found: {}", current_star_count);

            // Only log individual entities if there was a change
            if count_changed || children_changed {
                for (entity, has_children) in &current_children_state {
                    debug!(
                        "StarOfDavid entity {:?} has children: {}",
                        entity, has_children
                    );
                }
            }

            // Update the log state
            log_state.last_star_count = current_star_count;
            log_state.last_children_state = current_children_state;
        }
    }
}

/// System to create and render the Star of David
pub fn render_star_of_david(
    mut commands: Commands,
    query: Query<Entity, (With<StarOfDavid>, Without<Children>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Only process entities that don't have children yet
    for entity in &query {
        // Create the material once - gold color
        let material = materials.add(Color::srgb(1.0, 0.84, 0.0));

        // Create a triangle mesh
        let triangle_mesh = meshes.add(create_equilateral_triangle_mesh(150.0));

        // Spawn the child entities for the two triangles
        commands.entity(entity).with_children(|parent| {
            // First triangle (pointing up)
            parent.spawn((
                Mesh2d::from(triangle_mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(0.0, 20.0, 901.0),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));

            // Second triangle (pointing down)
            parent.spawn((
                Mesh2d::from(triangle_mesh),
                MeshMaterial2d(material),
                Transform::from_xyz(0.0, -20.0, 901.0)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));
        });
    }
}

/// Create an equilateral triangle mesh
fn create_equilateral_triangle_mesh(size: f32) -> Mesh {
    // Calculate vertices for an equilateral triangle
    let half_size = size / 2.0;
    let height = size * 0.866; // sqrt(3)/2 * size

    let vertices = vec![
        [0.0, height / 2.0, 0.0],         // Top
        [-half_size, -height / 2.0, 0.0], // Bottom left
        [half_size, -height / 2.0, 0.0],  // Bottom right
    ];

    let indices = vec![0, 1, 2];
    let normals = vec![[0.0, 0.0, 1.0]; 3];
    let uvs = vec![[0.5, 0.0], [0.0, 1.0], [1.0, 1.0]];

    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    mesh
}

/// Create a Star of David bundle for spawning
pub fn create_star_of_david() -> impl Bundle {
    // Use debug level to reduce log spam
    debug!("Creating StarOfDavid bundle");
    (
        // Position behind UI but still within camera view
        Transform::from_xyz(0.0, 0.0, 900.0),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        StarOfDavid,
        AppLayer::Menu.layer(), // Only visible on menu layer
    )
}
