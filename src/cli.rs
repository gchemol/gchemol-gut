// src

// [[file:~/Workspace/Programming/guts/guts.note::*src][src:1]]
// pub use structopt::StructOpt;

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
// src:1 ends here
