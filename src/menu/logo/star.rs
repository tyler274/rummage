use bevy::prelude::*;

/// Component for the Star of David shape
#[derive(Component)]
pub struct StarOfDavid;

/// Plugin that renders the Star of David
pub struct StarOfDavidPlugin;

impl Plugin for StarOfDavidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_star_of_david);
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
    asset_server: Res<AssetServer>,
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

            // Spawn the child entities for the two triangles
            commands.entity(entity).with_children(|parent| {
                // First triangle (pointing up)
                let triangle1 = parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.84, 0.0),
                        custom_size: Some(Vec2::new(80.0, 80.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                });
                info!("Spawned first triangle: {:?}", triangle1.id());

                // Second triangle (pointing down)
                let triangle2 = parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.84, 0.0),
                        custom_size: Some(Vec2::new(80.0, 80.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 1.1)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                    ..default()
                });
                info!("Spawned second triangle: {:?}", triangle2.id());
            });
        }
    }
}

/// Create a Star of David bundle for spawning
pub fn create_star_of_david() -> impl Bundle {
    info!("Creating StarOfDavid bundle");
    (
        Transform::from_xyz(0.0, 0.0, 1.0),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        StarOfDavid,
    )
}
