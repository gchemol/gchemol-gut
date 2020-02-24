// mods

// [[file:~/Workspace/Programming/gchemol-rs/gut/gut.note::*mods][mods:1]]

// mods:1 ends here

// pub

// [[file:~/Workspace/Programming/gchemol-rs/gut/gut.note::*pub][pub:1]]
pub mod prelude {
    pub use crate::config::Configure;

    pub use itertools::Itertools;

    pub use anyhow::Context as _Context; // avoid name conflicting
    pub use anyhow::{anyhow, bail, ensure, format_err};
    pub use anyhow::{Error, Result};

    #[doc(hidden)]
    pub use serde::*;

    #[doc(hidden)]
    pub use log::{debug, error, info, trace, warn};

    pub use rayon::prelude::*;

    // for write! and writeln! macros
    // avoid name conflict with std::io::Write
    pub use std::fmt::Write as FmtWrite;
}

pub mod cli;
pub mod config;
pub mod fs;

pub use itertools;
// pub:1 ends here
