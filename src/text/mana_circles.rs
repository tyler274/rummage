use bevy::prelude::*;

/// Component to mark a sprite that is meant to be a mana circle
#[derive(Component, Debug)]
pub struct ManaCircle {
    // The radius field is being removed since it's unused
    // We can add it back if needed later with proper usage
}

/// System to update the size and scale of mana circles to ensure they appear round
pub fn update_mana_circles(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Sprite, &Name), Without<ManaCircle>>,
) {
    for (entity, _transform, sprite, name) in query.iter() {
        // Only process sprites with "Mana Circle" in their name
        if !name.as_str().contains("Mana Circle") {
            continue;
        }

        if let Some(size) = sprite.custom_size {
            // Add the ManaCircle component as a marker
            commands.entity(entity).insert(ManaCircle {});

            // Make the sprite slightly larger to compensate for the circular appearance
            // This helps make the circle fill the same space as the text
            let adjusted_size = Vec2::new(size.x * 1.1, size.x * 1.1);

            // Update the sprite to be a perfect square (for circular appearance)
            commands.entity(entity).insert(Sprite {
                color: sprite.color,
                custom_size: Some(adjusted_size),
                ..default()
            });
        }
    }
}
