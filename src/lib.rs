// mods

// [[file:~/Workspace/Programming/guts/guts.note::*mods][mods:1]]

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

pub mod cli;
pub mod config;
pub mod fs;
// pub:1 ends here
