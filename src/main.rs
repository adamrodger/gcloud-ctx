use anyhow::Result;

mod arguments;
mod commands;
mod configuration;
mod error;
mod properties;

fn main() -> Result<()> {
    arguments::run()?;
    Ok(())
}
