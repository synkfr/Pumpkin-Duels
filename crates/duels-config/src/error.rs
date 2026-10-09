use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse TOML at {path}: {source}")]
    Parse {
        path: PathBuf,
        source: Box<toml_edit::TomlError>,
    },

    #[error("Failed to deserialize configuration at {path}: {source}")]
    Deserialize {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("Failed to serialize configuration at {path}: {source}")]
    Serialize {
        path: PathBuf,
        source: toml::ser::Error,
    },
}
