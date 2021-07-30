// [[file:../gut.note::*src][src:1]]
// NOTE: To make StructOpt derive work, structopt must be included in Cargo.toml
pub use structopt::*;

pub use duct;

/// A handy alias for `Result` that carries a generic error type.
// pub type CliResult = ::std::result::Result<(), ::exitfailure::ExitFailure>;
#[deprecated(note = "Use anyhow::Result<()>")]
pub type CliResult = anyhow::Result<()>;

pub use crate::logger::{setup_logger, setup_logger_for_test};
// src:1 ends here

// [[file:../gut.note::*verbose][verbose:1]]
// adopted from: https://github.com/rust-cli/clap-verbosity-flag
#[derive(structopt::StructOpt, Debug, Clone, Default)]
pub struct Verbosity {
    /// Pass many times for more log output
    ///
    /// By default, it will only report warnings.
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,
}

impl Verbosity {
    /// Set up logging according to verbosity level.
    pub fn setup_logger(&self) {
        // allow user to override using RUST_LOG env
        if std::env::var("RUST_LOG").is_err() {
            match self.verbose {
                0 => std::env::set_var("RUST_LOG", "warn"),
                1 => std::env::set_var("RUST_LOG", "info"),
                2 => std::env::set_var("RUST_LOG", "debug"),
                _ => std::env::set_var("RUST_LOG", "trace"),
            }
        }

        crate::logger::setup_logger();
    }

    /// Set up logging according to verbosity level.
    pub fn setup_plain_logger(&self) {
        // allow user to override using RUST_LOG env
        if std::env::var("RUST_LOG").is_err() {
            match self.verbose {
                0 => std::env::set_var("RUST_LOG", "warn"),
                1 => std::env::set_var("RUST_LOG", "info"),
                2 => std::env::set_var("RUST_LOG", "debug"),
                _ => std::env::set_var("RUST_LOG", "trace"),
            }
        }

        crate::logger::setup_plain_logger();
    }
}
// verbose:1 ends here
