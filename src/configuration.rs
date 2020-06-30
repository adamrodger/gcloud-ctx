use crate::error::Error;
use anyhow::{bail, Context, Result};
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, fs, path::PathBuf};

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

impl Ord for Configuration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Configuration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Configuration {}

#[derive(Debug)]
/// Represents the store of gcloud configurations
pub struct ConfigurationStore {
    /// Location of the configuration store on disk
    location: PathBuf,

    /// Available configurations
    configurations: HashMap<String, Configuration>,

    /// Name of the active configuration
    active: String,
}

impl ConfigurationStore {
    /// Opens the configuration store using the OS-specific default location
    pub fn new() -> Result<Self> {
        let gcloud_path = dirs::config_dir().ok_or(Error::ConfigurationDirectoryNotFound)?;
        let gcloud_path = gcloud_path.join("gcloud");

        if !gcloud_path.is_dir() {
            bail!(Error::ConfigurationStoreNotFound(gcloud_path));
        }

        let configurations_path = gcloud_path.join("configurations");

        if !configurations_path.is_dir() {
            bail!(Error::ConfigurationStoreNotFound(configurations_path));
        }

        let file_name_regex = Regex::new(r"^config_[a-zA-Z0-9]+$").unwrap();
        let mut configurations: HashMap<String, Configuration> = HashMap::new();

        for file in fs::read_dir(&configurations_path)? {
            if file.is_err() {
                // ignore files we're unable to read - e.g. permissions errors
                continue;
            }

            let file = file.unwrap();
            let name = file.file_name();
            let name = match name.to_str() {
                Some(name) => name,
                None => continue, // ignore files that aren't valid utf8
            };

            if !file_name_regex.is_match(name) {
                continue;
            }

            let name = name.trim_start_matches("config_").to_owned();

            configurations.insert(
                name.clone(),
                Configuration {
                    name,
                    path: file.path(),
                },
            );
        }

        if configurations.is_empty() {
            bail!(Error::NoConfigurationsFound(configurations_path));
        }

        let active = gcloud_path.join("active_config");
        let active = fs::read_to_string(active)
            .with_context(|| format!("Determining active configuration in {:?}", gcloud_path))?;

        Ok(ConfigurationStore {
            location: gcloud_path,
            configurations,
            active,
        })
    }

    /// Get the name of the currently active configuration
    pub fn active(&self) -> &str {
        &self.active
    }

    /// Get the collection of currently available configurations
    pub fn configurations(&self) -> Vec<&Configuration> {
        let mut value: Vec<&Configuration> = self.configurations.values().collect();
        value.sort();
        value
    }

    /// Check if the given configuration is active
    pub fn is_active(&self, configuration: &Configuration) -> bool {
        configuration.name == self.active
    }

    /// Activate a configuration by name
    pub fn activate(&mut self, name: &str) -> Result<()> {
        let configuration = self
            .find_by_name(name)
            .ok_or_else(|| Error::UnknownConfiguration(name.to_owned()))?;

        let path = self.location.join("active_config");
        std::fs::write(path, &configuration.name).context("Setting active configuration")?;

        self.active = configuration.name.to_owned();

        Ok(())
    }

    /// Rename a configuration
    pub fn rename(&mut self, old_name: &str, new_name: &str) -> Result<&Configuration> {
        if !self.configurations.contains_key(old_name) {
            bail!(Error::UnknownConfiguration(old_name.to_owned()));
        }

        if self.configurations.contains_key(new_name) {
            bail!(Error::ExistingConfiguration(new_name.to_owned()));
        }

        if !Regex::new(r"^[a-zA-Z0-9]+$").unwrap().is_match(new_name) {
            bail!(Error::InvalidName(new_name.to_owned()));
        }

        let (active, new_value) = {
            let existing = self.configurations.get(old_name).unwrap();

            let new_value = Configuration {
                name: new_name.to_owned(),
                path: existing.path.with_file_name(format!("config_{}", new_name)),
            };

            std::fs::rename(&existing.path, &new_value.path)?;

            (self.is_active(&existing), new_value)
        };

        self.configurations.remove(old_name);
        self.configurations.insert(new_name.to_owned(), new_value);

        // check if the active configuration is the one being renamed
        if active {
            self.activate(new_name)?;
        }

        let new_value = self.configurations.get(new_name).unwrap();
        Ok(&new_value)
    }

    /// Find a configuration by name
    pub fn find_by_name(&self, name: &str) -> Option<&Configuration> {
        self.configurations.get(&name.to_owned())
    }
}
