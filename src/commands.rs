use crate::configuration::ConfigurationStore;
use anyhow::Result;

/// List the available configurations with an indicator of the active one
pub fn list() -> Result<()> {
    let store = ConfigurationStore::new()?;

    for config in store.configurations() {
        let prefix = if store.is_active(config) { "* " } else { "  " };

        println!("{}{}", prefix, config.name());
    }

    Ok(())
}

/// Activate the given configuration by name
pub fn activate(name: &str) -> Result<()> {
    let mut store = ConfigurationStore::new()?;
    store.activate(name)?;

    println!("Successfully activated '{}'", name);

    Ok(())
}

/// Show the current activated configuration
pub fn current() -> Result<()> {
    let store = ConfigurationStore::new()?;
    println!("{}", store.active());
    Ok(())
}

/// Rename a configuration
pub fn rename(old_name: &str, new_name: &str) -> Result<()> {
    let mut store = ConfigurationStore::new()?;
    store.rename(old_name, new_name)?;

    println!(
        "Successfully renamed configuration '{}' to '{}'",
        old_name, new_name
    );

    if let Some(configuration) = store.find_by_name(new_name) {
        if store.is_active(configuration) {
            println!("Configuration '{}' is now active", new_name);
        }
    }

    Ok(())
}
