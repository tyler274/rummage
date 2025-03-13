use bevy::log::Level;
use bevy::prelude::*;
use std::panic;

// Thread-local to store the last panic message
thread_local! {
    static LAST_PANIC_MESSAGE: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

// Resource to track system execution and detect potential issues
#[derive(Resource, Default)]
pub struct SystemExecutionTracker {
    pub currently_running: Vec<String>,
    pub completed_systems: Vec<String>,
    pub failed_systems: Vec<String>,
    pub startup_complete: bool,
    pub frame_count: u64,
    pub last_panic_info: Option<String>,
}

// Plugin for tracing systems execution
pub struct SystemsTracePlugin;

impl Plugin for SystemsTracePlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing Systems Trace Plugin");

        // Register panic hook to capture system panics
        let previous_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            // Call the previous hook
            previous_hook(panic_info);

            // Format and log the panic information
            let panic_message = format!("{}", panic_info);
            error!("PANIC DETECTED: {}", panic_message);

            // Store in a global static or similar to retrieve later
            // This is a simple approach; in a real app you might use a channel or atomic
            LAST_PANIC_MESSAGE.with(|cell| {
                *cell.borrow_mut() = Some(panic_message);
            });
        }));

        // Add systems tracking resources
        app.insert_resource(SystemTraceConfig {
            log_system_init: true,
            log_system_start: true,
            log_system_finish: true,
            log_system_panic: true,
            trace_level: Level::DEBUG,
        })
        .init_resource::<SystemExecutionTracker>()
        .add_systems(Startup, log_startup_systems)
        .add_systems(PreUpdate, log_system_initialization)
        .add_systems(PreUpdate, log_frame_start.after(log_system_initialization))
        .add_systems(Update, detect_panics)
        .add_systems(Last, log_frame_end)
        .add_systems(Last, check_for_system_panics.after(log_frame_end))
        .add_systems(First, track_main_schedule_begin)
        .add_systems(Last, track_main_schedule_end);

        info!("Systems Trace Plugin initialized");
    }
}

// System to track the beginning of the Main schedule
fn track_main_schedule_begin() {
    debug!("Main schedule beginning");
}

// System to track the end of the Main schedule
fn track_main_schedule_end() {
    debug!("Main schedule ending");
}

// System to detect and handle panics
fn detect_panics(mut tracker: ResMut<SystemExecutionTracker>) {
    // Check if we have a new panic to process
    LAST_PANIC_MESSAGE.with(|cell| {
        if let Some(panic_msg) = cell.borrow_mut().take() {
            let frame_count = tracker.frame_count;
            error!("Processing panic from previous frame: {}", panic_msg);
            tracker.last_panic_info = Some(panic_msg);
            tracker
                .failed_systems
                .push(format!("unknown_system_panic_{}", frame_count));
        }
    });
}

// Configuration for the system tracing
#[derive(Resource)]
pub struct SystemTraceConfig {
    pub log_system_init: bool,
    pub log_system_start: bool,
    pub log_system_finish: bool,
    pub log_system_panic: bool,
    pub trace_level: Level,
}

// Log systems during startup phase
fn log_startup_systems(mut tracker: ResMut<SystemExecutionTracker>) {
    info!("=== STARTUP SYSTEMS RUNNING ===");
    debug!("Tracking startup system initialization");
    tracker.currently_running.push("startup".to_string());
}

// System to log system initialization during startup
fn log_system_initialization(world: &World, mut tracker: ResMut<SystemExecutionTracker>) {
    if !tracker.startup_complete {
        info!("=== STARTUP COMPLETE ===");
        tracker.startup_complete = true;

        // Log information about app state after startup
        let schedule_names = world
            .resource::<bevy::ecs::schedule::Schedules>()
            .iter()
            .map(|(id, _)| format!("{:?}", id))
            .collect::<Vec<_>>();

        info!("Registered schedules: {}", schedule_names.join(", "));
        debug!("System initialization check complete");
    }
}

// Log at the start of each frame
fn log_frame_start(mut tracker: ResMut<SystemExecutionTracker>) {
    tracker.frame_count += 1;
    tracker.currently_running.clear();
    tracker.completed_systems.clear();

    debug!("=== FRAME {} START ===", tracker.frame_count);
}

// Log at the end of each frame
fn log_frame_end(tracker: Res<SystemExecutionTracker>) {
    debug!("=== FRAME {} END ===", tracker.frame_count);
    if !tracker.failed_systems.is_empty() {
        warn!(
            "Failed systems in this frame: {}",
            tracker.failed_systems.join(", ")
        );
    }
}

// System to check for panics that have occurred
fn check_for_system_panics(mut tracker: ResMut<SystemExecutionTracker>) {
    if !tracker.failed_systems.is_empty() {
        error!(
            "Detected {} system failures this frame",
            tracker.failed_systems.len()
        );
        // Clear failures for next frame
        tracker.failed_systems.clear();
    }
}
