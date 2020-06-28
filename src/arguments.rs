use crate::commands;
use anyhow::Result;
use clap::Clap;

/// Run the application using the command line arguments
pub fn run() -> Result<()> {
    let opts: Opts = Opts::parse();

    if let Some(name) = opts.context {
        // shortcut for activate
        commands::activate(&name)?;
        return Ok(());
    } else if let Some(subcmd) = opts.subcmd {
        match subcmd {
            SubCommand::Activate { name } => commands::activate(&name)?,
            SubCommand::Current => commands::current()?,
            SubCommand::List => commands::list()?,
        }
    } else {
        commands::current()?;
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
    /// Activate a configuration by name
    Activate {
        /// Name of the configuration to activate
        name: String,
    },

    /// Show the current configuration
    Current,

    /// List all available configurations
    List,
}
