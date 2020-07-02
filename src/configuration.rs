use crate::error::Error;
use anyhow::{bail, Context, Result};
use ini::Ini;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, fs, path::PathBuf};

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new("^[a-z][-a-z0-9]*$").unwrap();
}

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

    /// Is the given name a valid configuration name?
    ///
    /// Names must start with a lowercase ASCII character
    /// then zero or more ASCII alphanumerics and hyphens
    pub fn is_valid_name(name: &str) -> bool {
        NAME_REGEX.is_match(name)
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

/// Represents a single configuration property
pub struct ConfigurationProperty {
    pub key: String,
    pub value: String,
}

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
    /// Opens the configuration store using the OS-specific defaults
    ///
    /// If the `CLOUDSDK_CONFIG` environment variable is set then this will be used, otherwise an
    /// OS-specific default location will be used, as defined by the [dirs] crate, e.g.:
    ///
    /// - Windows: `%APPDATA%\gcloud`
    /// - Linux: `~/.config/gcloud`
    /// - Mac: `~/.config/gcloud` - note that this does not follow the Apple Developer Guidelines
    ///
    /// [dirs]: https://crates.io/crates/dirs
    pub fn with_default_location() -> Result<Self> {
        let gcloud_path: PathBuf = if let Ok(value) = std::env::var("CLOUDSDK_CONFIG") {
            value.into()
        } else {
            let gcloud_path = if cfg!(target_os = "macos") {
                dirs::home_dir()
                    .ok_or(Error::ConfigurationDirectoryNotFound)?
                    .join(".config")
            } else {
                dirs::config_dir().ok_or(Error::ConfigurationDirectoryNotFound)?
            };

            gcloud_path.join("gcloud")
        };

        Self::with_location(gcloud_path)
    }

    /// Opens a configuration store at the given path
    pub fn with_location(gcloud_path: PathBuf) -> Result<Self> {
        if !gcloud_path.is_dir() {
            bail!(Error::ConfigurationStoreNotFound(gcloud_path));
        }

        let configurations_path = gcloud_path.join("configurations");

        if !configurations_path.is_dir() {
            bail!(Error::ConfigurationStoreNotFound(configurations_path));
        }

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
            let name = name.trim_start_matches("config_");

            if !Configuration::is_valid_name(name) {
                continue;
            }

            configurations.insert(
                name.to_owned(),
                Configuration {
                    name: name.to_owned(),
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

        if !Configuration::is_valid_name(new_name) {
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

    /// Describe all the properties in the given configuration
    pub fn describe(&self, name: &str) -> Result<Vec<ConfigurationProperty>> {
        let configuration = self
            .find_by_name(name)
            .ok_or(Error::UnknownConfiguration(name.to_owned()))?;
        let ini_file =
            Ini::load_from_file(&configuration.path).context("Describing configuration")?;
        let mut properties = Vec::new();

        for section in ini_file.iter() {
            if let Some(header) = section.0 {
                for property in section.1.iter() {
                    properties.push(ConfigurationProperty {
                        key: format!("{}/{}", header, property.0),
                        value: property.1.to_owned(),
                    });
                }
            }
        }

        Ok(properties)
    }

    /// Find a configuration by name
    pub fn find_by_name(&self, name: &str) -> Option<&Configuration> {
        self.configurations.get(&name.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_valid_name_with_valid_name() {
        assert!(Configuration::is_valid_name("foo"));
        assert!(Configuration::is_valid_name("f"));
        assert!(Configuration::is_valid_name("f123"));
        assert!(Configuration::is_valid_name("foo-bar"));
        assert!(Configuration::is_valid_name("foo-123"));
        assert!(Configuration::is_valid_name("foo-a1b2c3"));
    }

    #[test]
    pub fn test_is_valid_name_with_invalid_name() {
        // too short
        assert!(!Configuration::is_valid_name(""));

        // doesn't start with lowercase ASCII
        assert!(!Configuration::is_valid_name("F"));
        assert!(!Configuration::is_valid_name("1"));
        assert!(!Configuration::is_valid_name("-"));

        // doesn't contain only alphanumerics and ASCII
        assert!(!Configuration::is_valid_name("foo_bar"));
        assert!(!Configuration::is_valid_name("foo.bar"));
        assert!(!Configuration::is_valid_name("foo|bar"));
        assert!(!Configuration::is_valid_name("foo$bar"));
        assert!(!Configuration::is_valid_name("foo#bar"));
        assert!(!Configuration::is_valid_name("foo@bar"));
        assert!(!Configuration::is_valid_name("foo;bar"));
        assert!(!Configuration::is_valid_name("foo?bar"));

        // doesn't contain only lowercase
        assert!(!Configuration::is_valid_name("camelCase"));
    }
}
