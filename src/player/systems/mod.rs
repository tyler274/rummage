//! Player systems module - aggregates all player related logic

// Keep spawn module public as it's used by game setup
pub mod spawn;

// Make debug module public for external use (e.g., CameraPlugin)
pub mod debug;

// Other player systems can remain private for now
// mod interactions; // Example: handle player clicks, etc.
// mod movement; // Example: if players could move independently
// ... add other player system modules here
