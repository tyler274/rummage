use std::path::Path;
use std::process::Command;

/// Documentation utilities for building and serving the project documentation
pub struct Docs;

impl Docs {
    /// Build the documentation using mdbook
    pub fn build() -> Result<(), String> {
        println!("Building documentation...");
        let output = Command::new("mdbook")
            .args(["build", "docs"])
            .current_dir(Self::project_root())
            .output()
            .map_err(|e| format!("Failed to execute mdbook: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "mdbook build failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        println!("Documentation built successfully!");
        Ok(())
    }

    /// Serve the documentation using mdbook
    pub fn serve() -> Result<(), String> {
        println!("Starting documentation server...");
        println!("Open your browser to http://localhost:3000 to view the documentation");
        println!("Press Ctrl+C to stop the server");

        let status = Command::new("mdbook")
            .args(["serve", "docs", "--open"])
            .current_dir(Self::project_root())
            .status()
            .map_err(|e| format!("Failed to execute mdbook: {}", e))?;

        if !status.success() {
            return Err("mdbook serve failed".to_string());
        }

        Ok(())
    }

    /// Check the documentation for broken links
    pub fn check() -> Result<(), String> {
        println!("Checking documentation...");
        let output = Command::new("mdbook")
            .args(["test", "docs"])
            .current_dir(Self::project_root())
            .output()
            .map_err(|e| format!("Failed to execute mdbook: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "mdbook test failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        println!("Documentation checks passed!");
        Ok(())
    }

    /// Get the project root directory
    fn project_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
    }
}
