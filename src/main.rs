use configuration::ConfigurationStore;

mod configuration;
mod error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = ConfigurationStore::new()?;
    println!("{:?}", store);
    Ok(())
}
