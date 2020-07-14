use anyhow::{Context, Result};
use gcloud_ctx::{ConfigurationStore, PropertiesBuilder};

/// List the available configurations with an indicator of the active one
pub fn list() -> Result<()> {
    let store = ConfigurationStore::with_default_location().context("Opening configuration store")?;

    for config in store.configurations() {
        let prefix = if store.is_active(config) { "* " } else { "  " };

        println!("{}{}", prefix, config.name());
    }

    Ok(())
}

/// Activate the given configuration by name
pub fn activate(name: &str) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location().context("Opening configuration store")?;
    store.activate(name)?;

    println!("Successfully activated '{}'", name);

    Ok(())
}

/// Copy an existing configuration, optionally overriding properties
pub fn copy(src_name: &str, dest_name: &str, force: bool, activate: bool) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    store.copy(src_name, dest_name, force)?;

    println!("Successfully copied configuration '{}' to '{}'", src_name, dest_name);

    if activate {
        store.activate(dest_name)?;
        println!("Configuration '{}' is now active", dest_name);
    }

    Ok(())
}

/// Create a new configuration
pub fn create(
    name: &str,
    project: &str,
    account: &str,
    zone: &str,
    region: Option<&str>,
    force: bool,
    activate: bool,
) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location().context("Opening configuration store")?;
    let mut builder = PropertiesBuilder::default();

    builder.with_project(project).with_account(account).with_zone(zone);

    if let Some(region) = region {
        builder.with_region(region);
    }

    let properties = builder.build();

    store.create(name, &properties, force)?;

    println!("Successfully created configuration '{}'", name);

    if activate {
        store.activate(name)?;
        println!("Configuration '{}' is now active", name);
    }

    Ok(())
}

/// Show the current activated configuration
pub fn current() -> Result<()> {
    let store = ConfigurationStore::with_default_location().context("Opening configuration store")?;
    println!("{}", store.active());
    Ok(())
}

/// Delete a configuration
pub fn delete(name: &str) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location().context("Opening configuration store")?;
    store.delete(name)?;

    println!("Successfully deleted configuration '{}'", name);
    Ok(())
}

/// Describe all the properties in the given configuration
pub fn describe(name: &str) -> Result<()> {
    let store = ConfigurationStore::with_default_location().context("Opening configuration store")?;
    let properties = store.describe(name)?;

    properties
        .to_writer(std::io::stdout())
        .context("Serialising properties for display")?;

    Ok(())
}

/// Rename a configuration
pub fn rename(old_name: &str, new_name: &str, force: bool) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location().context("Opening configuration store")?;
    store.rename(old_name, new_name, force)?;

    println!("Successfully renamed configuration '{}' to '{}'", old_name, new_name);

    if let Some(configuration) = store.find_by_name(new_name) {
        if store.is_active(configuration) {
            println!("Configuration '{}' is now active", new_name);
        }
    }

    Ok(())
}
