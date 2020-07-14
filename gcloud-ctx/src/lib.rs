mod configuration;
mod properties;

pub use configuration::{Configuration, ConfigurationStore};
pub use properties::*;

use std::path::PathBuf;
use thiserror::Error;

/// gcloud-ctx result
pub type Result<T> = std::result::Result<T, Error>;

/// gcloud-ctx error
#[derive(Debug, Error)]
pub enum Error {
    /// The configuration directory was not found within the configuration store directory
    #[error("Unable to locate user configuration directory")]
    ConfigurationDirectoryNotFound,

    /// Unable to find the gcloud configuration root directory
    #[error("Unable to find the gcloud configuration directory at {0}\n\nIs gcloud installed?")]
    ConfigurationStoreNotFound(PathBuf),

    /// Attempted to delete the active configuration
    #[error("Unable to delete the configuration because it is currently active")]
    DeleteActiveConfiguration,

    /// Error loading properties from a configuration
    #[error("Unable to load properties")]
    LoadingProperties(#[from] serde_ini::de::Error),

    /// The operation would overwrite an existing configuration
    #[error("A configuration named '{0}' already exists. Use --force to overwrite it")]
    ExistingConfiguration(String),

    /// The configuration name is invalid
    #[error("'{0}' is invalid. Configuration names must only contain ASCII letters and numbers")]
    InvalidName(String),

    /// General I/O error
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    /// Not configurations were found in the configuration store
    #[error("Unable to find any gcloud configurations in {0}")]
    NoConfigurationsFound(PathBuf),

    /// Error saving properties to a configuration
    #[error("Unable to save properties")]
    SavingProperties(#[from] serde_ini::ser::Error),

    /// A configuration with the given name wasn't found
    #[error("Unable to find configuration '{0}'")]
    UnknownConfiguration(String),
}
