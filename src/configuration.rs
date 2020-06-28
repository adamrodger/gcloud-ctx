use crate::error::Error;
use anyhow::{anyhow, bail, Context, Result};
use std::{convert::TryFrom, path::PathBuf};

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

impl TryFrom<PathBuf> for Configuration {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let name = value
            .file_name()
            .ok_or_else(|| Error::UnableToReadConfiguration(value.clone()))
            .context("Parsing configuration names")?
            .to_str()
            .ok_or_else(|| Error::UnableToReadConfiguration(value.clone()))
            .context("Converting configuration names")?;

        let name = name.trim_start_matches("config_").to_owned();

        Ok(Configuration { name, path: value })
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
    pub fn new() -> Result<Self> {
        let location = dirs::config_dir().ok_or(Error::ConfigurationDirectoryNotFound)?;
        let location = location.join("gcloud");

        if !(location.exists() && location.is_dir()) {
            bail!(Error::ConfigurationStoreNotFound(location));
        }

        let configurations_glob = location.join("configurations").join("config_*");
        let configurations_glob = configurations_glob
            .to_str()
            .ok_or_else(|| anyhow!("Invalid search pattern: {:?}", configurations_glob))?;

        let configurations = glob::glob(configurations_glob)
            .map_err(anyhow::Error::new)
            .context("Searching for configurations")?
            .map(|path| {
                let path = path.context("Parsing configuration files")?;
                Configuration::try_from(path)
            })
            .collect::<Result<Vec<Configuration>, anyhow::Error>>()?;

        if configurations.is_empty() {
            bail!(Error::NoConfigurationsFound(location));
        }

        let active = location.join("active_config");
        let active = std::fs::read_to_string(active)
            .with_context(|| format!("Determining active configuration in {:?}", location))?;

        Ok(ConfigurationStore {
            location,
            configurations,
            active,
        })
    }

    /// Get the name of the currently active configuration
    pub fn active(&self) -> &str {
        &self.active
    }

    /// Get the collection of currently available configurations
    pub fn configurations(&self) -> &[Configuration] {
        &self.configurations
    }

    /// Check if the given configuration is active
    pub fn is_active(&self, configuration: &Configuration) -> bool {
        configuration.name == self.active
    }

    /// Activate a configuration by name
    pub fn activate(&mut self, name: &str) -> Result<()> {
        let configuration = self
            .configurations
            .iter()
            .find(|&c| c.name == name)
            .ok_or_else(|| Error::UnknownConfiguration(name.to_owned()))?;

        let path = self.location.join("active_config");
        std::fs::write(path, &configuration.name).context("Setting active configuration")?;

        self.active = configuration.name.to_owned();

        Ok(())
    }
}
