use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// All filesystem paths belonging to one EIR database.
///
/// A database has one logical `.eir` file and a physical storage
/// directory containing configuration, segments, and WAL data.
///
/// Example:
///
/// ```text
/// nutrition/
/// ├── nutrition.eir
/// ├── eir.toml
/// ├── segments/
/// │   ├── 0000.deir
/// │   └── 0001.deir
/// └── wal
/// ```
#[derive(Debug, Clone)]
pub struct DatabasePaths {
    pub root: PathBuf,
    pub database: PathBuf,
    pub config: PathBuf,
    pub segments: PathBuf,
    pub wal: PathBuf,
}

impl DatabasePaths {
    /// Create paths for a new named database.
    ///
    /// data + "foods" =>
    ///
    /// data/
    /// └── foods/
    ///     ├── foods.eir
    ///     ├── eir.toml
    ///     ├── segments/
    ///     └── wal/
    pub fn new(parent: impl AsRef<Path>, name: &str) -> Self {
        let root = parent.as_ref().join(name);

        Self {
            database: root.join(format!("{name}.eir")),
            config: root.join("eir.toml"),
            segments: root.join("segments"),
            wal: root.join("wal"),
            root,
        }
    }

    /// Resolve the physical database layout from an existing `.eir` file.
    pub fn from_database(path: &Path) -> Self {
        let database = path.to_path_buf();

        let root = database
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        Self {
            database,
            config: root.join("eir.toml"),
            segments: root.join("segments"),
            wal: root.join("wal"),
            root,
        }
    }

    pub fn config_path(&self) -> &Path {
        &self.config
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    /// Human-readable and logical database name.
    pub name: String,

    /// Physical storage root.
    ///
    /// This is resolved from the location of `eir.toml` when loading
    /// an existing database.
    #[serde(skip)]
    pub root: PathBuf,

    #[serde(default = "default_max_segment_size")]
    pub max_segment_size: u64,

    #[serde(default = "default_max_segments")]
    pub max_segments: usize,
}

impl StorageConfig {
    pub fn segment_path(&self) -> PathBuf {
        self.root.join("segments")
    }

    pub fn wal_path(&self) -> PathBuf {
        self.root.join("wal")
    }

    pub fn config_path(&self) -> PathBuf {
        self.root.join("eir.toml")
    }

    pub fn database_path(&self) -> PathBuf {
        self.root.join(format!("{}.eir", self.name))
    }

    pub fn paths(&self) -> DatabasePaths {
        DatabasePaths {
            database: self.database_path(),
            config: self.config_path(),
            segments: self.segment_path(),
            wal: self.wal_path(),
            root: self.root.clone(),
        }
    }

    pub fn for_database(parent: impl AsRef<Path>, name: impl Into<String>) -> Self {
        let name = name.into();
        let paths = DatabasePaths::new(parent, &name);

        Self {
            name,
            root: paths.root,
            ..Default::default()
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            name: "database".to_string(),
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub storage: StorageConfig,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let contents = std::fs::read_to_string(path)?;

        let mut config: Config =
            toml::from_str(&contents).map_err(|error| Error::InvalidFormat(error.to_string()))?;

        // The storage root is determined by the location of eir.toml,
        // rather than being stored as a portable absolute/relative path
        // in the configuration file.
        config.storage.root = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = self.storage.config_path();

        let contents = toml::to_string_pretty(self)
            .map_err(|error| Error::InvalidFormat(error.to_string()))?;

        std::fs::write(path, contents)?;

        Ok(())
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
name = "nutrition"
max_segment_size = 1024
max_segments = 4
"#,
        )?;

        let config = Config::load(&config_path)?;

        assert_eq!(config.storage.name, "nutrition");
        assert_eq!(config.storage.root, temp.path());
        assert_eq!(config.storage.max_segment_size, 1024);
        assert_eq!(config.storage.max_segments, 4);

        Ok(())
    }

    #[test]
    fn storage_config_has_defaults() {
        let config = StorageConfig::default();

        assert_eq!(config.name, "database");
        assert_eq!(config.root, PathBuf::from("data"));
        assert_eq!(config.max_segment_size, 64 * 1024 * 1024);
        assert_eq!(config.max_segments, 16);
    }

    #[test]
    fn storage_paths_are_derived_from_root() {
        let config = StorageConfig {
            name: "nutrition".into(),
            root: PathBuf::from("data/nutrition"),
            max_segment_size: 1024,
            max_segments: 4,
        };

        assert_eq!(
            config.database_path(),
            PathBuf::from("data/nutrition/nutrition.eir")
        );

        assert_eq!(
            config.segment_path(),
            PathBuf::from("data/nutrition/segments")
        );

        assert_eq!(config.wal_path(), PathBuf::from("data/nutrition/wal"));
    }

    #[test]
    fn database_paths_use_explicit_name() {
        let paths = DatabasePaths::new("data", "nutrition");

        assert_eq!(paths.root, PathBuf::from("data/nutrition"));
        assert_eq!(
            paths.database,
            PathBuf::from("data/nutrition/nutrition.eir")
        );
        assert_eq!(paths.config, PathBuf::from("data/nutrition/eir.toml"));
        assert_eq!(paths.segments, PathBuf::from("data/nutrition/segments"));
        assert_eq!(paths.wal, PathBuf::from("data/nutrition/wal"));
    }

    #[test]
    fn database_paths_from_database_preserves_root() {
        let paths = DatabasePaths::from_database(Path::new("data/nutrition/nutrition.eir"));

        assert_eq!(paths.root, PathBuf::from("data/nutrition"));
        assert_eq!(
            paths.database,
            PathBuf::from("data/nutrition/nutrition.eir")
        );
        assert_eq!(paths.config, PathBuf::from("data/nutrition/eir.toml"));
        assert_eq!(paths.segments, PathBuf::from("data/nutrition/segments"));
        assert_eq!(paths.wal, PathBuf::from("data/nutrition/wal"));
    }
}
