# WebAssembly Local Storage Support

The save/load system is designed to work seamlessly in both native desktop applications and WebAssembly environments running in a web browser.

## How It Works

When your game runs in a web browser via WebAssembly, there is no traditional filesystem available. Instead, the save/load system automatically switches to using the browser's Local Storage API. This is done transparently, so your game code doesn't need to handle platform differences.

## Path Handling

Path handling is abstracted to work across different platforms:

- **Native Desktop:** Saves are stored in the `saves/` directory relative to the game executable
- **WebAssembly:** Saves are stored in the browser's local storage with keys prefixed with `saves/`

This is implemented using bevy-persistent's storage backend system, where paths starting with `/local/` are mapped to browser local storage.

## Storage Limitations

When running in WebAssembly, be aware of these limitations:

1. **Storage Size:** Browsers typically limit local storage to 5-10MB total. This means all your saves combined should stay under this limit.
   
2. **Persistence:** Unlike files on a desktop computer, local storage can be cleared if:
   - The user clears their browser data/cache
   - The browser decides to free up space
   - The user is in private/incognito mode (storage will be temporary)

3. **Serialization Format:** The system uses bincode serialization for efficiency, which works well for both platforms but produces binary data. In the browser, this binary data is base64-encoded to be stored in local storage.

## Debugging WebAssembly Storage

To inspect saved game data in a browser:

1. Open your browser's developer tools (F12 or right-click > Inspect)
2. Go to the "Application" tab (Chrome) or "Storage" tab (Firefox)
3. Look for "Local Storage" in the left panel
4. Select your site's domain
5. Look for entries with keys starting with `saves/`

## Implementation Details

The path handling is implemented through the `get_storage_path` helper function, which conditionally compiles different paths based on the target platform:

```rust
fn get_storage_path(filename: &str) -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        // For WebAssembly, use local storage with a prefix
        Path::new("/local/saves").join(filename)
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native platforms, use the filesystem
        Path::new("saves").join(filename)
    }
}
```

This ensures that save files are automatically directed to the appropriate storage system based on the platform. 