use anyhow::{bail, Result};
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use gcloud_ctx::ConfigurationStore;

/// Find a configuration to activate using by giving the user an interactive prompt
pub fn fuzzy_find_config() -> Result<String> {
    let store = ConfigurationStore::with_default_location()?;

    let items = store.configurations().iter().map(|&c| c.name()).collect::<Vec<_>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => Ok(items[index].to_owned()),
        None => bail!("No configuration selected"),
    }
}
