use anyhow::Result;

mod arguments;
mod commands;
mod configuration;
mod error;

fn main() -> Result<()> {
    arguments::run()?;
    Ok(())
}
