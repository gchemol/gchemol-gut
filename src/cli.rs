// [[file:~/Workspace/Programming/gchemol-rs/gut/gut.note::*src][src:1]]
// pub use structopt::StructOpt;

/// A handy alias for `Result` that carries a generic error type.
// pub type CliResult = ::std::result::Result<(), ::exitfailure::ExitFailure>;
#[deprecated(note = "Use anyhow::Result<()>")]
pub type CliResult = anyhow::Result<()>;

/// Setup logging for cmdline uses
///
/// https://docs.rs/env_logger/*/env_logger/
pub fn setup_logger() {
    use log::*;

    // only logging warn by default
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    pretty_env_logger::init();

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
