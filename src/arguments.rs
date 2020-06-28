use crate::{commands, error::Error};
use clap::Clap;

/// Run the application using the command line arguments
pub fn run() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    if let Some(name) = opts.context {
        // shortcut for activate
        commands::activate(&name)?;
        return Ok(());
    }
    else if let Some(subcmd) = opts.subcmd {
        match subcmd {
            SubCommand::Activate(args) => commands::activate(&args.name)?,
            SubCommand::List(_) => commands::list()?,
        }
    }

    Ok(())
}

/// gcloud configuration manager
#[derive(Clap)]
pub struct Opts {
    /// Switch to this context (shorthand for activate)
    context: Option<String>,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    Activate(Activate),
    List(List),
}

#[derive(Clap)]
/// Activate a configuration
struct Activate {
    /// Name of the configuration to activate
    name: String,
}

/// List all available configurations
#[derive(Clap)]
struct List {}
