// imports

// [[file:~/Workspace/Programming/guts/guts.note::*imports][imports:1]]
use structopt::*;

use crate::prelude::*;
// imports:1 ends here

// trait

// [[file:~/Workspace/Programming/guts/guts.note::*trait][trait:1]]
pub trait Configure: Default + de::DeserializeOwned + Serialize {
    /// Print current configuration in toml format.
    fn print_toml(&self) {
        let x = toml::to_string(self).unwrap();
        println!("{:}", x);
    }

    /// Load configuration from default config file.
    ///
    /// # Panics
    ///
    /// * panics if configuration file does not exist.
    ///
    fn load() -> Self {
        let config_file = format!("{}.conf", env!("CARGO_PKG_NAME"));
        debug!("load config file: {}", config_file);

        Self::load_from_file(config_file)
    }

    /// Load configuration from file `config_file`.
    ///
    /// # Panics
    ///
    /// * panics if config_file does not exist.
    ///
    fn load_from_file<P: AsRef<std::path::Path>>(config_file: P) -> Self {
        let path = config_file.as_ref();
        let toml_str = crate::fs::read_file(path).expect("Failed to read config file!");
        toml::from_str(&toml_str).expect("Failed to parse toml config!")
    }
}

// FIXME: remove
pub trait DocHelp: StructOpt {
    /// Show help for configuration parameters
    fn print_help() {
        Self::clap().print_help().expect("clap help");
    }
}

impl<T> DocHelp for T where T: StructOpt {}
// trait:1 ends here

// reexports

// [[file:~/Workspace/Programming/guts/guts.note::*reexports][reexports:1]]
pub use lazy_static::*;
// reexports:1 ends here

// test

// [[file:~/Workspace/Programming/guts/guts.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Atom {
        /// Element symbol for this Atom
        symbol: String,

        /// Atomic mass
        mass: f64,
    }

    #[derive(Deserialize, Serialize, Debug)]
    /// User defined parameters for atoms
    pub(crate) struct Settings {
        /// user defined bond valence paramters
        pub atoms: Vec<Atom>,
    }

    impl Default for Settings {
        fn default() -> Self {
            let atoms = vec![
                Atom {
                    symbol: "H".into(),
                    mass: 1.008,
                },
                Atom {
                    symbol: "C".into(),
                    mass: 12.011,
                },
            ];

            Settings { atoms }
        }
    }

    impl Configure for Settings {}

    #[test]
    fn test_settings() {
        Settings::default().print_toml();
    }
}
// test:1 ends here
