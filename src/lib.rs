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
//! let config = HomeConfig::new("app", "config.json");
//! // macOS: /Users/name/.config/app/config.json
//! // Linux: /home/name/.config/app/config.json
//! // Windows: C:\Users\name\app\config.json
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
use std::io::{Error as IoError, ErrorKind, Result as IoResult, Write};
use std::path::{Path, PathBuf};

/// Failed to parse configuration file
#[derive(Debug)]
pub enum ParseError {
    /// File does not exist
    NotFound,
    /// Failed to read file
    Io(IoError),
    /// Serde Error
    Deserialize(SerdeError),
}

/// Failed to save configuration file
#[derive(Debug)]
pub enum SaveError {
    /// Failed to write file
    Io(IoError),
    /// Serde Error
    Serialize(SerdeError),
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

    /// Get the configuration file path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Read the entire contents of a file into a string
    pub fn read_to_string(&self) -> IoResult<String> {
        fs::read_to_string(&self.path)
    }

    /// Parse the configuration file into a struct
    pub fn parse<T>(&self) -> Result<T, ParseError>
    where
        T: DeserializeOwned,
    {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                return Err(ParseError::NotFound);
            }
            Err(err) => {
                return Err(ParseError::Io(err));
            }
        };
        serde_json::from_reader(file).map_err(ParseError::Deserialize)
    }

    /// Save struct to local file
    pub fn save<T>(&self, data: T) -> Result<(), SaveError>
    where
        T: Serialize,
    {
        if !self.path.exists() {
            if let Some(parent) = self.path.parent() {
                fs::create_dir_all(parent).map_err(SaveError::Io)?;
            }
        }

        let bytes = serde_json::to_vec_pretty(&data).map_err(SaveError::Serialize)?;
        let mut f = File::create(&self.path).map_err(SaveError::Io)?;
        f.write_all(&bytes).map_err(SaveError::Io)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
    struct Options {
        name: String,
        age: u32,
    }

    #[test]
    fn test_parse() {
        let config = HomeConfig::new("test", "not.json");

        let rst = config.parse::<Options>().err().unwrap();
        assert!(matches!(rst, ParseError::NotFound));

        // todo
    }

    #[test]
    fn test_save() {
        let config = HomeConfig::new("test", "config.json");

        config
            .save(Options {
                name: "XiaoMing".to_string(),
                age: 18,
            })
            .unwrap();

        let opt = config.parse::<Options>().unwrap();
        assert_eq!(opt.age, 18);
        assert_eq!(opt.name, "XiaoMing");
    }
}
