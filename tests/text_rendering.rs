#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::text::Font;
    use rummage::card::{Card, CardDetails, CardTextContent, CardTextType, CardTypes, DebugConfig};
    use rummage::mana::{Color as ManaColor, Mana};
    use rummage::text::spawn_card_text;

    /// Helper function to set up a test environment with necessary resources
    fn setup_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .insert_resource(DebugConfig::default())
            .init_asset::<Font>(); // Initialize Font asset type

        app.add_systems(Update, spawn_card_text);
        app.update(); // Run startup systems
        app
    }

    #[test]
    fn test_mana_cost_visibility() {
        let mut app = setup_test_app();

        // Get the AssetServer before spawning entities
        let font_handle = {
            let asset_server = app.world().resource::<AssetServer>();
            asset_server.load("fonts/mana.ttf")
        };

        // Create a test card with mana cost {W}{2}
        let card = Card {
            name: "Test Card".to_string(),
            cost: Mana {
                color: ManaColor::WHITE | ManaColor::COLORLESS,
                white: 1,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                colorless: 2,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
            rules_text: String::new(),
        };

        // Spawn the card entity
        let card_entity = app
            .world_mut()
            .spawn((
                card.clone(),
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(100.0, 140.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        // Create a test entity with mana cost text
        let text_entity = app
            .world_mut()
            .spawn((
                Text2d::new("{W}{2}"),
                TextFont {
                    font: font_handle,
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                CardTextContent {
                    text: "{W}{2}".to_string(),
                    text_type: CardTextType::Cost,
                },
            ))
            .id();

        // Set up parent-child relationship
        app.world_mut()
            .entity_mut(text_entity)
            .set_parent(card_entity);

        // Run the spawn_card_text system
        app.update();

        // Verify the text component
        let text = app.world().get::<Text2d>(text_entity).unwrap();
        assert_eq!(text.0, "{W}{2}");
        let font = app.world().get::<TextFont>(text_entity).unwrap();
        assert_eq!(font.font_size, 24.0);
    }

    #[test]
    fn test_mana_cost_in_rules_text() {
        let mut app = setup_test_app();

        // Get the AssetServer before spawning entities
        let font_handle = {
            let asset_server = app.world().resource::<AssetServer>();
            asset_server.load("fonts/mana.ttf")
        };

        // Create a test card with rules text
        let card = Card {
            name: "Test Card".to_string(),
            cost: Mana::default(),
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
            rules_text: "Pay {W} to gain 2 life".to_string(),
        };

        // Spawn the card entity
        let card_entity = app
            .world_mut()
            .spawn((
                card.clone(),
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(100.0, 140.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        // Create a test entity with rules text containing mana symbols
        let text_entity = app
            .world_mut()
            .spawn((
                Text2d::new("Pay {W} to gain 2 life"),
                TextFont {
                    font: font_handle,
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                TextLayout::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                CardTextContent {
                    text: "Pay {W} to gain 2 life".to_string(),
                    text_type: CardTextType::RulesText,
                },
            ))
            .id();

        // Set up parent-child relationship
        app.world_mut()
            .entity_mut(text_entity)
            .set_parent(card_entity);

        // Run the spawn_card_text system
        app.update();

        // Verify the text component
        let text = app.world().get::<Text2d>(text_entity).unwrap();
        assert_eq!(text.0, "Pay {W} to gain 2 life");
        let font = app.world().get::<TextFont>(text_entity).unwrap();
        assert_eq!(font.font_size, 18.0);
    }
}
