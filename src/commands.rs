use crate::configuration::ConfigurationStore;
use anyhow::Result;

/// List the available configurations with an indicator of the active one
pub fn list() -> Result<()> {
    let store = ConfigurationStore::with_default_location()?;

    for config in store.configurations() {
        let prefix = if store.is_active(config) { "* " } else { "  " };

        println!("{}{}", prefix, config.name());
    }

    Ok(())
}

/// Activate the given configuration by name
pub fn activate(name: &str) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    store.activate(name)?;

    println!("Successfully activated '{}'", name);

    Ok(())
}

/// Create a new configuration
pub fn create(name: &str, project: &str, account: &str, zone: &str, region: Option<&str>, force: bool, activate: bool) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    store.create(name, project, account, zone, region, force)?;
    println!("Successfully created configuration '{}'", name);

    if activate {
        store.activate(name)?;
        println!("Configuration '{}' is now active", name);
    }

    Ok(())
}

/// Show the current activated configuration
pub fn current() -> Result<()> {
    let store = ConfigurationStore::with_default_location()?;
    println!("{}", store.active());
    Ok(())
}

/// Describe all the properties in the given configuration
pub fn describe(name: &str) -> Result<()> {
    let store = ConfigurationStore::with_default_location()?;
    let properties = store.describe(name)?;

    for property in properties {
        println!("{} = {}", property.key, property.value);
    }

    Ok(())
}

/// Rename a configuration
pub fn rename(old_name: &str, new_name: &str, force: bool) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    store.rename(old_name, new_name, force)?;

    println!("Successfully renamed configuration '{}' to '{}'", old_name, new_name);

    if let Some(configuration) = store.find_by_name(new_name) {
        if store.is_active(configuration) {
            println!("Configuration '{}' is now active", new_name);
        }
    }

    Ok(())
}
