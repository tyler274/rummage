# Bevy 0.16 Upgrade Notes

This document summarizes the compiler errors and warnings encountered after upgrading to Bevy 0.16, based on the provided `cargo check` output. This is intended to guide the process of fixing the breaking changes and deprecated APIs.

## Deprecated APIs

### `EventWriter::send`

The `send` method on `bevy::prelude::EventWriter` is deprecated.

**Fix:** Replace `.send(event)` with `.write(event)`.

**Occurrences:**
- `src/snapshot/systems.rs:58`
- `src/snapshot/systems.rs:198`
- `src/snapshot/systems.rs:397`
- `src/snapshot/systems.rs:493`
- `src/snapshot/systems.rs:586`
- `src/tests/visual_testing/capture.rs:102`

### `Query::get_single`

The `get_single` method on `bevy::prelude::Query` is deprecated.

**Fix:** Replace `.get_single()` with `.single()`.

**Occurrences:**
- `src/tests/visual_testing/capture.rs:34`
- `src/tests/visual_testing/diff.rs:106`

## Compilation Errors (potentially related to System API changes)

The compiler reported 158 errors, with detailed explanations available for E0277, E0308, E0412, E0423, E0599, E0603, and E0609. The snippet shows a complex type mismatch error (E0308) involving system functions (`set_initial_zoom`, `handle_window_resize`, `camera_movement`) and traits like `CurveExt` and `Iterator`.

**Analysis:** This strongly suggests changes in the Bevy ECS system registration or parameter handling in Bevy 0.16. The way these specific systems are defined or added to the application's schedule needs to be reviewed and updated according to the new Bevy 0.16 API. This might involve changes to system signatures, the use of system sets, or how system tuples are handled.

**Fix:** Examine the definitions and registration of the systems `set_initial_zoom`, `handle_window_resize`, and `camera_movement`. Consult the Bevy 0.16 migration guide and documentation for the correct way to define and add systems, especially those accessing multiple resources and queries or used in complex compositions.

This document covers the issues identified from the provided compiler output. There might be other errors not detailed in the snippet, which would also need investigation. 