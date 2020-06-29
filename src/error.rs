use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to locate user configuration directory")]
    ConfigurationDirectoryNotFound,

    #[error("Unable to find the gcloud configuration directory at {0}\n\nIs gcloud installed?")]
    ConfigurationStoreNotFound(PathBuf),

    #[error("A configuration named '{0}' already exists")]
    ExistingConfiguration(String),

    #[error("Unable to find any gcloud configurations in {0}")]
    NoConfigurationsFound(PathBuf),

    #[error("Unable to load gcloud configuration at {0}")]
    UnableToReadConfiguration(PathBuf),

    #[error("Unable to find configuration '{0}'")]
    UnknownConfiguration(String),
}
