mod configuration;
mod commands;
mod error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    commands::list()?;
    Ok(())
}
