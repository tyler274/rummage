use crate::game_engine::visual_testing::config::VisualTestConfig;
use crate::game_engine::visual_testing::{
    capture_entity_rendering, compare_images, load_reference_image, save_difference_visualization,
    setup_animation_keyframe, setup_animation_test, setup_card_state, setup_test_scene,
    setup_ui_state, setup_ui_test_scene,
};
use bevy::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_engine::tests::setup_visual_test_environment;
    use crate::game_engine::visual_testing::{ANIMATION_TESTS, CARD_TEST_STATES, UI_TEST_STATES};

    /// Example test for card rendering consistency
    #[test]
    fn test_card_rendering_consistency() {
        // Set up the test app
        let mut app = App::new();
        setup_visual_test_environment(&mut app);
        app.add_systems(Startup, setup_test_scene);

        // Create a test entity that will be our card
        let card_entity = app.world.spawn_empty().id();

        // Process the initial setup
        app.update();

        // Test each card state
        for state in CARD_TEST_STATES {
            // 1. Set up the card in the appropriate state
            setup_card_state(&mut app, state);
            app.update();

            // 2. Take a screenshot
            let screenshot = capture_entity_rendering(&app, card_entity);

            // 3. Generate reference or compare to reference
            if app.world.resource::<VisualTestConfig>().update_references {
                // Generate reference image
                if let Err(e) = crate::game_engine::visual_testing::utils::save_reference_image(
                    screenshot,
                    &format!("{}.png", state),
                ) {
                    panic!("Failed to save reference image: {}", e);
                }
            } else {
                // Compare to reference
                match load_reference_image(&format!("{}.png", state)) {
                    Ok(reference) => {
                        let result = compare_images(&screenshot, &reference);

                        // Save difference visualization if similarity is below threshold
                        if result.similarity_score < 0.99 {
                            let _ = save_difference_visualization(
                                &screenshot,
                                &reference,
                                &format!("{}_diff.png", state),
                            );
                        }

                        assert!(
                            result.similarity_score >= 0.99,
                            "Card rendering for state '{}' differs from reference. Similarity: {}",
                            state,
                            result.similarity_score
                        );
                    }
                    Err(e) => panic!("Failed to load reference image: {}", e),
                }
            }
        }
    }

    /// Example test for UI element rendering
    #[test]
    fn test_ui_element_rendering() {
        // Set up the test app
        let mut app = App::new();
        setup_visual_test_environment(&mut app);
        app.add_systems(Startup, setup_ui_test_scene);

        // Process the initial setup
        app.update();

        // Test each UI state
        for state in UI_TEST_STATES {
            // Set up the UI in the appropriate state
            setup_ui_state(&mut app, state);
            app.update();

            // Take a screenshot
            if let Some(screenshot) =
                crate::game_engine::visual_testing::capture::take_screenshot(&app)
            {
                let reference_name = format!("ui_{}.png", state);

                if app.world.resource::<VisualTestConfig>().update_references {
                    if let Err(e) = crate::game_engine::visual_testing::utils::save_reference_image(
                        screenshot,
                        &reference_name,
                    ) {
                        panic!("Failed to save UI reference image: {}", e);
                    }
                } else {
                    match load_reference_image(&reference_name) {
                        Ok(reference) => {
                            let result = compare_images(&screenshot, &reference);

                            if result.similarity_score < 0.99 {
                                let _ = save_difference_visualization(
                                    &screenshot,
                                    &reference,
                                    &format!("ui_{}_diff.png", state),
                                );
                            }

                            assert!(
                                result.similarity_score >= 0.99,
                                "UI rendering for state '{}' differs from reference. Similarity: {}",
                                state,
                                result.similarity_score
                            );
                        }
                        Err(e) => panic!("Failed to load UI reference image: {}", e),
                    }
                }
            } else {
                panic!("Failed to take screenshot for UI state '{}'", state);
            }
        }
    }

    /// Example test for animation consistency at keyframes
    #[test]
    fn test_animation_keyframes() {
        // Set up the test app
        let mut app = App::new();
        setup_visual_test_environment(&mut app);
        app.add_systems(Startup, setup_animation_test);

        // Process the initial setup
        app.update();

        // Test each animation at each keyframe
        for (animation, keyframe) in ANIMATION_TESTS {
            // Set up the animation at the specified keyframe
            setup_animation_keyframe(&mut app, animation, *keyframe);
            app.update();

            // Take a screenshot
            if let Some(screenshot) =
                crate::game_engine::visual_testing::capture::take_screenshot(&app)
            {
                let reference_name = format!("anim_{}_{}.png", animation, keyframe);

                if app.world.resource::<VisualTestConfig>().update_references {
                    if let Err(e) = crate::game_engine::visual_testing::utils::save_reference_image(
                        screenshot,
                        &reference_name,
                    ) {
                        panic!("Failed to save animation reference image: {}", e);
                    }
                } else {
                    match load_reference_image(&reference_name) {
                        Ok(reference) => {
                            let result = compare_images(&screenshot, &reference);

                            if result.similarity_score < 0.98 {
                                // Slightly lower threshold for animations
                                let _ = save_difference_visualization(
                                    &screenshot,
                                    &reference,
                                    &format!("anim_{}_{}_diff.png", animation, keyframe),
                                );
                            }

                            assert!(
                                result.similarity_score >= 0.98,
                                "Animation '{}' at keyframe {} differs from reference. Similarity: {}",
                                animation,
                                keyframe,
                                result.similarity_score
                            );
                        }
                        Err(e) => panic!("Failed to load animation reference image: {}", e),
                    }
                }
            } else {
                panic!(
                    "Failed to take screenshot for animation '{}' at keyframe {}",
                    animation, keyframe
                );
            }
        }
    }

    /// Example test for generating reference images
    #[test]
    fn test_generate_reference_images() {
        // Only run this test when explicitly enabled
        // This is typically used to generate initial reference images
        // or update them after intentional visual changes
        if std::env::var("GENERATE_REFERENCE_IMAGES").is_err() {
            return;
        }

        // Set up the test app
        let mut app = App::new();
        setup_visual_test_environment(&mut app);

        // Set update_references to true
        if let Some(mut config) = app.world.get_resource_mut::<VisualTestConfig>() {
            config.update_references = true;
        } else {
            app.insert_resource(VisualTestConfig {
                update_references: true,
                ..Default::default()
            });
        }

        // Generate card reference images
        app.add_systems(Startup, setup_test_scene);
        app.update();
        for state in CARD_TEST_STATES {
            setup_card_state(&mut app, state);
            app.update();

            let screenshot =
                crate::game_engine::visual_testing::capture::take_screenshot(&app).unwrap();
            let _ = crate::game_engine::visual_testing::utils::save_reference_image(
                screenshot,
                &format!("{}.png", state),
            );
        }

        // Generate UI reference images
        app = App::new();
        setup_visual_test_environment(&mut app);
        app.insert_resource(VisualTestConfig {
            update_references: true,
            ..Default::default()
        });
        app.add_systems(Startup, setup_ui_test_scene);
        app.update();
        for state in UI_TEST_STATES {
            setup_ui_state(&mut app, state);
            app.update();

            let screenshot =
                crate::game_engine::visual_testing::capture::take_screenshot(&app).unwrap();
            let _ = crate::game_engine::visual_testing::utils::save_reference_image(
                screenshot,
                &format!("ui_{}.png", state),
            );
        }

        // Generate animation reference images
        app = App::new();
        setup_visual_test_environment(&mut app);
        app.insert_resource(VisualTestConfig {
            update_references: true,
            ..Default::default()
        });
        app.add_systems(Startup, setup_animation_test);
        app.update();
        for (animation, keyframe) in ANIMATION_TESTS {
            setup_animation_keyframe(&mut app, animation, *keyframe);
            app.update();

            let screenshot =
                crate::game_engine::visual_testing::capture::take_screenshot(&app).unwrap();
            let _ = crate::game_engine::visual_testing::utils::save_reference_image(
                screenshot,
                &format!("anim_{}_{}.png", animation, keyframe),
            );
        }
    }
}
