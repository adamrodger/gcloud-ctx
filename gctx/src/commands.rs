use anyhow::{bail, Context, Result};
use colored::*;
use dialoguer::{Confirm, Input};
use gcloud_ctx::{ConfigurationStore, ConflictAction, PropertiesBuilder};

/// Used to control whether to activate a configuration after creation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PostCreation {
    /// Ignore the new configuration
    Noop,

    /// Activate the new configuration
    Activate,
}

impl From<bool> for PostCreation {
    fn from(value: bool) -> Self {
        if value {
            Self::Activate
        } else {
            Self::Noop
        }
    }
}

/// List the available configurations with an indicator of the active one
pub fn list() -> Result<()> {
    let store = ConfigurationStore::with_default_location()?;

    for config in store.configurations() {
        if store.is_active(config) {
            println!("{} {}", "*".blue(), config.name().blue());
        } else {
            println!("  {}", config.name());
        }
    }

    Ok(())
}

/// Activate the given configuration by name
pub fn activate(name: &str) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    store.activate(name)?;

    println!("Successfully activated '{}'", name.blue());

    Ok(())
}

/// Copy an existing configuration
pub fn copy(src_name: &str, dest_name: &str, conflict: ConflictAction, activate: PostCreation) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    store.copy(src_name, dest_name, conflict)?;

    println!(
        "Successfully copied configuration '{}' to '{}'",
        src_name.yellow(),
        dest_name.blue()
    );

    if activate == PostCreation::Activate {
        store.activate(dest_name)?;
        println!("Configuration '{}' is now active", dest_name.blue());
    }

    Ok(())
}

/// Create a new configuration interactively
pub fn create_interactive() -> Result<()> {
    let store = ConfigurationStore::with_default_location()?;

    let name = Input::<String>::new()
        .with_prompt("Name".blue().to_string())
        .interact()?;

    if store.find_by_name(&name).is_some() {
        let prompt = "A configuration with the same name already exists. Overwrite?"
            .yellow()
            .to_string();
        let confirm = Confirm::new().with_prompt(prompt).default(false).interact()?;

        if !confirm {
            bail!("Operation cancelled".yellow());
        }
    }

    let project = Input::<String>::new()
        .with_prompt("Project".blue().to_string())
        .interact()?;

    let account = Input::<String>::new()
        .with_prompt("Account".blue().to_string())
        .interact()?;

    let zone = Input::<String>::new()
        .with_prompt("Zone".blue().to_string())
        .interact()?;

    let region = Input::<String>::new()
        .with_prompt("Region (optional)".blue().to_string())
        .allow_empty(true)
        .interact()?;
    let region = if region.is_empty() { None } else { Some(region) };

    let activate = Confirm::new()
        .with_prompt("Activate".blue().to_string())
        .default(false)
        .interact()?;

    create(
        &name,
        &project,
        &account,
        &zone,
        region.as_deref(),
        ConflictAction::Overwrite,
        activate.into(),
    )?;

    Ok(())
}

/// Create a new configuration
pub fn create(
    name: &str,
    project: &str,
    account: &str,
    zone: &str,
    region: Option<&str>,
    conflict: ConflictAction,
    activate: PostCreation,
) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    let mut builder = PropertiesBuilder::default();

    builder.project(project).account(account).zone(zone);

    if let Some(region) = region {
        builder.region(region);
    }

    let properties = builder.build();

    store.create(name, &properties, conflict)?;

    println!("Successfully created configuration '{}'", name.blue());

    if activate == PostCreation::Activate {
        store.activate(name)?;
        println!("Configuration '{}' is now active", name.blue());
    }

    Ok(())
}

/// Show the current activated configuration
pub fn current() -> Result<()> {
    let store = ConfigurationStore::with_default_location()?;
    println!("{}", store.active().blue());
    Ok(())
}

/// Delete a configuration
pub fn delete(name: &str) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    store.delete(name)?;

    println!("Successfully deleted configuration '{}'", name.yellow());
    Ok(())
}

/// Describe all the properties in the given configuration
pub fn describe(name: Option<&str>) -> Result<()> {
    let store = ConfigurationStore::with_default_location()?;
    let name = name.unwrap_or_else(|| store.active());
    let properties = store.describe(name)?;

    properties
        .to_writer(std::io::stdout())
        .context("Serialising properties for display")?;

    Ok(())
}

/// Rename a configuration
pub fn rename(old_name: &str, new_name: &str, conflict: ConflictAction) -> Result<()> {
    let mut store = ConfigurationStore::with_default_location()?;
    store.rename(old_name, new_name, conflict)?;

    println!(
        "Successfully renamed configuration '{}' to '{}'",
        old_name.yellow(),
        new_name.blue()
    );

    if let Some(configuration) = store.find_by_name(new_name) {
        if store.is_active(configuration) {
            println!("Configuration '{}' is now active", new_name.blue());
        }
    }

    Ok(())
}
