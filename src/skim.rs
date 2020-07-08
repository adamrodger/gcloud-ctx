use crate::configuration::ConfigurationStore;
use crate::error::Error;
use anyhow::{Context, Result};
use skim::prelude::*;
use std::io::Cursor;

/// Use built-in fuzzy finder to select configuration to activate
pub fn fuzzy_find_config() -> Result<String> {
    // load list of configurations into string seperated by newlines
    let store = ConfigurationStore::with_default_location().context("Opening configuration store")?;
    let configs_as_str = store
        .configurations()
        .iter()
        .map(|x| x.name())
        .collect::<Vec<&str>>()
        .join("\n");

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .or(Err(Error::SkimBuildError))?;

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(configs_as_str));

    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    Ok(selected_item
        .first()
        .ok_or(Error::SkimErrorNoConfiguration)?
        .output()
        .to_string())
}
