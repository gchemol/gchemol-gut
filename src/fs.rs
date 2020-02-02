// imports

// [[file:~/Workspace/Programming/guts/guts.note::*imports][imports:1]]
pub use std::fs::File;
pub use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub use std::path::{Path, PathBuf};

use anyhow::*;
// imports:1 ends here

// pub
// adopted from quicli::fs

// [[file:~/Workspace/Programming/guts/guts.note::*pub][pub:1]]
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
// pub:1 ends here
