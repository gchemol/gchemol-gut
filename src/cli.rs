// [[file:../gut.note::32071587][32071587]]
// NOTE: To make Clap derive work, clap must be included in Cargo.toml
pub use clap::{AppSettings, IntoApp};
pub use clap::{ArgEnum, Args, Subcommand};
pub use clap::{Parser, StructOpt};
pub use duct;

pub use crate::logger::setup_logger;
pub use crate::logger::setup_logger_for_test;
// 32071587 ends here

// [[file:../gut.note::352ffd7a][352ffd7a]]
#[derive(Parser, Debug, Clone, Default)]
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

        setup_logger();
    }
}
// 352ffd7a ends here
