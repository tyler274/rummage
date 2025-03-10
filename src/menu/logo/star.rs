use crate::camera::components::AppLayer;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;

/// Component for the Star of David shape
#[derive(Component)]
pub struct StarOfDavid;

/// Plugin that renders the Star of David
pub struct StarOfDavidPlugin;

impl Plugin for StarOfDavidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
) {
    info!("StarOfDavid entities found: {}", query.iter().count());

    for entity in &query {
        let has_children = children_query
            .get(entity)
            .map(|children| !children.is_empty())
            .unwrap_or(false);

        info!(
            "StarOfDavid entity {:?} has children: {}",
            entity, has_children
        );

        // Only spawn children if it doesn't have children yet
        if !has_children {
            info!("Adding children to StarOfDavid entity {:?}", entity);

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
                    Transform::from_xyz(0.0, 0.0, -10.0),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ));

                // Second triangle (pointing down)
                parent.spawn((
                    Mesh2d::from(triangle_mesh),
                    MeshMaterial2d(material),
                    Transform::from_xyz(0.0, 0.0, -10.0)
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
    info!("Creating StarOfDavid bundle");
    (
        Transform::from_xyz(0.0, 0.0, -20.0),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        StarOfDavid,
        AppLayer::Menu.layer(), // Only visible on menu layer
    )
}
