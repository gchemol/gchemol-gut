// mods

// [[file:~/Workspace/Programming/guts/guts.note::*mods][mods:1]]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
// mods:1 ends here

// pub

// [[file:~/Workspace/Programming/guts/guts.note::*pub][pub:1]]
pub mod prelude {
    pub use itertools::Itertools;

    #[doc(hidden)]
    pub use failure::{bail, ensure, err_msg, format_err, ResultExt};
    pub type Result<T> = ::std::result::Result<T, failure::Error>;

    #[doc(hidden)]
    pub use serde::*;

    #[doc(hidden)]
    pub use log::{debug, error, info, trace, warn};

    pub use rayon::prelude::*;

    pub use lazy_static::*;
}

pub mod cli {
    pub use structopt::StructOpt;

    /// A handy alias for `Result` that carries a generic error type.
    pub type CliResult = ::std::result::Result<(), ::exitfailure::ExitFailure>;

    /// https://docs.rs/env_logger/*/env_logger/
    pub fn setup_logger() {
        use log::*;

        // only logging error by default
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "error");
        }

        pretty_env_logger::init();

        if !log_enabled!(Level::Debug) {
            eprintln!("To see the full logging, try setting `RUST_LOG=debug`.");
        }
    }
}

// adopted from quicli::fs
pub mod fs {
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Read, Write};

    pub use std::path::{Path, PathBuf};

    use failure::{ensure, Error, ResultExt};

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

        let file = File::create(path)
            .with_context(|_| format!("Could not create/open file {:?}", path))?;
        let mut file = BufWriter::new(file);

        file.write_all(content.as_bytes())
            .with_context(|_| format!("Could not write to file {:?}", path))?;

        Ok(())
    }
}
// pub:1 ends here
