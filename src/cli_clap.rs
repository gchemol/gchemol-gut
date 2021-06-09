// [[file:../gut.note::*src][src:1]]
// NOTE: To make Clap derive work, clap must be included in Cargo.toml
pub use clap::{Clap, IntoApp};
pub use duct;

/// Setup logging for cmdline uses
///
/// https://docs.rs/env_logger/*/env_logger/
pub fn setup_logger() {
    use log::*;

    // only logging warn by default
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    pretty_env_logger::init_timed();

    if !log_enabled!(Level::Debug) {
        eprintln!("To see the full logging, try setting `RUST_LOG=debug`.");
    }
}

/// Setup logging for cargo test
///
/// https://github.com/sebasmagri/env_logger/pull/127
pub fn setup_logger_for_test() {
    // logging debug by default
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    // FIXME: pretty_env_logger's builder not work
    // if let Err(e) = pretty_env_logger::formatted_builder().is_test(true).try_init() {
    if let Err(e) = env_logger::builder().is_test(true).try_init() {
        eprintln!("setup logger failed: {:?}", e);
    }
}
// src:1 ends here

// [[file:../gut.note::*verbose][verbose:1]]
#[derive(Clap, Debug, Clone, Default)]
pub struct Verbosity {
    /// Pass many times for more log output (-v, -vv, -vvv)
    ///
    /// By default, it will only report warnings.
    #[clap(long, short, parse(from_occurrences))]
    verbose: i8,
}

impl Verbosity {
    /// Set up logging according to verbosity level.
    pub fn setup_logger(&self) {
        match self.verbose {
            0 => std::env::set_var("RUST_LOG", "warn"),
            1 => std::env::set_var("RUST_LOG", "info"),
            2 => std::env::set_var("RUST_LOG", "debug"),
            _ => std::env::set_var("RUST_LOG", "trace"),
        }

        pretty_env_logger::init_timed();
    }
}
// verbose:1 ends here
