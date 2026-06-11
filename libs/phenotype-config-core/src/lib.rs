//! # Phenotype Config Core
//!
//! Minimal, composable config loading for Phenotype crates.
//!
//! Loads TOML config from a cascade of sources:
//! 1. System config (`/etc/phenotype/<name>.toml`)
//! 2. User config (`~/.config/phenotype/<name>.toml`)
//! 3. Project config (`./<name>.toml`)
//! 4. Custom paths via [`ConfigLoader::with_path`]
//!
//! ## Quick start
//!
//! ```
//! use phenotype_config_core::ConfigLoader;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, PartialEq, Debug)]
//! struct AppConfig {
//!     name: String,
//!     port: u16,
//! }
//!
//! // `load_from` reads a specific file (no env or path-cascade).
//! let dir = tempfile::tempdir().unwrap();
//! let path = dir.path().join("app.toml");
//! std::fs::write(&path, "name = \"demo\"\nport = 9090\n").unwrap();
//!
//! let cfg: AppConfig = ConfigLoader::load_from(&path).unwrap();
//! assert_eq!(cfg, AppConfig { name: "demo".to_string(), port: 9090 });
//! ```
//!
//! [`ConfigLoader::with_path`]: ConfigLoader::with_path

use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors produced by the loader.
///
/// # Examples
///
/// ```
/// use phenotype_config_core::ConfigError;
///
/// let not_found = ConfigError::NotFound;
/// assert_eq!(not_found.to_string(), "config not found at any search path");
///
/// let io_err = ConfigError::Io(std::io::Error::new(
///     std::io::ErrorKind::NotFound,
///     "missing file",
/// ));
/// assert!(io_err.to_string().starts_with("io error: "));
/// ```
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("config not found at any search path")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, ConfigError>;

/// Config loader with cascading source resolution.
pub struct ConfigLoader {
    name: String,
    search_paths: Vec<PathBuf>,
}

impl ConfigLoader {
    /// Create a new config loader for the given config name (e.g. "agileplus").
    ///
    /// The loader is seeded with three default search paths:
    /// `<config_dir>/phenotype/<name>.toml` (when the host has a
    /// `config_dir()`), `/etc/phenotype/<name>.toml`, and the project
    /// cwd entry `<name>.toml`. Add more with
    /// [`ConfigLoader::with_path`].
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_config_core::ConfigLoader;
    ///
    /// let loader = ConfigLoader::new("myapp");
    ///
    /// // Every search path ends in `<name>.toml`.
    /// assert!(loader.search_paths().iter().all(|p| p.ends_with("myapp.toml")));
    /// assert_eq!(loader.name(), "myapp");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let mut search_paths = Vec::new();

        // System config
        search_paths.push(PathBuf::from(format!("/etc/phenotype/{name}.toml")));

        // User config
        if let Some(config_dir) = dirs::config_dir() {
            search_paths.push(config_dir.join("phenotype").join(format!("{name}.toml")));
        }

        // Project config (cwd)
        search_paths.push(PathBuf::from(format!("{name}.toml")));

        Self { name, search_paths }
    }

    /// Add a custom search path.
    ///
    /// The path is inserted just before the project-cwd entry, so it
    /// takes precedence over `./<name>.toml` but is still beaten by
    /// the system and user-cascade entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_config_core::ConfigLoader;
    ///
    /// let dir = tempfile::tempdir().unwrap();
    /// let custom = dir.path().join("my.toml");
    ///
    /// let loader = ConfigLoader::new("myapp").with_path(&custom);
    /// let paths = loader.search_paths();
    /// assert!(paths.iter().any(|p| p == &custom));
    /// // The original cwd entry is still last.
    /// assert_eq!(paths.last().unwrap(), &std::path::PathBuf::from("myapp.toml"));
    /// ```
    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        let pos = self.search_paths.len().saturating_sub(1);
        self.search_paths.insert(pos, path.into());
        self
    }

    /// Load and deserialize config from the first found file.
    ///
    /// Walks [`ConfigLoader::search_paths`] in order and parses the
    /// first file that exists. Returns
    /// [`ConfigError::NotFound`] when no entry in the cascade resolves
    /// to a readable file.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_config_core::{ConfigError, ConfigLoader};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct TestCfg;
    ///
    /// // A name with no on-disk entries yields `NotFound`.
    /// let loader = ConfigLoader::new("nonexistent-phenotype-config-xyz");
    /// let result: Result<TestCfg, _> = loader.load();
    /// assert!(matches!(result, Err(ConfigError::NotFound)));
    /// ```
    pub fn load<T: DeserializeOwned>(&self) -> Result<T> {
        for path in &self.search_paths {
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                let config: T = toml::from_str(&content)?;
                return Ok(config);
            }
        }
        Err(ConfigError::NotFound)
    }

    /// Load from a specific file path.
    ///
    /// Use this when you already know the on-disk location of the
    /// config file and want to bypass the
    /// [`ConfigLoader::search_paths`](ConfigLoader::search_paths)
    /// cascade.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_config_core::ConfigLoader;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// struct DbConfig {
    ///     url: String,
    ///     max_conns: u16,
    /// }
    ///
    /// let dir = tempfile::tempdir().unwrap();
    /// let path = dir.path().join("db.toml");
    /// std::fs::write(
    ///     &path,
    ///     "url = \"postgres://localhost/x\"\nmax_conns = 8\n",
    /// )
    /// .unwrap();
    ///
    /// let cfg: DbConfig = ConfigLoader::load_from(&path).unwrap();
    /// assert_eq!(
    ///     cfg,
    ///     DbConfig {
    ///         url: "postgres://localhost/x".to_string(),
    ///         max_conns: 8,
    ///     }
    /// );
    /// ```
    pub fn load_from<T: DeserializeOwned>(path: &Path) -> Result<T> {
        let content = std::fs::read_to_string(path)?;
        let config: T = toml::from_str(&content)?;
        Ok(config)
    }

    /// Return the list of paths that will be searched.
    pub fn search_paths(&self) -> &[PathBuf] {
        &self.search_paths
    }

    /// Get the config name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::io::Write;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        port: u16,
    }

    #[test]
    fn load_from_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.toml");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "name = \"myapp\"\nport = 8080").unwrap();

        let config: TestConfig = ConfigLoader::load_from(&path).unwrap();
        assert_eq!(config.name, "myapp");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn not_found() {
        let loader = ConfigLoader::new("nonexistent-config-xyz");
        let result: Result<TestConfig> = loader.load();
        assert!(matches!(result, Err(ConfigError::NotFound)));
    }

    #[test]
    fn custom_search_path() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("custom.toml");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "name = \"custom\"\nport = 3000").unwrap();

        let loader = ConfigLoader::new("custom").with_path(dir.path().join("custom.toml"));
        let config: TestConfig = loader.load().unwrap();
        assert_eq!(config.name, "custom");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn search_paths_populated() {
        let loader = ConfigLoader::new("test");
        assert!(!loader.search_paths().is_empty());
        assert_eq!(loader.name(), "test");
    }
}
