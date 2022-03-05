//! Usage
//! ```
//! use home_config::HomeConfig;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Default)]
//! struct Options {
//!     name: String,
//!     age: u32,
//! }
//!
//! let config = HomeConfig::new("your_name", "config.json");
//! // /Users/name/your_name/.config/config.json
//!
//! // Parse
//! let options = config.parse::<Options>().unwrap_or_default();
//! // options.name == "XiaoMing";
//! // options.age == 18;
//!
//! // Save to file
//! config.save(&options).unwrap();
//! ```

use dirs::home_dir;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Error as SerdeError;
use std::fs::{self, File};
use std::io::{Error as IoError, Write};
use std::path::{Path, PathBuf};

/// Failed to parse configuration file
#[derive(Debug)]
pub enum ConfigError {
    /// File does not exist
    Lost,
    /// Failed to read file
    Io(IoError),
    /// Serde Error
    Parse(SerdeError),
}

/// Use the configuration file in the current user directory
#[derive(Debug, Clone)]
pub struct HomeConfig {
    path: PathBuf,
}

impl HomeConfig {
    /// Parse or create configuration file
    ///
    /// eg. /Users/root/.config/<app_name>/config.json
    pub fn new<P: AsRef<Path>>(app_name: &'static str, file: P) -> Self {
        let path = home_dir().unwrap();
        Self {
            path: path.join(".config").join(app_name).join(file),
        }
    }

    /// Parse the configuration file into a struct
    pub fn parse<T>(&self) -> Result<T, ConfigError>
    where
        T: DeserializeOwned,
    {
        if !self.path.exists() {
            return Err(ConfigError::Lost);
        }
        let file = File::open(&self.path).map_err(ConfigError::Io)?;
        serde_json::from_reader(file).map_err(ConfigError::Parse)
    }

    /// Save struct to local file
    pub fn save<T>(&self, data: T) -> Result<(), IoError>
    where
        T: Serialize,
    {
        if !self.path.exists() {
            if let Some(parent) = self.path.parent() {
                fs::create_dir_all(parent)?;
            }
            File::create(&self.path)?;
        }

        let bytes = serde_json::to_vec_pretty(&data).unwrap();
        let mut f = File::create(&self.path)?;
        f.write_all(&bytes)?;
        Ok(())
    }
}
