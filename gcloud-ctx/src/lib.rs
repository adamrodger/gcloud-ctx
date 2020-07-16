#![warn(missing_docs)]

//! A Rust implementation of [`gcloud config configurations`](https://cloud.google.com/sdk/gcloud/reference/config/configurations)
//! for managing different `gcloud` configurations for Google Cloud Platform. This is the library containing the core logic
//! which is used to build the associated [`gctx`](https://github.com/adamrodger/gctx) command line utility.
//!
//! **Note**: `gcloud-ctx` is independent and not affiliated with Google in any way.
//!
//! ## Usage
//!
//! ```rust
//! # // use a temp dir so that running doc tests doesn't change your real configurations
//! # use std::fs::File;
//! # let tmp = tempfile::tempdir().unwrap();
//! # File::create(tmp.path().join("active_config")).unwrap();
//! # let configs = tmp.path().join("configurations");
//! # std::fs::create_dir(&configs).unwrap();
//! # File::create(configs.join("config_foo")).unwrap();
//! # std::env::set_var("CLOUDSDK_CONFIG", tmp.path());
//! #
//! use gcloud_ctx::ConfigurationStore;
//!
//! let mut store = ConfigurationStore::with_default_location()?;
//!
//! // create a new configuration, optionally with a force overwrite
//! use gcloud_ctx::PropertiesBuilder;
//! let properties = PropertiesBuilder::default()
//!     .project("my-project")
//!     .account("a.user@example.org")
//!     .zone("europe-west1-d")
//!     .region("europe-west1")
//!     .build();
//!
//! store.create("foo", &properties, true)?;
//!
//! // list configurations
//! for config in store.configurations() {
//!     println!("{}", config.name());
//! }
//!
//! // activate a configuration by name
//! store.activate("foo")?;
//!
//! // get the active configuration
//! println!("{}", store.active());
//!
//! // copy an existing configuration, with force overwrite
//! store.copy("foo", "bar", true)?;
//!
//! // rename an existing configuration, with force overwrite
//! store.rename("bar", "baz", true)?;
//!
//! // delete a configuration
//! store.delete("baz")?;
//!
//! // get properties of a configuration
//! let properties = store.describe("foo")?;
//! properties.to_writer(std::io::stdout())?;
//! #
//! # Ok::<(), gcloud_ctx::Error>(())
//! ```

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
