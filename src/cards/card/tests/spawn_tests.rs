use crate::cards::card::Card;
use crate::cards::details::CardDetails;
use crate::cards::types::CardTypes;
// Mana is used in the test, but let's import it directly from the test function
// where it's needed
use bevy::prelude::App;
use bevy::prelude::Commands;
use bevy::prelude::MinimalPlugins;
use bevy::prelude::Update;

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
            Mana::new(1, 0, 0, 0, 0, 0),
            CardTypes::new_creature(vec!["Wizard".to_string()]),
            CardDetails::new_creature(2, 2),
            "Flying",
        );
    }

    // Run the system to spawn the card
    app.add_systems(Update, spawn_test_card);
    app.update();

    // Check that the entity was created with Card component
    let card_exists = app.world.query::<&Card>().iter(&app.world).count() > 0;
    assert!(card_exists);
}
