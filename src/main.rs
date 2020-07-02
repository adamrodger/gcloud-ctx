use anyhow::Result;

mod arguments;
mod commands;
mod configuration;
mod error;
mod fzf;

fn main() -> Result<()> {
    arguments::run()?;
    Ok(())
}
