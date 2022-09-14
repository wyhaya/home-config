//! Usage
//! ```no_run
//! use home_config::HomeConfig;
//!
//! let config = HomeConfig::with_config_dir("app", "config");
//! // Linux: /home/name/.config/app/config
//! // macOS: /Users/name/.config/app/config
//! // Windows: C:\Users\name\.config\app\config
//!
//! // Write
//! config.save("123456789").unwrap();
//!
//! // Read
//! let data = config.read_to_string().unwrap();
//! // 123456789
//! ```
//!
//! ### Serde support
//!
//! * feature `json`
//! * feature `yaml`
//! * feature `toml`
//!
//! ```toml
//! home-config = { version = "*", features = ["json", "yaml", "toml"] }
//! ```
//!
//! ```no_run
//! use home_config::HomeConfig;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Default)]
//! struct People {
//!     name: String,
//!     age: u32,
//! }
//!
//! let config = HomeConfig::with_file("test.json");
//! // Linux: /home/name/test.json
//! // macOS: /Users/name/test.json
//! // Windows: C:\Users\name\test.json
//!
//! // Parse
//! let people = config.json::<People>().unwrap();
//! // people.name == "XiaoMing";
//! // people.age == 18;
//!
//! // Save to file
//! config.save_json(&people).unwrap();
//! ```

#[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
#[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
use std::io::Error as IoError;
use std::io::{ErrorKind, Read, Result as IoResult};
use std::path::{Path, PathBuf};

fn home_dir() -> PathBuf {
    dirs::home_dir().expect("get home dir")
}

/// Serde `json` error
#[derive(Debug)]
#[cfg(feature = "json")]
pub enum JsonError {
    Io(IoError),
    Serde(serde_json::Error),
}

/// Serde `yaml` error
#[derive(Debug)]
#[cfg(feature = "yaml")]
pub enum YamlError {
    Io(IoError),
    Serde(serde_yaml::Error),
}

/// Serde `toml` parse error
#[derive(Debug)]
#[cfg(feature = "toml")]
pub enum TomlParseError {
    Io(IoError),
    Serde(toml::de::Error),
}

/// Serde `toml` save error
#[derive(Debug)]
#[cfg(feature = "toml")]
pub enum TomlSaveError {
    Io(IoError),
    Serde(toml::ser::Error),
}

/// Use the configuration file in the current user directory
#[derive(Debug, Clone)]
pub struct HomeConfig {
    path: PathBuf,
}

impl HomeConfig {
    /// Parse or create configuration file
    ///
    /// eg. `/home/name/.config/app/config`
    pub fn with_config_dir<P: AsRef<Path>>(app_name: &'static str, file_name: P) -> Self {
        Self {
            path: home_dir().join(".config").join(app_name).join(file_name),
        }
    }

    /// Parse or create configuration file
    ///
    /// eg. `/home/name/test.json`
    pub fn with_file<P: AsRef<Path>>(p: P) -> Self {
        Self {
            path: home_dir().join(p),
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

    /// Read the entire contents of a file into a `Vec<u8>`
    pub fn read_to_vec(&self) -> IoResult<Vec<u8>> {
        let mut f = File::open(&self.path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Parse the config file from `json` content
    #[cfg(feature = "json")]
    pub fn json<T>(&self) -> Result<T, JsonError>
    where
        T: DeserializeOwned,
    {
        let f = File::open(&self.path).map_err(JsonError::Io)?;
        serde_json::from_reader(f).map_err(JsonError::Serde)
    }

    /// Parse the config file from `yaml` content
    #[cfg(feature = "yaml")]
    pub fn yaml<T>(&self) -> Result<T, YamlError>
    where
        T: DeserializeOwned,
    {
        let f = File::open(&self.path).map_err(YamlError::Io)?;
        serde_yaml::from_reader(f).map_err(YamlError::Serde)
    }

    /// Parse the config file from `toml` content
    #[cfg(feature = "toml")]
    pub fn toml<T>(&self) -> Result<T, TomlParseError>
    where
        T: DeserializeOwned,
    {
        let bytes = self.read_to_vec().map_err(TomlParseError::Io)?;
        toml::from_slice(&bytes).map_err(TomlParseError::Serde)
    }

    fn create_parent_dir(&self) -> IoResult<()> {
        if !self.path.exists() {
            if let Some(parent) = self.path.parent() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }

    /// Save content to local file
    pub fn save<T: AsRef<[u8]>>(&self, data: T) -> IoResult<()> {
        self.create_parent_dir()?;
        fs::write(&self.path, data.as_ref())
    }

    /// Save struct to local file (`json` format)
    #[cfg(feature = "json")]
    pub fn save_json<T>(&self, data: T) -> Result<(), JsonError>
    where
        T: Serialize,
    {
        let bytes = serde_json::to_vec_pretty(&data).map_err(JsonError::Serde)?;
        self.create_parent_dir().map_err(JsonError::Io)?;
        fs::write(&self.path, &bytes).map_err(JsonError::Io)?;
        Ok(())
    }

    /// Save struct to local file (`yaml` format)
    #[cfg(feature = "yaml")]
    pub fn save_yaml<T>(&self, data: T) -> Result<(), YamlError>
    where
        T: Serialize,
    {
        let bytes = serde_yaml::to_string(&data).map_err(YamlError::Serde)?;
        self.create_parent_dir().map_err(YamlError::Io)?;
        fs::write(&self.path, &bytes).map_err(YamlError::Io)?;
        Ok(())
    }

    /// Save struct to local file (`toml` format)
    #[cfg(feature = "toml")]
    pub fn save_toml<T>(&self, data: T) -> Result<(), TomlSaveError>
    where
        T: Serialize,
    {
        let bytes = toml::to_string_pretty(&data).map_err(TomlSaveError::Serde)?;
        self.create_parent_dir().map_err(TomlSaveError::Io)?;
        fs::write(&self.path, &bytes).map_err(TomlSaveError::Io)?;
        Ok(())
    }

    /// Delete the config file
    pub fn delete(&self) -> IoResult<()> {
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_content() {
        let config = HomeConfig::with_config_dir("test", "file");
        // Save
        config.save("123").unwrap();
        // Read
        assert_eq!(config.read_to_string().unwrap(), "123");
    }

    #[test]
    fn test_delete() {
        let config = HomeConfig::with_config_dir("test", "delete");

        assert!(!config.path().exists());

        config.save("0").unwrap();
        assert!(config.path().exists());

        config.delete().unwrap();
        assert!(!config.path().exists());
    }

    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    use serde::{Deserialize, Serialize};

    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    #[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
    struct People {
        name: String,
        age: u32,
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json() {
        let config = HomeConfig::with_config_dir("test", "config.json");
        let data = People {
            name: "123".to_string(),
            age: 18,
        };
        config.save_json(&data).unwrap();
        assert_eq!(config.json::<People>().unwrap(), data);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml() {
        let config = HomeConfig::with_config_dir("test", "config.yaml");
        let data = People {
            name: "123".to_string(),
            age: 18,
        };
        config.save_yaml(&data).unwrap();
        assert_eq!(config.yaml::<People>().unwrap(), data);
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml() {
        let config = HomeConfig::with_config_dir("test", "config.toml");
        let data = People {
            name: "123".to_string(),
            age: 18,
        };
        config.save_toml(&data).unwrap();
        assert_eq!(config.toml::<People>().unwrap(), data);
    }
}
