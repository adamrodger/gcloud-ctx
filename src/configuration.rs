use crate::error::Error;
use std::path::PathBuf;

#[derive(Debug, Clone)]
/// Represents a gcloud named configuration
pub struct Configuration {
    /// Name of the configuration
    name: String,

    /// Path to the configuration file
    path: PathBuf,
}

impl Configuration {
    /// Name of the configuration
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
/// Represents the store of gcloud configurations
pub struct ConfigurationStore {
    /// Location of the configuration store on disk
    location: PathBuf,

    /// Available configurations
    configurations: Vec<Configuration>,

    /// Name of the active configuration
    active: String,
}

impl ConfigurationStore {
    /// Opens the configuration store using the OS-specific default location
    pub fn new() -> Result<Self, Error> {
        let location = dirs::config_dir().ok_or(Error::ConfigurationStoreNotFound)?;
        let location = location.join("gcloud");

        if !(location.exists() && location.is_dir()) {
            return Err(Error::ConfigurationStoreNotFound);
        }

        let configurations_glob = location.join("configurations").join("config_*");
        let configurations_glob = configurations_glob
            .to_str()
            .ok_or(Error::ConfigurationStoreNotFound)?;

        let configurations = glob::glob(configurations_glob)
            .map_err(|_| Error::UnableToReadConfigurations)?
            .map(|path| {
                let path = path.map_err(|_| Error::UnableToReadConfigurations)?;
                let name = path
                    .file_name()
                    .ok_or(Error::UnableToReadConfigurations)?
                    .to_str()
                    .ok_or(Error::UnableToReadConfigurations)?
                    .trim_start_matches("config_")
                    .to_owned();

                Ok(Configuration { name, path })
            })
            .collect::<Result<Vec<Configuration>, Error>>()?;

        let active = location.join("active_config");

        if !(active.exists() && active.is_file()) {
            return Err(Error::ConfigurationStoreNotFound);
        }

        let active = std::fs::read_to_string(active)?;

        Ok(ConfigurationStore {
            location,
            configurations,
            active,
        })
    }

    /// Get the collection of currently available configurations
    pub fn configurations(&self) -> &[Configuration] {
        &self.configurations
    }

    /// Check if the given configuration is active
    pub fn is_active(&self, name: &str) -> bool {
        name == self.active
    }
}
