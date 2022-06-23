// [[file:../gut.note::379d56a3][379d56a3]]
/// Returns a `env_logger::Builder` for further customization.
fn formatted_builder(test: bool) -> env_logger::Builder {
    let mut builder = env_logger::builder();
    builder.format_timestamp_secs();
    builder.is_test(test);
    builder
}
// 379d56a3 ends here

// [[file:../gut.note::3ba1b0d4][3ba1b0d4]]
/// Setup logging for cmdline uses
///
/// https://docs.rs/env_logger/*/env_logger/
///
/// # Panics
///
/// * This function will panic if it is called more than once, or if another
/// library has already initialized a global logger.
///
pub fn setup_logger() {
    // only logging warn by default
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    formatted_builder(false).init();
}

/// Setup logging for cargo test
///
/// https://github.com/sebasmagri/env_logger/pull/127
pub fn setup_logger_for_test() {
    // logging debug by default
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    if let Err(e) = formatted_builder(true).try_init() {
        eprintln!("setup logger failed: {:?}", e);
    }
}

/// A plain logger no formatting, like using println macro
pub fn setup_plain_logger() {
    use std::io::Write;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
}
// 3ba1b0d4 ends here
