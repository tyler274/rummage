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
        app.init_resource::<StarOfDavidLogState>().add_systems(
            Update,
            render_star_of_david.run_if(|state: Res<State<GameMenuState>>| {
                matches!(
                    state.get(),
                    GameMenuState::MainMenu | GameMenuState::PausedGame
                )
            }),
        );
    }

    fn finish(&self, _app: &mut App) {
        // No custom pipeline setup needed
    }
}

/// System to create and render the Star of David
pub fn render_star_of_david(
    mut commands: Commands,
    query: Query<Entity, With<StarOfDavid>>,
    children_query: Query<&Children>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut log_state: ResMut<StarOfDavidLogState>,
) {
    // Count stars
    let star_count = query.iter().count();

    // Check for changes in entity count
    let count_changed = star_count != log_state.last_star_count;

    // Track changes in children states
    let mut children_changed = false;
    let mut current_children_state = std::collections::HashMap::new();

    for entity in &query {
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

        // Only spawn children if it doesn't have children yet
        if !has_children {
            // Track that we made changes for next frame's logging
            log_state.changes_made = true;

            // Create the material once - gold color
            let material = materials.add(Color::srgb(1.0, 0.84, 0.0));

            // Create a triangle mesh
            let triangle_mesh = meshes.add(create_equilateral_triangle_mesh(75.0));

            // Spawn the child entities for the two triangles
            commands.entity(entity).with_children(|parent| {
                // First triangle (pointing up)
                parent.spawn((
                    Mesh2d::from(triangle_mesh.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(0.0, 0.0, 901.0),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ));

                // Second triangle (pointing down)
                parent.spawn((
                    Mesh2d::from(triangle_mesh),
                    MeshMaterial2d(material),
                    Transform::from_xyz(0.0, 0.0, 901.0)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ));
            });
        }
    }

    // Determine if we should log based on changes or forced time interval
    let current_time = time.elapsed_secs();
    let force_log_by_time = current_time - log_state.last_log_time > log_state.log_interval;
    let should_log =
        force_log_by_time || count_changed || children_changed || log_state.changes_made;

    // Only log if we have changes or it's been a long time
    if should_log {
        log_state.last_log_time = current_time;
        log_state.last_star_count = star_count;
        log_state.last_children_state = current_children_state;
        log_state.changes_made = false;

        debug!("StarOfDavid entities found: {}", star_count);

        // Only log individual entities if there was a change
        if count_changed || children_changed {
            for (entity, has_children) in &log_state.last_children_state {
                debug!(
                    "StarOfDavid entity {:?} has children: {}",
                    entity, has_children
                );
            }
        }
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
