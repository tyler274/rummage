use crate::camera::components::AppLayer;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use std::time::Duration;

/// Component for the Star of David shape
#[derive(Component)]
pub struct StarOfDavid;

/// Resource to control logging frequency and track entity state
#[derive(Resource)]
pub struct StarOfDavidLogState {
    /// When we last logged star debug info
    last_log_time: f64,
    /// Minimum time between logs in seconds
    log_interval: f64,
    /// Last recorded number of stars
    last_star_count: usize,
    /// Whether any changes were made to stars since last log
    changes_made: bool,
}

impl Default for StarOfDavidLogState {
    fn default() -> Self {
        Self {
            last_log_time: 0.0,
            log_interval: 5.0, // Only log once every 5 seconds
            last_star_count: 0,
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

    // Check if we need to log based on time interval or state changes
    let current_time = time.elapsed_seconds();
    let should_log = current_time - log_state.last_log_time > log_state.log_interval
        || star_count != log_state.last_star_count
        || log_state.changes_made;

    // If we're going to log, reset the state tracking
    if should_log {
        log_state.last_log_time = current_time;
        log_state.last_star_count = star_count;
        log_state.changes_made = false;

        debug!("StarOfDavid entities found: {}", star_count);
    }

    // Process stars and create any missing children
    for entity in &query {
        let has_children = children_query
            .get(entity)
            .map(|children| !children.is_empty())
            .unwrap_or(false);

        // Only log detailed entity info when we're already logging
        if should_log {
            debug!(
                "StarOfDavid entity {:?} has children: {}",
                entity, has_children
            );
        }

        // Only spawn children if it doesn't have children yet
        if !has_children {
            // Track that we made changes for next frame's logging
            log_state.changes_made = true;

            if should_log {
                debug!("Adding children to StarOfDavid entity {:?}", entity);
            }

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
