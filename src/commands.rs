use crate::{configuration::ConfigurationStore, error::Error};

/// List the available configurations with an indicator of the active one
pub fn list() -> Result<(), Error> {
    let store = ConfigurationStore::new()?;

    for config in store.configurations() {
        let prefix = if store.is_active(config) { "* " } else { "  " };

        println!("{}{}", prefix, config.name());
    }

    Ok(())
}

/// Activate the given configuration by name
pub fn activate(name: &str) -> Result<(), Error> {
    let mut store = ConfigurationStore::new()?;
    store.activate(name)?;

    println!("Successfully activated '{}'", name);

    Ok(())
}
