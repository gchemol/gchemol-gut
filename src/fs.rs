// [[file:../gut.note::*imports][imports:1]]
pub use std::fs::File;
pub use std::io::{BufRead, BufReader, BufWriter, Read, Seek, Write};

pub use std::path::{Path, PathBuf};

use anyhow::*;
// imports:1 ends here

// [[file:../gut.note::ac7549e1][ac7549e1]]
pub trait ShellEscapeLossyExt {
    fn shell_escape_lossy(&self) -> std::borrow::Cow<str>;
}

pub trait ShellEscapeExt {
    fn shell_escape(&self) -> std::borrow::Cow<str>;
}

impl ShellEscapeLossyExt for Path {
    /// Escape characters that may have special meaning in a shell.
    fn shell_escape_lossy(&self) -> std::borrow::Cow<str> {
        let s = self.to_string_lossy();
        shell_escape::escape(s)
    }
}

impl<'a> ShellEscapeExt for &'a str {
    /// Escape characters that may have special meaning in a shell.
    fn shell_escape(&self) -> std::borrow::Cow<str> {
        shell_escape::escape((*self).into())
    }
}
// ac7549e1 ends here

// [[file:../gut.note::eabef1ae][eabef1ae]]
pub use tempfile;

/// Read file content into string
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let s = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read string from file {:?}", path.as_ref()))?;

    Ok(s)
}

/// Write string to file
///
/// _Note:_ Replaces the current file content if the file already exists.
pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    std::fs::write(&path, content.as_bytes())
        .with_context(|| format!("Failed to write to file {:?}", path.as_ref()))?;

    Ok(())
}

/// Create an executable script. The leading directories will automatically
/// created.
#[cfg(unix)]
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
// eabef1ae ends here

// [[file:../gut.note::d392546b][d392546b]]
#[test]
fn test_shell_escape() {
    let f = "a b/test.sh";
    let f: &Path = f.as_ref();
    assert_eq!(f.shell_escape_lossy(), "'a b/test.sh'");
}
// d392546b ends here
