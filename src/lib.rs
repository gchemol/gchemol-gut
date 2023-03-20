// [[file:../gut.note::36e53031][36e53031]]
#![deny(clippy::all)]

pub mod cli;
pub mod config;
pub mod fs;
pub mod logger;
pub mod utils;
// 36e53031 ends here

// [[file:../gut.note::b07726f0][b07726f0]]
pub mod prelude {
    pub use crate::config::Configure;

    pub use itertools::Itertools;

    pub use anyhow::Context as _Context; // avoid name conflicting
    pub use anyhow::Ok as Ok_; // useful when return a Result in a closure
    pub use anyhow::{anyhow, bail, ensure, format_err};
    pub use anyhow::{Error, Result};

    #[doc(hidden)]
    // NOTE: to make serde deriving work, serde must be included in Cargo.toml
    pub use serde::*;

    pub use super::log_dbg;
    #[doc(hidden)]
    pub use log::{debug, error, info, trace, warn};

    pub use rayon::prelude::*;

    // for write! and writeln! macros
    // avoid name conflict with std::io::Write
    pub use std::fmt::Write as FmtWrite;
    pub use std::io::{Read, Write};

    // FooBar::from_str
    pub use std::str::FromStr;

    // provides shell_escape_lossy method for `Path`
    pub use crate::fs::{ShellEscapeExt, ShellEscapeLossyExt};
}

// re-exports external crates
pub use itertools;
pub use rayon;

/// Similar to std::dbg! macro, but print with info! instead of eprintln!
#[macro_export]
macro_rules! log_dbg {
    () => {
        info!("{}:{}", file!(), line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                info!("{}:{} {} = {:#?}",
                      file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::log_dbg!($val)),+,)
    };
}
// b07726f0 ends here
