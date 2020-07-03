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
            SubCommand::Create {
                name,
                project,
                account,
                zone,
                region,
                activate,
                force,
            } => {
                commands::create(&name, &project, &account, &zone, region.as_deref(), force, activate)?;
            }
            SubCommand::Current => commands::current()?,
            SubCommand::Describe { name } => commands::describe(&name)?,
            SubCommand::List => commands::list()?,
            SubCommand::Rename {
                old_name,
                new_name,
                force,
            } => commands::rename(&old_name, &new_name, force)?,
        }
    } else {
        commands::current()?;
    }

    Ok(())
}

/// gcloud configuration manager
#[derive(Clap)]
pub struct Opts {
    /// Switch to this context (shorthand for activate, ignores subsequent arguments)
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

    /// Create a new configuration
    Create {
        // Name of the new configuration
        name: String,

        /// Setting for core/project
        #[clap(short, long)]
        project: String,

        /// Setting for core/account
        #[clap(short, long)]
        account: String,

        /// Setting for compute/zone
        #[clap(short, long)]
        zone: String,

        /// Setting for compute/region
        #[clap(short, long)]
        region: Option<String>,

        /// Activate the new configuration immediately
        #[clap(long)]
        activate: bool,

        /// Force a create to overwrite an existing configuration
        #[clap(short, long)]
        force: bool,
    },

    /// Describe all the properties in a configuration
    Describe {
        /// Name of the configuration
        name: String,
    },

    /// Show the current configuration
    Current,

    /// List all available configurations
    List,

    /// Rename a configuration
    Rename {
        /// Name of an existing configuration
        old_name: String,

        /// New name
        new_name: String,

        /// Force a rename to overwrite an existing configuration
        #[clap(short, long)]
        force: bool,
    },
}
