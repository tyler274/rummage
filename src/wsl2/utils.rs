use std::fs::File;
use std::io::Read;

/// Detect if the application is running under WSL2
///
/// This checks for the presence of WSL-specific files and environment variables
pub fn detect_wsl2() -> bool {
    // Check for /proc/sys/kernel/osrelease containing "microsoft" or "WSL"
    if let Ok(mut file) = File::open("/proc/sys/kernel/osrelease") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let contents = contents.to_lowercase();
            if contents.contains("microsoft") || contents.contains("wsl") {
                return true;
            }
        }
    }

    // Check for WSL-specific environment variables
    if let Ok(wsl_env) = std::env::var("WSL_DISTRO_NAME") {
        if !wsl_env.is_empty() {
            return true;
        }
    }

    // Check for WSL-specific files
    if std::path::Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists() {
        return true;
    }

    false
}
