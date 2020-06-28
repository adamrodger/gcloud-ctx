mod arguments;
mod commands;
mod configuration;
mod error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    arguments::run()?;
    Ok(())
}
