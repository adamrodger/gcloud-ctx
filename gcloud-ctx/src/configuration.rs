use crate::{properties::Properties, Error, Result};
use fs::File;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, fs, io::BufReader, path::PathBuf};

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

/// Action to perform when a naming conflict occurs
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConflictAction {
    /// Abort the operation
    Abort,

    /// Overwrite the existing configuration
    Overwrite,
}

impl From<bool> for ConflictAction {
    fn from(value: bool) -> Self {
        if value {
            Self::Overwrite
        } else {
            Self::Abort
        }
    }
}

#[derive(Debug)]
/// Represents the store of gcloud configurations
pub struct ConfigurationStore {
    /// Location of the configuration store on disk
    location: PathBuf,

    /// Path to the configurations sub-folder
    configurations_path: PathBuf,

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
            return Err(Error::ConfigurationStoreNotFound(gcloud_path));
        }

        let configurations_path = gcloud_path.join("configurations");

        if !configurations_path.is_dir() {
            return Err(Error::ConfigurationStoreNotFound(configurations_path));
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
            return Err(Error::NoConfigurationsFound(configurations_path));
        }

        let active = gcloud_path.join("active_config");
        let active = fs::read_to_string(active)?;

        Ok(Self {
            location: gcloud_path,
            configurations_path,
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
        std::fs::write(path, &configuration.name)?;

        self.active = configuration.name.to_owned();

        Ok(())
    }

    /// Copy an existing configuration, preserving all properties
    pub fn copy(&mut self, src_name: &str, dest_name: &str, conflict: ConflictAction) -> Result<()> {
        let src = self
            .configurations
            .get(src_name)
            .ok_or_else(|| Error::UnknownConfiguration(src_name.to_owned()))?;

        if !Configuration::is_valid_name(dest_name) {
            return Err(Error::InvalidName(dest_name.to_owned()));
        }

        if conflict == ConflictAction::Abort && self.configurations.contains_key(dest_name) {
            return Err(Error::ExistingConfiguration(dest_name.to_owned()));
        }

        // just copy the file on disk so that any properties which aren't directly supported are maintained
        let filename = self.configurations_path.join(format!("config_{dest_name}"));
        fs::copy(&src.path, &filename)?;

        let dest = Configuration {
            name: dest_name.to_owned(),
            path: filename,
        };

        self.configurations.insert(dest_name.to_owned(), dest);

        Ok(())
    }

    /// Create a new configuration
    pub fn create(&mut self, name: &str, properties: &Properties, conflict: ConflictAction) -> Result<()> {
        if !Configuration::is_valid_name(name) {
            return Err(Error::InvalidName(name.to_owned()));
        }

        if conflict == ConflictAction::Abort && self.configurations.contains_key(name) {
            return Err(Error::ExistingConfiguration(name.to_owned()));
        }

        let filename = self.configurations_path.join(format!("config_{name}"));
        let file = File::create(&filename)?;
        properties.to_writer(file)?;

        self.configurations.insert(
            name.to_owned(),
            Configuration {
                name: name.to_owned(),
                path: filename,
            },
        );

        Ok(())
    }

    /// Delete a configuration
    pub fn delete(&mut self, name: &str) -> Result<()> {
        let configuration = self
            .find_by_name(name)
            .ok_or_else(|| Error::UnknownConfiguration(name.to_owned()))?;

        if self.is_active(configuration) {
            return Err(Error::DeleteActiveConfiguration);
        }

        let path = &configuration.path;
        fs::remove_file(path)?;

        self.configurations.remove(name);

        Ok(())
    }

    /// Describe the properties in the given configuration
    pub fn describe(&self, name: &str) -> Result<Properties> {
        let configuration = self
            .find_by_name(name)
            .ok_or_else(|| Error::UnknownConfiguration(name.to_owned()))?;

        let path = &configuration.path;
        let handle = File::open(path)?;
        let reader = BufReader::new(handle);

        let properties = Properties::from_reader(reader)?;

        Ok(properties)
    }

    /// Rename a configuration
    pub fn rename(&mut self, old_name: &str, new_name: &str, conflict: ConflictAction) -> Result<()> {
        let src = self
            .configurations
            .get(old_name)
            .ok_or_else(|| Error::UnknownConfiguration(old_name.to_owned()))?;

        let active = self.is_active(src);

        if !Configuration::is_valid_name(new_name) {
            return Err(Error::InvalidName(new_name.to_owned()));
        }

        if conflict == ConflictAction::Abort && self.configurations.contains_key(new_name) {
            return Err(Error::ExistingConfiguration(new_name.to_owned()));
        }

        let new_value = Configuration {
            name: new_name.to_owned(),
            path: src.path.with_file_name(format!("config_{new_name}")),
        };

        std::fs::rename(&src.path, &new_value.path)?;

        self.configurations.remove(old_name);
        self.configurations.insert(new_name.to_owned(), new_value);

        // check if the active configuration is the one being renamed
        if active {
            self.activate(new_name)?;
        }

        Ok(())
    }

    /// Find a configuration by name
    pub fn find_by_name(&self, name: &str) -> Option<&Configuration> {
        self.configurations.get(name)
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
