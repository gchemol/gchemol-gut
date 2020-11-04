// [[file:../gut.note::*imports][imports:1]]
use crate::prelude::*;
// imports:1 ends here

// [[file:../gut.note::*trait][trait:1]]
pub trait Configure: Default + de::DeserializeOwned + Serialize {
    #[deprecated(note = "plan to be removed")]
    /// Print current configuration in toml format.
    fn print_toml(&self) {
        println!("{:}", self.to_toml().unwrap());
    }

    #[deprecated(note = "plan to be removed")]
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

    #[deprecated(note = "plan to be removed")]
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

    /// Deserialize an instance of type T from a string of JSON text.
    fn from_json(s: &str) -> Result<Self> {
        let x = serde_json::from_str(s)?;
        Ok(x)
    }

    /// Deserialize an instance of type T from a string of TOML text.
    fn from_toml(s: &str) -> Result<Self> {
        let x = toml::from_str(s)?;
        Ok(x)
    }

    /// Serialize it to a JSON string.
    fn to_json(&self) -> Result<String> {
        let s = serde_json::to_string_pretty(self)?;
        Ok(s)
    }

    /// Serialize self to a TOML string.
    fn to_toml(&self) -> Result<String> {
        let s = toml::to_string(self)?;
        Ok(s)
    }
}
// trait:1 ends here

// [[file:../gut.note::*reexports][reexports:1]]
pub use lazy_static::*;
// reexports:1 ends here

// [[file:../gut.note::*test][test:1]]
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
        let s = Settings::default().to_json().unwrap();
        println!("{}", s);
    }
}
// test:1 ends here
