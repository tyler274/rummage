use crate::camera::components::GameCamera;
use crate::snapshot::components::{CameraSnapshot, SnapshotSettings};
use crate::snapshot::resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};
use crate::snapshot::systems::{
    handle_snapshot_events, snapshot_enabled, take_snapshot, take_snapshot_safe,
};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[test]
fn test_snapshot_enabled_condition() {
    // Test directly with constructed resources
    let disabled = SnapshotDisabled::disabled();
    let enabled = SnapshotDisabled::enabled();

    // Create a new App and systems to run with these resources
    let mut app = App::new();

    // Insert resources and run system
    app.insert_resource(disabled);

    // Add a system that tests the condition and share the result via Arc<Mutex>
    let result = Arc::new(Mutex::new(false));
    let result_clone = result.clone();

    app.add_systems(Update, move |res: Res<SnapshotDisabled>| {
        let is_enabled = snapshot_enabled(res);
        *result_clone.lock().unwrap() = is_enabled;
    });
    app.update();
    assert!(!*result.lock().unwrap(), "Snapshot should be disabled");

    // Change the resource and test again
    app.insert_resource(enabled);
    app.update();
    assert!(*result.lock().unwrap(), "Snapshot should be enabled");
}

#[test]
fn test_take_snapshot_safe() {
    // Set up a minimal world to test take_snapshot_safe
    let mut app = App::new();
    app.add_event::<SnapshotEvent>();

    // Create a test entity to use as a camera
    let test_entity = Entity::from_raw(42);
    let description_str = "test_snapshot".to_string();

    // Add a system that calls take_snapshot_safe
    app.add_systems(Update, move |mut events: EventWriter<SnapshotEvent>| {
        take_snapshot_safe(&mut events, test_entity, &description_str);
    });

    // Run the system
    app.update();

    // Create a result variable to track verification
    let verification_result = Arc::new(Mutex::new(false));
    let verification_result_clone = verification_result.clone();
    let expected_description = Some("test_snapshot".to_string());

    // Add a system to verify the event
    app.add_systems(Update, move |mut events: EventReader<SnapshotEvent>| {
        let events_received: Vec<_> = events.read().collect();

        if !events_received.is_empty() {
            assert_eq!(events_received.len(), 1);
            let event = &events_received[0];
            assert_eq!(event.camera_entity, Some(test_entity));
            assert_eq!(event.description, expected_description);
            assert_eq!(event.include_debug, Some(true));

            *verification_result_clone.lock().unwrap() = true;
        }
    });

    // Run the verification system
    app.update();

    assert!(
        *verification_result.lock().unwrap(),
        "Events were not verified during test"
    );
}

#[test]
fn test_take_snapshot() {
    // Set up a minimal world to test take_snapshot
    let mut app = App::new();
    app.add_event::<SnapshotEvent>();

    // Create a test entity to use as a camera
    let test_entity = Entity::from_raw(42);
    let description_str = "test_description".to_string();
    let description = Some(description_str.clone());

    // Add a system that calls take_snapshot
    app.add_systems(Update, move |mut events: EventWriter<SnapshotEvent>| {
        take_snapshot(&mut events, Some(test_entity), description.clone());
    });

    // Run the system
    app.update();

    // Create a result variable to track verification
    let verification_result = Arc::new(Mutex::new(false));
    let verification_result_clone = verification_result.clone();
    let expected_description = Some("test_description".to_string());

    // Add a system to verify the event
    app.add_systems(Update, move |mut events: EventReader<SnapshotEvent>| {
        let events_received: Vec<_> = events.read().collect();

        if !events_received.is_empty() {
            assert_eq!(events_received.len(), 1);
            let event = &events_received[0];
            assert_eq!(event.camera_entity, Some(test_entity));
            assert_eq!(event.description, expected_description);
            assert_eq!(event.include_debug, Some(true));

            *verification_result_clone.lock().unwrap() = true;
        }
    });

    // Run the verification system
    app.update();

    assert!(
        *verification_result.lock().unwrap(),
        "Events were not verified during test"
    );
}

#[test]
fn test_handle_snapshot_events() {
    // Create a test app
    let mut app = App::new();

    // Add required resources
    app.init_resource::<SnapshotConfig>()
        .init_resource::<Time>()
        .add_event::<SnapshotEvent>()
        .init_resource::<crate::snapshot::resources::SnapshotDebugState>();

    // Create a camera entity
    let camera_entity = app.world_mut().spawn((GameCamera,)).id();

    // Add the system we want to test
    app.add_systems(Update, handle_snapshot_events);

    // Add an event to be processed
    let mut events = app.world_mut().resource_mut::<Events<SnapshotEvent>>();
    events.send(SnapshotEvent::new().with_camera(camera_entity));

    // Run an update to process the system and event
    // Run a test update to process the event
    app.update();

    // Add the system we want to test
    app.add_systems(Update, handle_snapshot_events);

    // Run another update to process the system
    app.update();

    // Verify the camera has the snapshot components
    let camera = app.world().entity(camera_entity);
    assert!(camera.contains::<CameraSnapshot>());
    assert!(camera.contains::<SnapshotSettings>());
}
