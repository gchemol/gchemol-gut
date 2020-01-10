// imports

// [[file:~/Workspace/Programming/guts/guts.note::*imports][imports:1]]
pub use std::fs::File;
pub use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub use std::path::{Path, PathBuf};

use failure::{ensure, Error, ResultExt};
// imports:1 ends here

// pub
// adopted from quicli::fs

// [[file:~/Workspace/Programming/guts/guts.note::*pub][pub:1]]
/// Read file content into string
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let path = path.as_ref();
    ensure!(
        path.exists() && path.is_file(),
        "Path {:?} is not a file!",
        path
    );

    let file = File::open(path).with_context(|_| format!("Could not open file {:?}", path))?;
    let mut file = BufReader::new(file);

    let mut result = String::new();
    file.read_to_string(&mut result)
        .with_context(|_| format!("Could not read file {:?}", path))?;

    Ok(result)
}

/// Write string to file
///
/// _Note:_ Replaces the current file content if the file already exists.
pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), Error> {
    let path = path.as_ref();

    let file =
        File::create(path).with_context(|_| format!("Could not create/open file {:?}", path))?;
    let mut file = BufWriter::new(file);

    file.write_all(content.as_bytes())
        .with_context(|_| format!("Could not write to file {:?}", path))?;

    Ok(())
}
// pub:1 ends here
