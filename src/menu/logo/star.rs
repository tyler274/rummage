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
    windows: Query<&Window>,
) {
    // Get window dimensions for proper positioning
    let window_width = windows.iter().next().map(|w| w.width()).unwrap_or(1920.0);
    let window_height = windows.iter().next().map(|w| w.height()).unwrap_or(1080.0);

    // Only process entities that don't have children yet
    for entity in &query {
        // Create the material once - brighter gold color with higher saturation
        let material = materials.add(Color::srgb(1.0, 1.0, 0.0));

        // Log the star creation with window dimensions
        info!(
            "Rendering Star of David for entity {:?} at z-position 100.0, window: {}x{}",
            entity, window_width, window_height
        );

        // Create a triangle mesh - increased size for better visibility
        let triangle_mesh = meshes.add(create_equilateral_triangle_mesh(250.0 * 2.0));

        // Z-coordinate ensures it's visible above the background but behind the text
        let z_position = 100.0; // Dramatically increased for visibility

        // Triangle offset to create the star shape
        let triangle_offset = 60.0; // Increased offset for better shape

        // Update position based on window dimensions
        commands.entity(entity).insert(Transform::from_xyz(
            0.0,                  // Centered horizontally
            window_height * 0.15, // Positioned near the top (15% from top)
            z_position,
        ));

        // Spawn the child entities for the two triangles
        commands.entity(entity).with_children(|parent| {
            // First triangle (pointing up)
            parent.spawn((
                Mesh2d::from(triangle_mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(0.0, triangle_offset, z_position),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                GlobalZIndex(50), // Add specific z-index to the triangle itself
            ));

            // Second triangle (pointing down)
            parent.spawn((
                Mesh2d::from(triangle_mesh),
                MeshMaterial2d(material),
                Transform::from_xyz(0.0, -triangle_offset, z_position)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                GlobalZIndex(50), // Add specific z-index to the triangle itself
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
    // Use info level for better visibility in logs
    info!("Creating StarOfDavid bundle with GlobalZIndex(50)");
    (
        // Use a UI-oriented position that works with the menu camera
        // Z-index places it behind UI elements but still visible
        Transform::from_xyz(0.0, 200.0, 100.0), // Position in center top area of screen with high z value
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        StarOfDavid,
        AppLayer::Menu.layer(), // Only visible on menu layer
        // Add a GlobalZIndex to ensure proper z-ordering with other UI elements
        GlobalZIndex(50), // Dramatically increased from 10 to 50
    )
}
