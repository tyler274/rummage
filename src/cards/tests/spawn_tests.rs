use crate::cards::card::Card;
use crate::cards::details::CardDetails;
use crate::cards::types::CardTypes;
use crate::mana::Mana;
use bevy::prelude::*;

/// Test demonstrating the spawn method
#[test]
fn test_card_spawn() {
    // Create a new app for testing
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // System to spawn a test card
    fn spawn_test_card(mut commands: Commands) {
        let _card_entity = Card::spawn(
            &mut commands,
            "Test Card",
            Mana::new_with_colors(1, 0, 0, 0, 0, 0),
            CardTypes::new_creature(vec!["Wizard".to_string()]),
            CardDetails::new_creature(2, 2),
            "Flying",
        );
    }

    // Run the system to spawn the card
    app.add_systems(Update, spawn_test_card);
    app.update();

    // Check that the entity was created with Card component
    // Use a system to check if cards exist
    #[derive(Resource)]
    struct CardCheckState {
        has_cards: bool,
    }

    let state = CardCheckState { has_cards: false };
    app.insert_resource(state);

    app.add_systems(
        Update,
        |query: Query<&Card>, mut state: ResMut<CardCheckState>| {
            state.has_cards = !query.is_empty();
        },
    );
    app.update();

    let state = app.world().resource::<CardCheckState>();
    assert!(state.has_cards);
}
