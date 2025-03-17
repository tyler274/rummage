use crate::cards::types::CardTypes;
use crate::cards::{Card, CardDetails};
use crate::mana::Mana;
use crate::text::components::CardNameText;
use bevy::prelude::*;

/// Test that demonstrates how to use the process_name_text_components function
#[test]
fn test_name_text_components() {
    // Setup test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Create a card with name component
    let card_entity = app
        .world_mut()
        .spawn((
            Card::builder("Test Card")
                .cost(Mana::default())
                .types(CardTypes::default())
                .details(CardDetails::default())
                .build_or_panic(),
            Sprite {
                custom_size: Some(Vec2::new(100.0, 140.0)),
                ..default()
            },
            Transform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    // Add name text component as child
    let text_entity = app
        .world_mut()
        .spawn(CardNameText {
            name: "Test Card".to_string(),
        })
        .id();

    // Add the child relationship
    app.world_mut()
        .entity_mut(card_entity)
        .add_child(text_entity);

    // Run a simple test system
    fn test_system(card_query: Query<Entity, With<Card>>, children_query: Query<&Children>) {
        for entity in card_query.iter() {
            if let Ok(children) = children_query.get(entity) {
                assert!(!children.is_empty());
            }
        }
    }

    app.add_systems(Update, test_system);
    app.update();
}
