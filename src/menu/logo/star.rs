use crate::menu::state::GameMenuState;
use bevy::prelude::*;

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

            // Create the material once
            let material = materials.add(Color::srgb(1.0, 0.84, 0.0));

            // Spawn the child entities for the two triangles
            commands.entity(entity).with_children(|parent| {
                // First triangle (pointing up)
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.84, 0.0),
                        custom_size: Some(Vec2::new(80.0, 80.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 1.0)
                        .with_rotation(Quat::from_rotation_z(0.0)),
                    ..default()
                });

                // Second triangle (pointing down)
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.84, 0.0),
                        custom_size: Some(Vec2::new(80.0, 80.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 1.1)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                    ..default()
                });
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
        // Set the RenderTarget to a specific camera window
        Camera::from_target(Default::default()),
    )
}
