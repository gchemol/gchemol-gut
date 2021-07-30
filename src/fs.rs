// [[file:../gut.note::*imports][imports:1]]
pub use std::fs::File;
pub use std::io::{BufRead, BufReader, BufWriter, Read, Seek, Write};

pub use std::path::{Path, PathBuf};

use anyhow::*;
// imports:1 ends here

// [[file:../gut.note::*pub][pub:1]]
/// Read file content into string
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let s = std::fs::read_to_string(&path).with_context(|| format!("Failed to read string from file {:?}", path.as_ref()))?;

    Ok(s)
}

/// Write string to file
///
/// _Note:_ Replaces the current file content if the file already exists.
pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    std::fs::write(&path, content.as_bytes()).with_context(|| format!("Failed to write to file {:?}", path.as_ref()))?;

    Ok(())
}

/// Create an executable script. The leading directories will automatically
/// created.
pub fn write_script_file(script_path: &Path, content: &str) -> Result<()> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // Create leading directories
    if let Some(dir) = script_path.parent() {
        fs::create_dir_all(dir).context("Create leading directories")?;
    }

    // Write content and make script executable
    write_to_file(script_path, content)?;
    fs::set_permissions(script_path, fs::Permissions::from_mode(0o755)).context("chmod +x")?;

    Ok(())
}
// pub:1 ends here
