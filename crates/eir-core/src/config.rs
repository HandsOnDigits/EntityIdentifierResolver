use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub root: PathBuf,

    #[serde(default = "default_max_segment_size")]
    pub max_segment_size: u64,

    #[serde(default = "default_max_segments")]
    pub max_segments: usize,
}

impl StorageConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;

        toml::from_str(&contents).map_err(|error| Error::InvalidFormat(error.to_string()))
    }

    pub fn segment_path(&self) -> PathBuf {
        self.root.join("segments")
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("data"),
            max_segment_size: default_max_segment_size(),
            max_segments: default_max_segments(),
        }
    }
}

fn default_max_segment_size() -> u64 {
    64 * 1024 * 1024
}

fn default_max_segments() -> usize {
    16
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub storage: StorageConfig,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;

        toml::from_str(&contents).map_err(|error| Error::InvalidFormat(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_config_loads() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let config_path = temp.path().join("eir.toml");

        std::fs::write(
            &config_path,
            r#"
[storage]
root = "data"
max_segment_size = 1024
max_segments = 4
"#,
        )?;

        let config = Config::load(&config_path)?;

        assert_eq!(config.storage.root, PathBuf::from("data"));
        assert_eq!(config.storage.max_segment_size, 1024);
        assert_eq!(config.storage.max_segments, 4);

        Ok(())
    }

    #[test]
    fn storage_config_has_defaults() {
        let config = StorageConfig::default();

        assert_eq!(config.root, PathBuf::from("data"));
        assert_eq!(config.max_segment_size, 64 * 1024 * 1024);
        assert_eq!(config.max_segments, 16);
    }

    #[test]
    fn storage_paths_are_derived_from_root() {
        let config = StorageConfig {
            root: PathBuf::from("data"),
            max_segment_size: 1024,
            max_segments: 4,
        };

        assert_eq!(config.segment_path(), PathBuf::from("data/segments"));
    }
}
